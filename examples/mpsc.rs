use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures::sink::SinkExt;

#[tokio::main]
async fn main() {
    // Create an unbounded mpsc channel
    let (mut tx, mut rx) = mpsc::unbounded::<i32>();

    // Send a message into the channel
    tx.send(1).await.unwrap();

    // Receive the message from the channel
    if let Some(msg) = rx.next().await {
        println!("Received: {}", msg);
    }
}