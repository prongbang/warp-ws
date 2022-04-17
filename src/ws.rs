use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use crate::model::{NEXT_USERID, Users};

pub async fn connect(ws: WebSocket, users: Users) {
    // Bookkeeping
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("Welcome User {}", my_id);

    // Establishing a connection
    let (user_tx, mut user_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(rx.forward(user_tx));
    users.write().await.insert(my_id, tx);

    // Reading and broadcasting messages
    while let Some(result) = user_rx.next().await {
        broadcast_msg(result.expect("Failed to fetch message"), &users).await;
    }

    // Disconnect
    disconnect(my_id, &users).await;
}

pub async fn broadcast_msg(msg: Message, users: &Users) {
    if let Ok(_) = msg.to_str() {
        for (&_uid, tx) in users.read().await.iter() {
            tx.send(Ok(msg.clone())).expect("Failed to send message");
        }
    }
}

pub async fn disconnect(my_id: usize, users: &Users) {
    println!("Good bye user {}", my_id);

    users.write().await.remove(&my_id);
}