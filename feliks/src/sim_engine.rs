use std::io::BufRead;

use crate::{custom_types::*, schedule::*, train_manager::*, human::*, human_manager::HumanManager};

/**
 * drive state updated in the whole system
 */

pub struct SimEngine<'a> {
    // number of state computation the system generates
    pub count_sim: u32,
    // the SimEngine itself does not holds any state. but it mutate states of others
    scheduler: &'a mut Scheduler,
    pub train_manager: &'a mut TrainManager<'a>,
    pub human_manager: HumanManager,
}

impl<'a> SimEngine<'a> {
    pub fn new(sch: &'a mut Scheduler, trm: &'a mut TrainManager<'a>, hum: HumanManager) -> Self {
        SimEngine {
            count_sim: 0,
            scheduler: sch,
            train_manager: trm,
            human_manager: hum,
        }
    }

    // sim engine should be responsible for spawning new agents
    // for each line, schedule event for each start time, and register a trian object
    pub fn spawn_train(&mut self, time: Time, lid: LineID, sid: StationID, tid: TrainID) {
        self.scheduler
            .push(time, Event::TrainArrival { lid, sid, tid });
    }

    pub fn spawn_human(&mut self, plan: TripUnit, hid: HumanID, trid: TripID) {
        // if let Some(trip) = plan.first() {
        //     let lid = trip.line;
        //     let sid = trip.on;
        //     let dsid = trip.off;
        //     let time = trip.start_time;
        //     self.scheduler.push(time, Event::HumanEnteredStation { hid, sid, dsid, lid, trid});
        // }
        let lid = plan.line;
        let sid = plan.on;
        let dsid = plan.off;
        let time = plan.start_time;
        // start a new trip each day
        // let new_trid = self.human_manager.new_trip(hid);
        // let's ignore new trip each day for now
        self.scheduler.push(time, Event::HumanEnteredStation { hid, sid, dsid, lid, trid});
    }

    pub fn do_step(&mut self, cur_time: Time) -> Vec<String> {
        let mut resqs = Vec::new();
        while let Some(event) = self.scheduler.consume(cur_time) {
            self.count_sim += 1;
            match event {
                Event::TrainArrival { lid, sid, tid } => {
                    let res = self.train_manager.handle_event(event, cur_time);
                    self.scheduler.push(cur_time + res.0, res.1.unwrap());

                    // schedule offload event. offload event happens after this event, to make sure the Arrival node already exist
                    for pevent in res.2.into_iter() {
                        self.scheduler.push(cur_time + 1, pevent);
                    }

                    let qs = format!("CREATE (v:TrainArrival:Event {{ lineID:{}, stationID:{}, trainID:{}, time:{}}})", lid, sid, tid, cur_time);
                    resqs.push(qs);
                    let qstation = format!("MATCH (v:TrainArrival:Event), (st:Station) WHERE v.stationID = {} AND v.trainID = {} AND st.stationID = {} CREATE (v)-[:AT_STATION]->(st)", sid, tid, sid);
                    resqs.push(qstation);
                    // find the departure event on the previous station
                    if let Some(psid) = self.train_manager.find_last_station(lid, sid) {
                        let qspst = format!("MATCH (v:TrainArrival:Event), (pv:TrainDeparture) WHERE v.lineID = {} AND v.stationID = {} AND v.trainID = {} AND pv.lineID = {} AND pv.stationID = {} AND pv.trainID = {} CREATE (pv)-[:RUN]->(v)", lid, sid, tid, lid, psid, tid);
                        resqs.push(qspst);
                    }
                }
                Event::TrainDeparture { lid, sid, tid } => {
                    let res = self.train_manager.handle_event(event, cur_time);
                    // only schedule new event if there is another station; otherwise do nothing
                    res.1.map(|event| {
                        self.scheduler.push(cur_time + res.0, event);
                    });

                    // schedule onload event, onload happens after this event
                    for pevent in res.2.into_iter() {
                        self.scheduler.push(cur_time + 1, pevent);
                    }

                    let qs = format!("CREATE (v:TrainDeparture:Event {{ lineID:{}, stationID:{}, trainID:{}, time:{}}})", lid, sid, tid, cur_time);
                    resqs.push(qs);
                    let qs_st = format!("MATCH (v:TrainDeparture), (pv:TrainArrival) WHERE v.lineID = {} AND v.stationID = {} AND v.trainID = {} AND pv.lineID = {} AND pv.stationID = {} AND pv.trainID = {} CREATE (pv)-[:STOP]->(v)", lid, sid, tid, lid, sid, tid);
                    resqs.push(qs_st);
                    let qstation = format!("MATCH (v:TrainDeparture:Event), (st:Station) WHERE v.stationID = {} AND v.trainID = {} AND st.stationID = {} CREATE (v)-[:AT_STATION]->(st)", sid, tid, sid);
                    resqs.push(qstation);
                }
                Event::HumanArriveStation {hid, sid, lid} => {
                    // ignore this case for now
                    
                }
                Event::HumanEnteredStation {hid, sid, dsid, lid, trid} => {
                    // push this human into the station waiting queue
                    // ignore reschedule for now
                    self.train_manager.putWaitingHuman(hid, sid, dsid, lid, cur_time, trid);

                    // SQL: create a trip node, create trip - prevTrip, trip - start, trip - end relationship
                    // this could be done in the spawn human method
                    let qs_start = format!(
                        "MATCH (st:Station), (tr: TripLog) WHERE st.stationID = {} AND tr.tripID = {} CREATE (tr)-[:PLAN_START]->(st)", sid, trid
                    ); 
                    resqs.push(qs_start);
                    let qs_end = format!(
                        "MATCH (st:Station), (tr: TripLog) WHERE st.stationID = {} AND tr.tripID = {} CREATE (tr)-[:PLAN_END]->(st)", dsid, trid
                    );
                    resqs.push(qs_end);
                }
                Event::HumanBoardTrain {hid, lid, sid, tid, trid} => {
                    // SQL: HumanBoardTrain Node
                    // SQL: station - event relationship
                    // SQL: trip - event relationship
                    let qs_event = format!(
                        "CREATE (v:HumanBoardTrain:Event {{ humanID: {}, lineID:{}, stationID:{}, trainID:{}, tripID:{}, time:{}}})", hid, lid, sid, tid, trid, cur_time
                    );
                    resqs.push(qs_event);
                    let qs_station = format!(
                        "MATCH (st:Station), (v:HumanBoardTrain) WHERE st.stationID = {} AND v.stationID =  {} AND v.tripID = {} CREATE (v)-[:STATION]->(st)", sid, sid, trid
                    );
                    resqs.push(qs_station);
                    let qs_trip = format!(
                        "MATCH (v:HumanBoardTrain), (tr:TripLog) WHERE v.tripID = {} AND tr.tripID = {} CREATE (tr)-[:START]->(v)", trid, trid
                    );
                    resqs.push(qs_trip);
                    println!("A HumanBoardTrain Event hid {} lid {} sid {} tid {} trid {}", hid, lid, sid, tid, trid);
                }
                Event::HumanUnboardTrain{hid, lid, sid,tid, trid} => {
                    // SQL: HumanUnboardTrain Node
                    // SQL: station - unboard relationship 
                    // SQL: trip - event relationship
                    let qs_event = format!(
                        "CREATE (v:HumanUnboardTrain:Event {{ humanID: {}, lineID:{}, stationID:{}, trainID:{}, tripID:{}, time:{}}})", hid, lid, sid, tid, trid, cur_time
                    );
                    resqs.push(qs_event);
                    let qs_station = format!(
                        "MATCH (st:Station), (v:HumanUnboardTrain) WHERE st.stationID = {} AND v.stationID =  {} AND v.tripID = {} CREATE (v)-[:STATION]->(st)", sid, sid, trid
                    );
                    resqs.push(qs_station);
                    let qs_trip = format!(
                        "MATCH (v:HumanUnboardTrain), (tr:TripLog) WHERE v.tripID = {} AND tr.tripID = {} CREATE (tr)-[:END]->(v)", trid, trid
                    );
                    resqs.push(qs_trip);
                    println!("A HumanUnoardTrain Event hid {} lid {} sid {} tid {} trid {}", hid, lid, sid, tid, trid);
                }
                Event::HumanLeaveStation{hid, sid} => {
                    
                }
                Event::HumanWait{hid, lid, sid, trid, waitTime} => {
                    let qs_wait = format!(
                        "CREATE (v:HumanWait:Event {{ humanID: {}, lineID:{}, stationID:{}, tripID:{}, waitTime:{}}})", hid, lid, sid, trid, waitTime
                    );
                
                    resqs.push(qs_wait);
                    let qs_trip = format!(
                        "MATCH (v:HumanWait), (tr:TripLog) WHERE v.tripID = {} AND tr.tripID = {} CREATE (tr)-[:HAS_WAIT]->(v)", trid, trid
                    );
                    resqs.push(qs_trip);
                    let qs_station = format!(
                        "MATCH (v:HumanWait), (st:Station) WHERE v.stationID = {} AND v.tripID = {} AND st.stationID = {} CREATE (v)-[:WAIT_AT]->(st)", sid, trid, sid
                    );
                    resqs.push(qs_station);
                }
            }
        }
        // the loop should break when there is no more event at this time point
        resqs
    }
}

