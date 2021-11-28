pub mod custom_types;
pub mod schedule;
pub mod sim_engine;
pub mod train;
pub mod train_manager;
use crate::custom_types::*;
use crate::sim_engine::SimEngine;
use crate::train_manager::TrainManager;
use schedule::Scheduler;

use crate::train::{LineTimeTable, StationTimeTable};

// neo4j related
use neo4rs::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use futures::stream::*;
use uuid::Uuid;




async fn createNeo() -> Arc<Graph> {
   let uri = "127.0.0.1:7687";
   let user = "neo4j";
   let pass = "felikslucky";

   let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
   graph
}

async fn execute(qs: &String, g: &Arc<Graph>) {
    g.run(
        query(&qs).param("id", 888)
      ).await.unwrap(); 
}


// a trait/struct for train, holds state of the train
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let g = createNeo().await;
    let test_q_s = "CREATE (p:Person {id: $id})".to_string();
    // execute(test_q_s, g).await;

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
            let qss = simengine.do_step(clock);
            // add the newly generated events to the database
            for qs in qss {
                execute(&qs, &g).await;
            }

            clock += 1;
        }

        println!("Count Sim Computation {} ", simengine.count_sim);
}
