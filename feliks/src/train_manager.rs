use crate::{custom_types::*, schedule::*, train::*};
use std::{collections::HashMap, marker::PhantomData};

/**
 * Central registry for trains, stations, lines
 * The Owner of all these objects
 * Is this a good design? I've got no idea
 */

pub struct TrainManager<'a> {
    lineTables: HashMap<LineID, LineTimeTable>,
    stations: HashMap<StationID, Station>,
    trains: HashMap<TrainID, Train>,
    next_line: LineID,
    next_station: StationID,
    next_train: TrainID,
    phantom: PhantomData<&'a u32>
}

impl<'a> TrainManager<'a> {
    pub fn new() -> Self {
        TrainManager {
            lineTables: HashMap::new(),
            stations: HashMap::new(),
            trains: HashMap::new(),
            next_line: 0,
            next_station: 0, 
            next_train: 0,
            phantom: PhantomData,
        }
    }

    // TODO: train system initializations

    pub fn register_station(&mut self, name: String, corx: Distance, cory: Distance) -> (StationID, String) {
        let qs = format!("CREATE (a:Station:Static {{ name: \"{}\", corX: {}, corY: {}}})", &name, corx, cory);
        
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

    pub fn add_station_toline(&mut self, lid: LineID, stt: StationTimeTable) {
        match self.lineTables.get_mut(&lid) {
            Some(ltb) => {
                ltb.add_station(stt);
            }
            None => {
                println!("Add Station: there is no line with ID {}!", lid);
            }
        }
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
    
    // This does not work! always double borrow...
    // pub fn initialize_trains(&mut self) {
    //     for (lid, ltb) in &mut self.lineTables {
    //         let fsid = ltb.get_first_station();
    //         for stime in ltb.get_new_times() {
    //             self.register_train(*lid, fsid, *stime);
    //         }
    //     }
    // }

    pub fn handle_event(&mut self, event: Event) -> (Time, Option<Event>){
        
        match event {
            Event::TrainArrival{ lid, sid, tid} => {
                
                // each arrival always map to a departure, event for the last station
                let changeTime = self.lineTables.get(&lid).unwrap().get_stop_time(sid);
                (changeTime, Some(Event::TrainDeparture{lid, sid, tid}))
            }
            Event::TrainDeparture{ lid, sid, tid} => {
                self.train_update(event, tid);
                // return a new event if there is next station, return none if no next station
                let nextStation = self.lineTables.get(&lid).unwrap().get_next_station(sid);
                match nextStation {
                    Some(st) => {
                        let dis = self.lineTables.get(&lid).unwrap().get_dis_next(sid).unwrap();
                        let speed = self.trains.get(&tid).unwrap().speed;
                        (dis / speed, Some(Event::TrainArrival{lid, sid: st, tid}))
                    }
                    None => {
                        (0, None)
                    }
                }
            }
        }
    }
}
