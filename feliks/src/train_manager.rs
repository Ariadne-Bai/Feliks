use crate::{custom_types::*, train::*};
use std::collections::HashMap;

/**
 * Central registry for trains, stations, lines
 * The Owner of all these objects
 * Is this a good design? I've got no idea
 */

pub struct TrainManager {
    lineTables: HashMap<LineID, LineTimeTable>,
    stationTables: HashMap<StationID, StationTimeTable>,
    stations: HashMap<StationID, Station>,
    trains: HashMap<TrainID, Train>,
}

impl TrainManager {
    pub fn new() -> Self {
        TrainManager {
            lineTables: HashMap::new(),
            stationTables: HashMap::new(),
            stations: HashMap::new(),
            trains: HashMap::new(),
        }
    }

    // TODO: train system initializations
    pub fn registerStation() {
        unimplemented!();
    }

    pub fn registerStationTable() {
        unimplemented!();
    }

    pub fn registerLineTable() {
        unimplemented!();
    }

    // register Train object, schedule initial event for train
    pub fn spawnTrains() {
        unimplemented!();
    }

    // TODO: look up and update state for some agent
    // agent have their own update() method, but manager will find which object to call
}
