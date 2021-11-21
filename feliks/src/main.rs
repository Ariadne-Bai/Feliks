use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};


// think about the design of data structures here
// a priority queue(scheduler) to hold the scheduled events
struct Scheduler {
    items: BinaryHeap<Item>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            items: BinaryHeap::new(),
        }
    }
}

// a trait/struct for train


// a trait/struct for metro timetable (start with a single line)

// a enum for event(start_at_station, stop_at_station)
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
enum EventType {
    TrainDeparture,
    TrainArrival,
}

// struct for a single event, to be pushed into the scheduler priority queue
#[derive(PartialEq, Eq, Clone)]
struct Item {
    time: u32,
    event_type: EventType,
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


fn main() {
    println!("Hello, world!");
    
    let mut sch = Scheduler::new();
    
    let item_1 = Item {
        time: 5,
        event_type: EventType::TrainArrival,
    };

    let item_2 = Item {
        time: 10,
        event_type: EventType::TrainDeparture,
    };

    sch.items.push(item_1);
    sch.items.push(item_2);

    loop {
        if sch.items.is_empty() {
            break;
        }
        println!("Time is {}", sch.items.peek().unwrap().time);
        sch.items.pop();
    }
}
