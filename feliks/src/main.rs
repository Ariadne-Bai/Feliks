pub mod custom_types;
pub mod schedule;
pub mod sim_engine;
pub mod train;
pub mod train_manager;
use crate::custom_types::*;
use schedule::Scheduler;

use crate::train::{LineTimeTable, StationTimeTable};

// a trait/struct for train, holds state of the train
fn main() {
    println!("Hello, world!");
    let mut sch = Scheduler::new();

    // initialize a line time table and see it the scheduling works
    let mut line_table = LineTimeTable::new(0, "BLUE".to_string());

    line_table.add_station(StationTimeTable::new(0, 5, Some(10)));

    let times: Vec<Time> = vec![10, 20, 30];
    line_table.set_new_trains(&times);
}
