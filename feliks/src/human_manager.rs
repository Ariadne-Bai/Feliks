use crate::{custom_types::*, schedule::*, human::*};
use std::collections::HashMap;

/**
 * central registry for human agents and logs
 */

 pub struct HumanManager {
     humans: HashMap<HumanID, Human>,
     tripLogs: HashMap<HumanID, Vec<TripID>>,
     next_human: HumanID,
     next_trip: TripID,
 }

 impl HumanManager {
     pub fn new() -> Self {
         HumanManager {
             humans : HashMap::new(),
             tripLog: HashMap::new(),
             next_human: 0,
             next_trip: 0,
         }
     }

     pub fn register_human(
         &mut self,
     ) -> (HumanID, String) {
         let hu = Human::new();
         // also register a dummy triplog
     }
 }