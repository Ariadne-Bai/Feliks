use neo4rs::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use futures::stream::*;
use uuid::Uuid;

#[tokio::main]
async fn main() {
   let uri = "127.0.0.1:7687";
   let user = "neo4j";
   let pass = "felikslucky";
   let id = Uuid::new_v4().to_string();

   let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
   let mut result = graph.run(
     query("CREATE (p:Person {id: $id})").param("id", id.clone())
   ).await.unwrap(); 
   println!("first result: {:?}", result);
   let mut result2 = graph.run(
    query("CREATE (p:Train {id: $id, color: $color})").param("id", id.clone()).param("color", "BLUE")
  ).await.unwrap();
   println!("second result: {:?}", result2);

   let mut handles = Vec::new();
   let mut count = Arc::new(AtomicU32::new(0));
   for _ in 1..=42 {
       let graph = graph.clone();
       let id = id.clone();
       let count = count.clone();
       let handle = tokio::spawn(async move {
           let mut result = graph.execute(
             query("MATCH (p:Person {id: $id}) RETURN p").param("id", id)
           ).await.unwrap();
           while let Ok(Some(row)) = result.next().await {
               count.fetch_add(1, Ordering::Relaxed);
           }
       });
       handles.push(handle);
   }

   futures::future::join_all(handles).await;
   assert_eq!(count.load(Ordering::Relaxed), 42);
}