// #[cfg(test)]
// mod test {
//     use crate::train::StationTimeTable;

//     use super::*;

//     #[test]
//     fn singleLineTrains() {
//         let mut sch = Scheduler::new();
//         let mut trmanager = TrainManager::new();
//         let mut simengine = SimEngine::new(&mut sch, &mut trmanager);

//         let appId = simengine.train_manager.register_station("Apple".to_string(), 0, 0);
//         let banId = simengine.train_manager.register_station("Banana".to_string(), 50, 0);
//         let cocId =  simengine.train_manager.register_station("Coconut".to_string(), 150, 0);
//         let draId = simengine.train_manager.register_station("Dragonfruit".to_string(), 150, 100);

//         let fruityId = simengine.train_manager.register_linetable("FRUITY".to_string(), 10);

//         // add station timetables
//         simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(appId, 5, Some(50), Some(banId)));
//         simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(banId, 6, Some(100), Some(cocId)));
//         simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(cocId, 7, Some(100), Some(draId)));
//         simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(draId, 8, None, None));

//         // expected results:
//         // train 0: (s0, Arr, 10), (s0, Dep, 15), (s1, Arr, 20), (s1, Dep, 26), (s2, Arr, 36), (s2, Dep, 43), (s3, Arr, 53), (s3, Dep, 61)
//         // train 0: (s0, Arr, 35), (s0, Dep, 40), (s1, Arr, 45), (s1, Dep, 51), (s2, Arr, 61), (s2, Dep, 68), (s3, Arr, 78), (s3, Dep, 86)

//         // new train will start from Station Apple on Time 0 and Time 75;
//         let new_times = vec![10, 35];

//         for time in new_times {
//             simengine.train_manager.add_new_traintime_toline(fruityId, time);
//             let ntid = simengine.train_manager.register_train(fruityId, appId, time);
//             simengine.spawn_train(time, fruityId, appId, ntid);
//         }

//         let mut clock = 0;
//         loop {
//             if clock >= 200 {
//                 break;
//             }
//             simengine.do_step(clock);
//             clock += 1;
//         }
//     }
