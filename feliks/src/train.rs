use crate::{custom_types::*, schedule::*};
use std::collections::HashMap;
// a trait/struct for train, holds state of the train
pub struct Train {
    // train id should be the same as the index of this train in TimeTable.start_times
    id: TrainID,
    line: LineID,
    // basic assumtion:
    // train run at the same speed as long as it start, no acceleration consitered
    pub speed: u32,
    state: TrainState,
}

impl Train {
    pub fn new(id: TrainID, line: LineID, speed: u32, sid: StationID, time: Time) -> Self {
        Train { 
            id, 
            line, 
            speed, 
            state: TrainState::StopInStation{ sid: sid, since: time },
        }
    }
    
    // TODO : this has not been implemented!!!!!!
    pub fn update_state(&mut self, event: Event) {
        
    }
}

// my current understanding is that train state is mainly for GUI drawing
enum TrainState {
    // stop at a particular stationID
    StopInStation{sid: StationID, since: Time},
    // running on the trail after some station
    RunOnTrail{sid: StationID, since: Time},
    // the train has departed the last station, which means it has finished all travel
    Finished{since: Time},
}

// could have some property like 'capacity', and state of 'crowd size'
pub struct Station {
    id: StationID,
    name: String,
    corx: Distance,
    cory: Distance,
}

impl Station {
    pub fn new(id: StationID, name: String, corx: Distance, cory: Distance) -> Self {
        Station { id, name, corx, cory }
    }
}

// a trait/struct for metro timetable (start with a single line)
// start with a list for stations first;
// should include a list of train start time during a day, like 6am, 8am....
// could be just simplified to a number for now
// in initialization, schedule events train arrival at first station for all trains starting on this day
pub struct LineTimeTable {
    id: LineID,
    name: String,
    speed: u32,
    stations: Vec<StationID>,
    new_train_times: Vec<Time>,
    station_tables: HashMap<StationID, StationTimeTable>,
}

// this is just a tmp structure for setting up a line time table
pub struct StationTimeTable {
    id: StationID,
    stop_time: Time,
    distance_next: Option<Distance>,
    station_next: Option<StationID>,
}

impl StationTimeTable {
    pub fn new(id: StationID, stop_time: Time, dn: Option<Distance>, sn: Option<StationID>) -> Self {
        StationTimeTable {
            id,
            stop_time,
            distance_next: dn,
            station_next: sn,
        }
    }
}

impl LineTimeTable {
    // a line has a defualt train speed
    pub fn new(id: LineID, name: String, speed: u32) -> Self {
        LineTimeTable {
            id: id,
            name: name,
            speed: speed,
            stations: Vec::new(),
            new_train_times: Vec::new(),
            station_tables: HashMap::new(),
        }
    }

    pub fn add_station(&mut self, stt: StationTimeTable) {
        self.stations.push(stt.id);
        self.station_tables.insert(stt.id, stt);
    }

    pub fn add_new_traintime(&mut self, time: Time) {
        self.new_train_times.push(time);
    }

    pub fn get_stop_time(&self, sid: StationID) -> Time {
        self.station_tables.get(&sid).unwrap().stop_time
    }

    pub fn get_next_station(&self, sid: StationID) -> Option<StationID> {
        self.station_tables.get(&sid).unwrap().station_next
    }

    pub fn get_dis_next(&self, sid: StationID) -> Option<Distance> {
        self.station_tables.get(&sid).unwrap().distance_next
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn get_new_times(&self) -> &Vec<Time> {
        &self.new_train_times
    }

    pub fn get_first_station(&self) -> StationID {
        *self.stations.get(0).unwrap()
    }

    // schedule events for train starting time;
    // TODO: move this functionality to the simulation engine
    // pub fn schedule_starts(&self, scheduler: &mut Scheduler) {
    //     for time in &self.new_train_times {
    //         scheduler.push(*time, Event::TrainArrival(self.stations[0]));
    //     }
    // }
}
