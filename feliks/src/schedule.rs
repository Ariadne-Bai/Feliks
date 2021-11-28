use crate::custom_types::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// a enum for event(start_at_station, stop_at_station)
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub enum Event {
    // depart from a stationID
    TrainDeparture {
        lid: LineID,
        sid: StationID,
        tid: TrainID,
    },
    // arrive at a stationID
    TrainArrival {
        lid: LineID,
        sid: StationID,
        tid: TrainID,
    },
}

// think about the design of data structures here
// a priority queue(scheduler) to hold the scheduled events
pub struct Scheduler {
    items: BinaryHeap<Item>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            items: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, time: Time, event: Event) {
        println!("pushed a new event {:?} for time {}", event, time);
        self.items.push(Item { time, event });
    }

    // for a certain time point, if there are Event at this time point to happen
    // return Some<Event>. The Sim engine will push new event accordingly,
    // and call methods to update agent state
    pub fn consume(&mut self, cur_time: Time) -> Option<Event> {
        match self.items.pop() {
            None => None,
            Some(item) => {
                if item.time <= cur_time {
                    println!(
                        "successfully consumed a event {:?} at cur_time {}",
                        item.event, cur_time
                    );
                    Some(item.event)
                } else {
                    self.items.push(item);
                    None
                }
            }
        }
    }

    // temporary printing for test purpose
    pub fn pretty_print_top(&self) {
        println!("{:?}", self.items.peek().unwrap());
    }
}

// struct for a single item, to be pushed into the scheduler priority queue

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct Item {
    time: u32,
    event: Event,
}

// what's the relationship between an PartialOrd and an Ord??
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        // BinaryHeap is a max-heap, reverse the comparision to get smallest times first
        let ord = other.time.cmp(&self.time);
        return ord;
    }
}

// a do_step function for simulate another iteration, and process update
