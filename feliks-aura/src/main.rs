use std::{collections::HashMap, env};

use tokio::io::BufStream;
use tokio_util::compat::*;

use bolt_client::*;
use bolt_proto::{message::*, value::*, version::*, Message, Value};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's say you have a type that implements AsyncRead + AsyncWrite. Here's one
    // provided by the `tokio-stream` feature of this library. In this example, all
    // connection/authentication details are stored in environment variables.
    let stream = Stream::connect(env::var("BOLT_TEST_ADDR")?,
                                 env::var("BOLT_TEST_DOMAIN").ok()).await?;
    let stream = BufStream::new(stream).compat();

    // Create a new connection to the server and perform a handshake to establish a
    // protocol version. This example demonstrates usage of the v4.3 or v4.2 protocol.
    let mut result = Client::new(stream, &[V4_3, V4_2, 0, 0]).await;
    let mut client = result.unwrap();
     
    // Send a HELLO message with authentication details to the server to initialize
    // the session.
    let response: Message = client.hello(
        Metadata::from_iter(vec![
            ("user_agent", "my-client-name/1.0"),
            ("scheme", "basic"),
            ("principal", &env::var("BOLT_TEST_USERNAME")?),
            ("credentials", &env::var("BOLT_TEST_PASSWORD")?),
        ])).await?;
    assert!(Success::try_from(response).is_ok());

    // Submit a query for execution on the server
    let response = client.run("RETURN 1 as num;", None, None).await?;

    // Successful responses will include a SUCCESS message with related metadata
    // Consuming these messages is optional and will be skipped for the rest of the example
    assert!(Success::try_from(response).is_ok());

    // Use PULL to retrieve results of the query, organized into RECORD messages
    // We get a (Vec<Record>, Message) returned from a PULL
    let pull_meta = Metadata::from_iter(vec![("n", 1)]);
    let (records, response) = client.pull(Some(pull_meta.clone())).await?;

    assert_eq!(records[0].fields(), &[Value::from(1)]);

    // Submit a more complex query with parameters
    let params = Params::from_iter(vec![("name", "Rust")]);
    client.run(
        "CREATE (:Client)-[:WRITTEN_IN]->(:Language {name: $name});",
        Some(params), None).await?;
    client.pull(Some(pull_meta.clone())).await?;

    // Grab a node from the database and convert it to a native type
    client.run("MATCH (rust:Language) RETURN rust;", None, None).await?;
    let (records, response) = client.pull(Some(pull_meta.clone())).await?;
    let node = Node::try_from(records[0].fields()[0].clone())?;

    // Access properties from returned values
    assert_eq!(node.labels(), &[String::from("Language")]);
    assert_eq!(node.properties(),
               &HashMap::from_iter(vec![(String::from("name"), Value::from("Rust"))]));

    // End the connection with the server
    client.goodbye().await?;

    Ok(())
}