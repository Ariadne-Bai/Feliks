use crate::{custom_types::*, schedule::*};
use std::collections::HashMap;
// a trait/struct for train, holds state of the train
pub struct Train {
    // train id should be the same as the index of this train in TimeTable.start_times
    id: TrainID,
    // basic assumtion:
    // train run at the same speed as long as it start, no acceleration consitered
    speed: i32,
    state: TrainState,
}

impl Train {}

enum TrainState {
    // stop at a particular stationID
    StopInStation(StationID),
    // running on the trail after some station
    RunOnTrail(StationID),
}

// could have some property like 'capacity'
pub struct Station {
    id: StationID,
    name: String,
}

// a trait/struct for metro timetable (start with a single line)
// start with a list for stations first;
// should include a list of train start time during a day, like 6am, 8am....
// could be just simplified to a number for now
// in initialization, schedule events train arrival at first station for all trains starting on this day
pub struct LineTimeTable {
    id: LineID,
    name: String,
    stations: Vec<StationID>,
    new_train_times: Vec<Time>,
    // time for new train to start from the beginning station in the day
    // for the final station, the start time means when
    // the train leaves operation and all passengers should have get off the train
    stop_times: HashMap<StationID, Time>,
    // distance from a station to next; the last station does not have this number apparently
    // so it's either some distance or none :)
    distance_next: HashMap<StationID, Option<Distance>>,
}

pub struct StationTimeTable {
    id: StationID,
    stop_time: Time,
    distance_next: Option<Distance>,
}

impl StationTimeTable {
    pub fn new(id: StationID, stop_time: Time, distance_next: Option<Distance>) -> Self {
        StationTimeTable {
            id,
            stop_time,
            distance_next,
        }
    }
}

impl LineTimeTable {
    pub fn new(id: LineID, name: String) -> Self {
        LineTimeTable {
            id: id,
            name: name,
            stations: Vec::new(),
            new_train_times: Vec::new(),
            stop_times: HashMap::new(),
            distance_next: HashMap::new(),
        }
    }

    pub fn add_station(&mut self, stt: StationTimeTable) {
        self.stations.push(stt.id);
        self.stop_times.insert(stt.id, stt.stop_time);
        self.distance_next.insert(stt.id, stt.distance_next);
    }

    pub fn set_new_trains(&mut self, times: &Vec<Time>) {
        for time in times {
            // dereferencing the borrow. not fully understand this yet
            self.new_train_times.push(*time);
        }
    }

    // schedule events for train starting time;
    // TODO: move this functionality to the simulation engine
    // pub fn schedule_starts(&self, scheduler: &mut Scheduler) {
    //     for time in &self.new_train_times {
    //         scheduler.push(*time, Event::TrainArrival(self.stations[0]));
    //     }
    // }
}
