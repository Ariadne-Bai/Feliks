use crate::{custom_types::*, schedule::*, train::*, human::{TripUnit, HumanState, self}};
use std::{collections::HashMap, marker::PhantomData};
use rand::Rng;

/**
 * Central registry for trains, stations, lines
 * The Owner of all these objects
 * Is this a good design? I've got no idea
 */

pub struct TrainManager<'a> {
    lineTables: HashMap<LineID, LineTimeTable>,
    stations: HashMap<StationID, Station>,
    stationIdMap: HashMap<String, StationID>,
    stationOnLines: HashMap<StationID, Vec<LineID>>,
    trains: HashMap<TrainID, Train>,
    next_line: LineID,
    next_station: StationID,
    next_train: TrainID,
    phantom: PhantomData<&'a u32>,
}

impl<'a> TrainManager<'a> {
    pub fn new() -> Self {
        TrainManager {
            lineTables: HashMap::new(),
            stations: HashMap::new(),
            stationIdMap: HashMap::new(),
            stationOnLines: HashMap::new(),
            trains: HashMap::new(),
            next_line: 0,
            next_station: 0,
            next_train: 0,
            phantom: PhantomData,
        }
    }

    // TODO: train system initializations

    pub fn register_station(
        &mut self,
        name: String,
        corx: Distance,
        cory: Distance,
    ) -> (StationID, String) {
        let qs = format!(
            "CREATE (a:Station:Static {{ stationID: {}, name: \"{}\", corX: {}, corY: {}}})",
            self.next_station, &name, corx, cory
        );

        let st = Station::new(self.next_station, name, corx, cory);
        self.stations.insert(self.next_station, st);
        self.next_station += 1;

        (self.next_station - 1, qs)
    }

    pub fn register_linetable(&mut self, name: String, speed: u32) -> LineID {
        let lt = LineTimeTable::new(self.next_line, name, speed);
        self.lineTables.insert(self.next_line, lt);
        self.next_line += 1;
        self.next_line - 1
    }

    pub fn add_new_traintime_toline(&mut self, lid: LineID, time: Time) {
        match self.lineTables.get_mut(&lid) {
            Some(ltb) => {
                ltb.add_new_traintime(time);
            }
            None => {
                println!("Add New Train Times: there is no line with ID {}!", lid);
            }
        }
    }

    pub fn add_station_toline(&mut self, lid: LineID, stt: StationTimeTable) -> Vec<String> {
        let mut qss = Vec::new();
        match self.lineTables.get_mut(&lid) {
            Some(ltb) => {
                let qs_tt_node = format!(
                    "CREATE (a:StationTable:Static {{ stationID: {}, lineID: {}, stop_time: {}}})",
                    stt.id, lid, &stt.stop_time
                );
                qss.push(qs_tt_node);
                let qs_tt_link = format!(
                    "MATCH (a:Station), (t:StationTable) WHERE a.stationID = {} AND t.stationID = {} AND t.lineID = {} CREATE (a)-[:TABLE]->(t)", 
                    stt.id, stt.id, lid
                );
                qss.push(qs_tt_link);

                if let Some(next_station) = stt.station_next {
                    let qs_ns_link = format!(
                        "MATCH (a:Station), (b:Station) WHERE a.stationID = {} AND b.stationID = {} CREATE (a)-[rel:NEXT_STATION]->(b) SET rel.distance = {}", 
                        stt.id, next_station, stt.distance_next.unwrap()
                    );
                    qss.push(qs_ns_link);
                }

                ltb.add_station(stt);
            }
            None => {
                println!("Add Station: there is no line with ID {}!", lid);
            }
        }
        qss
    }

    // register Train object; SimEngine should be responsible for spawn initial events
    pub fn register_train(&mut self, lid: LineID, sid: StationID, time: Time) -> TrainID {
        let sp = self.lineTables.get(&lid).unwrap().get_speed();
        let tr = Train::new(self.next_train, lid, sp, sid, time);
        self.trains.insert(self.next_train, tr);
        self.next_train += 1;
        self.next_train - 1
    }

    // TODO: build static im-memory graph of the transit network (use petgraph crate!)
    // use a pathfinding algorithm to find the route for passengers
    // there are also interesting infomation about graph algorithm with that crate

    // TODO: look up and update state for some agent
    // agent have their own update() method, but manager will find which object to call

    pub fn train_update(&mut self, event: Event, tid: TrainID) {
        if let Some(the_train) = self.trains.get_mut(&tid) {
            the_train.update_state(event);
        }
    }

    pub fn find_last_station(&self, lid: LineID, sid: StationID) -> Option<StationID> {
        self.lineTables.get(&lid).unwrap().get_last_station(sid)
    }

