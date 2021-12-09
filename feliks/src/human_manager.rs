use crate::{custom_types::*, schedule::*, human::*};
use std::collections::HashMap;

/**
 * central registry for human agents and logs
 */

 pub struct HumanManager {
     pub humans: HashMap<HumanID, Human>,
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
     
     // add the generated trip plan into human
     pub fn register_human(
         &mut self, plan: Vec<TripUnit>
     ) -> (HumanID, (String, String, String), TripID) {
         let mut hu = Human::new(self.next_human);
         hu.plan = plan;
         // also register a dummy triplog
         self.humans.insert(self.next_human, hu);
         
         match self.trip_logs.get_mut(&self.next_human) {
             Some(trips) => {
                 trips.push(self.next_trip);
             }
             None => {
                self.trip_logs.insert(self.next_human, [self.next_trip].to_vec());
             }
         }
         
         self.next_human += 1;
         self.next_trip += 1;

         // return a sql to register the human node
         let qs = format!(
             "CREATE (a:Human:Static {{ humanID: {}}})", self.next_human - 1
         );
         let qst = format! (
             "CREATE (t:TripLog {{ humanID: {}, tripID: {}}})", self.next_human - 1, self.next_trip - 1
         );
         let qstt = format! (
             "MATCH (a:Human), (t:TripLog) WHERE a.humanID = {} AND t.humanID = {} CREATE (a)-[:LOG]->(t)", self.next_human - 1, self.next_human -1
         );
         (self.next_human - 1, (qs, qst, qstt), self.next_trip - 1)
     }
 }