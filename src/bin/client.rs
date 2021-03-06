use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
  Get {
    key: String,
  },
  Set {
    key: String,
    val: Bytes,
  }
}

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);
  let tx2 = tx.clone();

  // The `move` keyword is used to **move** ownership of `rx` into the task.
  let manager = tokio::spawn(async move {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Start receiving messages
    while let Some(cmd) = rx.recv().await {
      use Command::*;

      match cmd {
        Get { key } => {
          client.get(&key).await;
        }
        Set { key, val } => {
          client.set(&key, val).await;
        }
      }
    }
  });


  // Spawn two tasks, one gets a key, the other sets a key
  let t1 = tokio::spawn(async move {
    use Command::Get;
    tx.send(Get{key: "ovo".to_string()}).await;
  });

  let t2 = tokio::spawn(async move {
    use Command::Set;
    tx2.send(Set{key:"ovo".to_string(),val: Bytes::from("PORRA")}).await;
  });

  manager.await.unwrap();
  t1.await.unwrap();
  t2.await.unwrap();
}