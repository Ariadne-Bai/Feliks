use crate::{custom_types::*, schedule::*, human::*};
use std::collections::HashMap;
use std::collections::VecDeque;
// a trait/struct for train, holds state of the train
pub struct Train {
    // train id should be the same as the index of this train in TimeTable.start_times
    id: TrainID,
    line: LineID,
    // basic assumtion:
    // train run at the same speed as long as it start, no acceleration consitered
    pub speed: u32,
    state: TrainState,
    pub passengers_queue: VecDeque<HumanState>,
}

impl Train {
    pub fn new(id: TrainID, line: LineID, speed: u32, sid: StationID, time: Time) -> Self {
        Train {
            id,
            line,
            speed,
            state: TrainState::StopInStation {
                sid: sid,
                since: time,
            },
            passengers_queue: VecDeque::new(),
        }
    }

    // TODO : this has not been implemented!!!!!!
    pub fn update_state(&mut self, event: Event) {}
}

// my current understanding is that train state is mainly for GUI drawing
enum TrainState {
    // stop at a particular stationID
    StopInStation { sid: StationID, since: Time },
    // running on the trail after some station
    RunOnTrail { sid: StationID, since: Time },
    // the train has departed the last station, which means it has finished all travel
    Finished { since: Time },
}

// could have some property like 'capacity', and state of 'crowd size'
pub struct Station {
    id: StationID,
    name: String,
    corx: Distance,
    cory: Distance,
    pub wait_queue: VecDeque<HumanState>,
}

impl Station {
    pub fn new(id: StationID, name: String, corx: Distance, cory: Distance) -> Self {
        Station {
            id,
            name,
            corx,
            cory,
            wait_queue: VecDeque::new(),
       }
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
    pub total_weight: u32,   // Eweight value of all stations on this line
    speed: u32,
    pub stations: Vec<StationID>,
    new_train_times: Vec<Time>,
    station_acc_weights: Vec<u32>,
    station_tables: HashMap<StationID, StationTimeTable>,
}

// this is just a tmp structure for setting up a line time table
pub struct StationTimeTable {
    pub id: StationID,
    pub stop_time: Time,
    pub weight: u32,
    pub distance_next: Option<Distance>,
    pub station_next: Option<StationID>,
}

impl StationTimeTable {
    pub fn new(
        id: StationID,
        stop_time: Time,
        weight: u32,
        dn: Option<Distance>,
        sn: Option<StationID>,
    ) -> Self {
        StationTimeTable {
            weight,
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
            total_weight: 0,
            speed: speed,
            stations: Vec::new(),
            new_train_times: Vec::new(),
            station_acc_weights: Vec::new(),
            station_tables: HashMap::new(),
        }
    }

    pub fn add_station(&mut self, stt: StationTimeTable) {
        self.stations.push(stt.id);
        self.total_weight += stt.weight;
        self.station_acc_weights.push(self.total_weight);
        self.station_tables.insert(stt.id, stt);
    }

    pub fn find_station_weight(&self, weight: u32) -> (usize, StationID) {
        for (idx, w) in self.station_acc_weights.iter().enumerate() {
            if w >= &weight {
                return (idx, *self.stations.get(idx).unwrap())
            }
        }
        // should not reach here
        println!("Cannot find station with weight {} on line {}", weight, self.id);
        return (0, 0);
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

    pub fn get_last_station(&self, sid: StationID) -> Option<StationID> {
        if sid == self.get_first_station() {
            None
        } else {
            let index = self.stations.iter().position(|&r| r == sid).unwrap();
            Some(*self.stations.get(index - 1).unwrap())
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