    pub fn handle_event(&mut self, event: Event, cur_time: Time) -> (Time, Option<Event>, Vec<Event>) {
        let mut humanEvents = Vec::new();
        match event {
            Event::TrainArrival { lid, sid, tid } => {
                // each arrival always map to a departure, event for the last station
                let changeTime = self.lineTables.get(&lid).unwrap().get_stop_time(sid);

                // offload passengers, generate Event:HumanUnboardTrain
                let mut offcap = self.trains.get(&tid).unwrap().passengers_queue.len();
                loop {
                    if self.trains.get(&tid).unwrap().passengers_queue.is_empty() {
                        break;
                    } else if offcap == 0 {
                        // the loop has been looped through
                        break;
                    }
                    let hsf = self.trains.get_mut(&tid).unwrap().passengers_queue.pop_front().unwrap();
                    if hsf.2 == sid {
                        // TODO: this passenger needs to get off, generate HumanUnboardEvent, and possibly delay event
                        // ignore delay event for now
                        humanEvents.push(Event::HumanUnboardTrain{hid: hsf.0, lid: hsf.3, sid: hsf.2, tid, trid: hsf.5});
                    } else {
                        // this passenger does not get off at this station
                        self.trains.get_mut(&tid).unwrap().passengers_queue.push_back(hsf);
                    }
                    offcap -= 1;
                }
                
                (changeTime, Some(Event::TrainDeparture { lid, sid, tid }), humanEvents)
            }
            Event::TrainDeparture { lid, sid, tid } => {
                self.train_update(event, tid);
                // return a new event if there is next station, return none if no next station
                let nextStation = self.lineTables.get(&lid).unwrap().get_next_station(sid);
                match nextStation {
                    Some(st) => {
                        let dis = self
                            .lineTables
                            .get(&lid)
                            .unwrap()
                            .get_dis_next(sid)
                            .unwrap();

                        // onload passengers, genetate Event:HumanBoardTrain
                        // consume the station queue; if there's no next station we do not load people :) 
                        let mut oncap = 50;
                        loop {
                            if self.stations.get(&sid).unwrap().wait_queue.is_empty() {
                                break;
                            } else if oncap == 0 {
                                break;
                            }
                            let hsf = self.stations.get_mut(&sid).unwrap().wait_queue.pop_front().unwrap();
                            if hsf.3 == lid {
                                // this human get onboard, push into train passengers_queue
                                self.trains.get_mut(&tid).unwrap().passengers_queue.push_back(hsf);

                                // TODO: generate HumanBoardTrain Event; May generate HumanExtraWaitEvent
                                // ignore wait event for now
                                humanEvents.push(Event::HumanBoardTrain{hid: hsf.0, lid: hsf.3, sid: hsf.1, tid, trid: hsf.5});
                                // wait event: if "curr Time is larger than since time"
                            } else {
                                // put this human back, not waiting for this trian
                                self.stations.get_mut(&sid).unwrap().wait_queue.push_back(hsf);
                            }
                            oncap -= 1;
                        }


                        let speed = self.trains.get(&tid).unwrap().speed;
                        (dis / speed, Some(Event::TrainArrival { lid, sid: st, tid }), humanEvents)
                    }
                    None => (0, None, humanEvents),
                }
            }
            Event::HumanArriveStation {hid, sid, lid} => {
                (0, None, humanEvents)
            }
            Event::HumanEnteredStation {hid, sid, dsid, lid, trid} => {
                (0, None, humanEvents)
            }
            Event::HumanBoardTrain {hid, lid, sid, tid,trid} => {
                (0, None, humanEvents)
            }
            Event::HumanUnboardTrain{hid, lid, sid,tid, trid} => {
                (0, None, humanEvents)
            }
            Event::HumanLeaveStation{hid, sid} => {
                (0, None, humanEvents)
            }
        }
    }
    

    // a random trip generator
    pub fn randomTrip(&self) -> Vec<TripUnit>{
        let mut trip:Vec<TripUnit> = Vec::new();
        // select a randome line to start
        let mut rng = rand::thread_rng();
        let numLines = self.next_line;

        let lineStart = rng.gen_range(0..numLines);   // rand 取左开右闭区间

        let stationsWeights = self.lineTables.get(&lineStart).unwrap().total_weight;
        let startWeight = rng.gen_range(0..stationsWeights - 3);
        let startStation = self.lineTables.get(&lineStart).unwrap().find_station_weight(startWeight);
        let endWeight = rng.gen_range(startWeight+3..stationsWeights);
        let endStation = self.lineTables.get(&lineStart).unwrap().find_station_weight(endWeight);
    
        // generate one trip unit
        let startTime = rng.gen_range(360..1380);   // 6:00 - 23:00
        let endTime = startTime + 2 * (endStation.0 - startStation.0);
        trip.push(TripUnit::new(lineStart, startStation.1, endStation.1, startTime.try_into().unwrap(), endTime.try_into().unwrap()));


        // let numTranLines = self.stationOnLines.get(&endStation.1).unwrap().len();
        // // transit when there are more lines and a 1/3 chance (this is really a magic number here)
        // if numTranLines > 2 && rng.gen_range(0..3) == 0 {
        //     let mut tranLineIdx = 0;
        //     loop {
        //         tranLineIdx = rng.gen_range(0..numTranLines);
        //         if self.stationOnLines.get(&endStation.1).unwrap().get(tranLineIdx).unwrap() != &lineStart {
        //             break;
        //         }
        //     }
        //     let lineTrans = self.stationOnLines.get(&endStation.1).unwrap().get(tranLineIdx).unwrap();
            
        // }


       // let's ignore the transitting problem for now, although it looks like fun
        println!("generated a random trip {:?}", trip);
        trip
    }

    pub fn putWaitingHuman(&mut self, hid: HumanID, sid: StationID, dsid: StationID, lid: LineID, since: Time, trid: TripID) {
        self.stations.get_mut(&sid).unwrap().wait_queue.push_back((hid, sid, dsid, lid, since, trid));
    }
}
