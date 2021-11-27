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

    pub fn do_step(cur_time: Time) {
        
    }
}
