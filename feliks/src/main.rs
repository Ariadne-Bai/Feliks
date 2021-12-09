pub mod custom_types;
pub mod schedule;
pub mod sim_engine;
pub mod train;
pub mod train_manager;
pub mod human;
pub mod human_manager;
use crate::custom_types::*;
use crate::sim_engine::SimEngine;
use crate::train_manager::TrainManager;
use crate::human_manager::HumanManager;
use schedule::Scheduler;

use crate::train::{LineTimeTable, StationTimeTable};

// neo4j related
use futures::stream::*;
use neo4rs::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use uuid::Uuid;

async fn createNeo() -> Arc<Graph> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "felikslucky";

    let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
    graph
}

async fn execute(qs: &String, g: &Arc<Graph>) {
    g.run(query(&qs).param("id", 888)).await.unwrap();
}

// a trait/struct for train, holds state of the train
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let neoflag = true;

    let g = createNeo().await;

    let mut sch = Scheduler::new();
    let mut trmanager = TrainManager::new();
    let mut humanager = HumanManager::new();
    let mut simengine = SimEngine::new(&mut sch, &mut trmanager, humanager);

    let mut qs_stations = Vec::new();
    let appId = simengine
        .train_manager
        .register_station("Apple".to_string(), 0, 0);
    qs_stations.push(&appId.1);
    let banId = simengine
        .train_manager
        .register_station("Banana".to_string(), 50, 0);
    qs_stations.push(&banId.1);
    let cocId = simengine
        .train_manager
        .register_station("Coconut".to_string(), 150, 0);
    qs_stations.push(&cocId.1);
    let draId = simengine
        .train_manager
        .register_station("Dragonfruit".to_string(), 150, 100);
    qs_stations.push(&draId.1);

    let choId = simengine
        .train_manager
        .register_station("Chocolate".to_string(), 100, 100);
    qs_stations.push(&choId.1);
    let milId = simengine
        .train_manager
        .register_station("Milk".to_string(), 200, 200);
    qs_stations.push(&milId.1);

    // add stations to neo
    if neoflag {
        for qs in qs_stations {
            println!("query string for new station {}", &qs);
            execute(&qs, &g).await;
        }
    }
    

    let fruityId = simengine
        .train_manager
        .register_linetable("FRUITY".to_string(), 10);
    let sweetyId = simengine
        .train_manager
        .register_linetable("SWEETY".to_string(), 3);

    // add station timetables
    let mut qstb = Vec::new();

    let mut qstb_app = simengine.train_manager.add_station_toline(
        fruityId,
        StationTimeTable::new(appId.0, 5, 1, Some(50), Some(banId.0)),
    );
    qstb.append(&mut qstb_app);

    let mut qstb_ban = simengine.train_manager.add_station_toline(
        fruityId,
        StationTimeTable::new(banId.0, 6, 1,Some(100), Some(cocId.0)),
    );
    qstb.append(&mut qstb_ban);

    let mut qstb_coc = simengine.train_manager.add_station_toline(
        fruityId,
        StationTimeTable::new(cocId.0, 7, 2,Some(100), Some(draId.0)),
    );
    qstb.append(&mut qstb_coc);

    let mut qstb_dra = simengine
        .train_manager
        .add_station_toline(fruityId, StationTimeTable::new(draId.0, 8, 3,None, None));
    qstb.append(&mut qstb_dra);

    let mut qstb_cho = simengine.train_manager.add_station_toline(
        sweetyId,
        StationTimeTable::new(choId.0, 10, 2,Some(70), Some(cocId.0)),
    );
    qstb.append(&mut qstb_cho);

    let mut qstb_coc_sw = simengine.train_manager.add_station_toline(
        sweetyId,
        StationTimeTable::new(cocId.0, 10, 2,Some(70), Some(milId.0)),
    );
    qstb.append(&mut qstb_coc_sw);

    let mut qstb_mil = simengine
        .train_manager
        .add_station_toline(sweetyId, StationTimeTable::new(milId.0, 10, 3,None, None));
    qstb.append(&mut qstb_mil);

    // add station tables and next station links to neo
    if neoflag {
        for qs in qstb {
            println!("query string for new station {}", &qs);
            execute(&qs, &g).await;
        }
    }

    // expected results:
    // train 0: (s0, Arr, 10), (s0, Dep, 15), (s1, Arr, 20), (s1, Dep, 26), (s2, Arr, 36), (s2, Dep, 43), (s3, Arr, 53), (s3, Dep, 61)
    // train 0: (s0, Arr, 35), (s0, Dep, 40), (s1, Arr, 45), (s1, Dep, 51), (s2, Arr, 61), (s2, Dep, 68), (s3, Arr, 78), (s3, Dep, 86)

    // new train will start from Station Apple on Time 0 and Time 75;
    let mut new_times = Vec::new();
    let mut newt = 400;  // every 60 minutes
    loop {
        if (newt >= 1380) {
            break;
        }
        new_times.push(newt);
        newt += 60;
    }
    let mut new_times_sweety = Vec::new();
    let mut new_sweett = 420;  // every 60 minutes
    loop {
        if (new_sweett >= 1380) {
            break;
        }
        new_times_sweety.push(new_sweett);
        new_sweett += 60;
    }

    for time in new_times {
        simengine
            .train_manager
            .add_new_traintime_toline(fruityId, time);
        let ntid = simengine
            .train_manager
            .register_train(fruityId, appId.0, time);
        simengine.spawn_train(time, fruityId, appId.0, ntid);
    }

    for time in new_times_sweety {
        simengine
            .train_manager
            .add_new_traintime_toline(sweetyId, time);
        let ntid = simengine
            .train_manager
            .register_train(sweetyId, choId.0, time);
        simengine.spawn_train(time, sweetyId, choId.0, ntid);
    }

    let mut qs_human = Vec::new();
    // create human
    let mut numHumans = 0;
    loop {
        if numHumans >= 15 {
            break;
        }

        let trip = simengine.train_manager.randomTrip();
        let plan = *trip.first().unwrap();
        let human = simengine.human_manager.register_human(trip);
        
        // let btrip = &simengine.human_manager.humans.get(&human.0).unwrap().plan;
        simengine.spawn_human(plan, human.0, human.2);
        qs_human.push(human.1.0);
        qs_human.push(human.1.1);
        qs_human.push(human.1.2);
        numHumans += 1;
    }
    // add humans to neo
    if neoflag {
        for qs in qs_human {
            println!("query string for new human {}", &qs);
            execute(&qs, &g).await;
        }
    }


    let mut clock = 360;
    loop {
        if clock >= 1440 {
            break;
        }
        let qss = simengine.do_step(clock);
        // add the newly generated events to the database
        if (neoflag) {
            for qs in qss {
                println!("got query {}", qs);
                execute(&qs, &g).await;
            }
        }
        

        clock += 1;
    }

    println!("Count Sim Computation {} ", simengine.count_sim);
}
