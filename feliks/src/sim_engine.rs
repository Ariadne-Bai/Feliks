use std::io::BufRead;

use crate::{custom_types::*, schedule::*, train_manager::*};

/**
 * drive state updated in the whole system
 */

pub struct SimEngine<'a> {
    // number of state computation the system generates
    count_sim: u32,
    // the SimEngine itself does not holds any state. but it mutate states of others
    scheduler: &'a mut Scheduler,
}

impl<'a> SimEngine<'a> {
    pub fn new(cnt: u32, sch: &'a mut Scheduler) -> Self {
        SimEngine {
            count_sim: cnt,
            scheduler: sch,
        }
    }

    pub fn do_step(&mut self, cur_time: Time) {
        while let Some(event) = self.scheduler.consume(cur_time) {
            match event {
                Event::TrainArrival{ sid, tid} => {
                    
                }
                Event::TrainDeparture{ sid, tid} => {

                }
            }
        }
        // the loop should break when there is no more event at this time point
    }
}
