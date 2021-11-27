use crate::{customTypes::*};
use std::{collections::HashMap, hash::Hash};
// a trait/struct for train, holds state of the train
pub struct Train {
    // train id should be the same as the index of this train in TimeTable.start_times
    id: TrainID,
    // basic assumtion: 
    // train run at the same speed as long as it start, no acceleration consitered
    speed: i32,
    state: TrainState,
}

impl Train {

}

enum TrainState {
    // stop at a particular stationID
    StopInStation(StationID),
    // running on the trail after some station
    RunOnTrail(StationID),
}


// could have some property like 'capacity'
struct Station {
    id: StationID,
    name: String,
}

// a trait/struct for metro timetable (start with a single line)
// start with a list for stations first;
// should include a list of train start time during a day, like 6am, 8am....
// could be just simplified to a number for now
// in initialization, schedule events train arrival at first station for all trains starting on this day
struct LineTimeTable {
    id: LineID,
    name: String,
    stations: Vec<StationID>,
    // time for new train to start from the beginning station in the day
    // for the final station, the start time means when 
    // the train leaves operation and all passengers should have get off the train
    start_times: HashMap<StationID, Time>,
    // distance from a station to next; the last station does not have this number apparently
    // so it's either some distance or none :)
    distance_next: HashMap<StationID, Option<Distance>>,
}

struct StationTimeTable {
    id: StationID,
    start_time: Time,
    distance_next: Option<Distance>,
}

impl LineTimeTable {
    pub fn new(id: LineID, name: String) -> Self {
        LineTimeTable {
            id: id,
            name: name,
            stations: Vec::new(),
            start_times: HashMap::new(),
            distance_next: HashMap::new(),
        }
    }

    pub fn add_tation(&mut self, stt: StationTimeTable) {
        self.stations.push(stt.id);
        self.start_times.insert(stt.id, stt.start_time);
        self.distance_next.insert(stt.id, stt.distance_next);
    }
}