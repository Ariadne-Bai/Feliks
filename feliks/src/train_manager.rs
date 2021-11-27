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
            next_train: 0,
            phantom: PhantomData,
        }
    }

    // TODO: train system initializations

    pub fn registerStation() {
        unimplemented!();
    }

    pub fn registerLineTable() {
        unimplemented!();
    }
    
    // register Train object; SimEngine should be responsible for spawn initial events
    pub fn registerTrain() {

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
