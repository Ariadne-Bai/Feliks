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
    pub train_manager: &'a mut TrainManager<'a>,
}

impl<'a> SimEngine<'a> {
    pub fn new(sch: &'a mut Scheduler, trm: &'a mut TrainManager<'a>) -> Self {
        SimEngine {
            count_sim: 0,
            scheduler: sch,
            train_manager: trm,
        }
    }

    // sim engine should be responsible for spawning new agents
    // for each line, schedule event for each start time, and register a trian object
    pub fn spawn_train(&mut self, time: Time, lid: LineID, sid: StationID, tid: TrainID) {
        self.scheduler.push(time, Event::TrainArrival{lid, sid, tid});
    }

    pub fn do_step(&mut self, cur_time: Time) {
        while let Some(event) = self.scheduler.consume(cur_time) {
            self.count_sim += 1;
            match event {
                Event::TrainArrival{ lid, sid, tid} => {
                    let res = self.train_manager.handle_event(event);
                    self.scheduler.push(cur_time + res.0, res.1.unwrap());
                }
                Event::TrainDeparture{ lid, sid, tid} => {
                    let res = self.train_manager.handle_event(event);
                    // only schedule new event if there is another station; otherwise do nothing
                    res.1.map(|event| {
                        self.scheduler.push(cur_time + res.0, event);
                    });
                }
            }
        }
        // the loop should break when there is no more event at this time point
    }
}


#[cfg(test)]
mod test {
    use crate::train::StationTimeTable;

    use super::*;

    #[test]
    fn singleLineTrains() {
        let mut sch = Scheduler::new();
        let mut trmanager = TrainManager::new();
        let mut simengine = SimEngine::new(&mut sch, &mut trmanager);
        
        let appId = simengine.train_manager.register_station("Apple".to_string(), 0, 0);
        let banId = simengine.train_manager.register_station("Banana".to_string(), 50, 0);
        let cocId =  simengine.train_manager.register_station("Coconut".to_string(), 150, 0);
        let draId = simengine.train_manager.register_station("Dragonfruit".to_string(), 150, 100);

        let fruityId = simengine.train_manager.register_linetable("FRUITY".to_string(), 10);
        
        // add station timetables
        simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(appId, 5, Some(50), Some(banId)));
        simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(banId, 6, Some(100), Some(cocId)));
        simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(cocId, 7, Some(100), Some(draId)));
        simengine.train_manager.add_station_toline(fruityId, StationTimeTable::new(draId, 8, None, None));
         
        // expected results:
        // train 0: (s0, Arr, 10), (s0, Dep, 15), (s1, Arr, 20), (s1, Dep, 26), (s2, Arr, 36), (s2, Dep, 43), (s3, Arr, 53), (s3, Dep, 61)
        // train 0: (s0, Arr, 35), (s0, Dep, 40), (s1, Arr, 45), (s1, Dep, 51), (s2, Arr, 61), (s2, Dep, 68), (s3, Arr, 78), (s3, Dep, 86)

        // new train will start from Station Apple on Time 0 and Time 75;
        let new_times = vec![10, 35];

        for time in new_times {
            simengine.train_manager.add_new_traintime_toline(fruityId, time);
            let ntid = simengine.train_manager.register_train(fruityId, appId, time);
            simengine.spawn_train(time, fruityId, appId, ntid);
        }
        
        let mut clock = 0;
        loop {
            if clock >= 200 {
                break;
            }
            simengine.do_step(clock);
            clock += 1;
        }
        // set up a line
        // pop up the line with 3 stations
        // pop up the line with 2 trains for the day
        // spawn trains for the line
        // do_step and see what got pushed/consumed ax
    }
    
}
