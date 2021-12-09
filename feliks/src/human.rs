use crate::{custom_types::*, schedule::*};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// how to be a human being? in a traffic simulation system?

pub struct Human {
    id: HumanID,
    // goal: TripUnit,   // src->dest. this is set when agents are initialized
    pub plan: Vec<TripUnit>,   // computed planed path (shortest path algorithm)
    // trip: Vec<TripUnit>,   // what actually happened
}

impl Human {
    pub fn new(id: HumanID) -> Self {
        Human {
            id,
            plan: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TripUnit {
    pub line: LineID,
    pub on: StationID,
    pub off:  StationID,
    pub start_time: Time,
    pub end_time: Time,
}

impl TripUnit {
    pub fn new(line: LineID, on: StationID, off: StationID, start: Time, end: Time) -> Self {
        TripUnit {
            line,
            on,
            off,
            start_time: start,
            end_time: end,
        }
    }
} 

// for simplicity we ignore ArriveStation event and EnteringStation state for now
pub enum HumanState {
    EnteringStation { hid: HumanID, sid: StationID, lid: LineID, since: Time },   // model things like security checks
    QueueingForTrain { hid: HumanID, sid: StationID, lid: LineID, since: Time },
    OnTrain { hid: HumanID, lid: LineID, tid: TrainID, to_sid: LineID, since: Time },  // plan to get off at to_sid station
    Finished { hid: HumanID, since: Time }, // for simplicity: consider off the last train as trip finished
}

// when a human is initialized
// compute shortest path plan and store it
// schedule HumanArriveStation at goal.start_time
// at HumanArriveStation, schedule HumanEnteredStation per station entering crowdness; add to station crowdness
// at HumanEnteredStation, reschedule HumanEnteredStation if station entering crowdness inceased a lot
// otherwise push Human into WaitingForLine queue
// at TrainArrival, first offboard anyone who's to_sid == sid (assume offboarding does not take time), schedule HumanBoardTrain immediately
// at TrainArrival, consume waitingForLine queue per trainCapacity, schedule HumanBoardTrain accordingly, decrease station crowdness
// at HumanBoardTrain, update HumanState to OnTrain (for offloading); if this agent decides to get off early, change this state
// at HumanUnBoardTrain, the human's trip is finished(schedule leave station) / or schedule next piece of trip, for simplicity not considering leaving train time, but, this will add to station crowdness!
// at HumanLeavesStation, decrease station crowdness
