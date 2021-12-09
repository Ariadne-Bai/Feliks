use crate::{custom_types::*, schedule::*, human::*};
use std::collections::HashMap;

/**
 * central registry for human agents and logs
 */

 pub struct HumanManager {
     humans: HashMap<HumanID, Human>,
     trip_logs: HashMap<HumanID, Vec<TripID>>,   // every day, there should be a new trip node
     next_human: HumanID,
     next_trip: TripID,
 }

 impl HumanManager {
     pub fn new() -> Self {
         HumanManager {
             humans : HashMap::new(),
             trip_logs: HashMap::new(),
             next_human: 0,
             next_trip: 0,
         }
     }
     
     // add the generated trip into human
     pub fn register_human(
         &mut self, plan: Vec<TripUnit>
     ) -> (HumanID, String) {
         let mut hu = Human::new(self.next_human);
         hu.plan = plan;
         // also register a dummy triplog
         self.humans.insert(self.next_human, hu);
         
         self.trip_logs.insert(self.next_human, [self.next_trip].to_vec());
         self.next_human += 1;
         self.next_trip += 1;

         // return a sql to register the human node
         (self.next_human - 1, "".to_string())
     }
 }