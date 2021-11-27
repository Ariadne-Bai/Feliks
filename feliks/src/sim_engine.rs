use std::io::BufRead;

use crate::{custom_types::*, schedule::*, train_manager::{*, self}};

/**
 * drive state updated in the whole system
 */

pub struct SimEngine<'a> {
    // number of state computation the system generates
    count_sim: u32,
    // the SimEngine itself does not holds any state. but it mutate states of others
    scheduler: &'a mut Scheduler,
    train_manager: &'a mut TrainManager<'a>,
}

impl<'a> SimEngine<'a> {
    pub fn new(cnt: u32, sch: &'a mut Scheduler, trm: &'a mut TrainManager<'a>) -> Self {
        SimEngine {
            count_sim: cnt,
            scheduler: sch,
            train_manager: trm,
        }
    }

    pub fn do_step(&mut self, cur_time: Time) {
        while let Some(event) = self.scheduler.consume(cur_time) {
            match event {
                Event::TrainArrival{ sid, tid} => {
                    let res =self.train_manager.handle_event(event);
                    self.scheduler.push(cur_time + res.0, res.1.unwrap());
                }
                Event::TrainDeparture{ sid, tid} => {
                    let res = self.train_manager.handle_event(event);
                    res.1.map(|event| {
                        self.scheduler.push(cur_time + res.0, event);
                    });
                }
            }
        }
        // the loop should break when there is no more event at this time point
    }
}
