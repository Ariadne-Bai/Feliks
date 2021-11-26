

type TrainID = u32;
type StationID = u32;

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

// a trait/struct for metro timetable (start with a single line)
// start with a list for stations first;
// should include a list of train start time during a day, like 6am, 8am....
// could be just simplified to a number for now
// in initialization, schedule events train arrival at first station for all trains starting on this day
struct TimeTable {
    line_name: String,
    stations: Vec<Station>,
    start_times: Vec<u32>,
}

struct Station {
    id: StationID,
    name: String,
    has_next: bool,
    distance_to_next: u32,
    stop_time: u32,
}