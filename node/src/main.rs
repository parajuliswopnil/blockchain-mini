//! here the code starts

use core::time;
use std::thread::sleep;

use mempool::{
    self, get_n_records, get_total_record_count, insert_record, Mempool, SerializedRecord,
};
use rpc::{self, Body};
use tokio::sync::mpsc::{self, Receiver};

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let mempool = Mempool::new();

    let (tx, rx) = mpsc::channel(32);
    let join1 = tokio::spawn(async {
        rpc::server(true, tx).await;
    });
    let mempool1 = mempool.clone();
    let join2 = tokio::spawn(async move {
        insert_record(mempool1, rx).await;
    });

    let mempool2 = mempool.clone();
    let join3 = tokio::spawn(async move {
        get_total_record_count(mempool2).await;
    });
    let mempool = mempool.clone();

    let (record_tx, record_rx) = mpsc::channel(32);
    let join4 = tokio::spawn(async move {
        get_n_records(mempool, record_tx).await;
    });

    let join5 = tokio::spawn(async move {
        receive_messages(record_rx).await;
    });

    handles.push(join1);
    handles.push(join2);
    handles.push(join3);
    handles.push(join4);
    handles.push(join5);

    for handle in handles {
        handle.await.unwrap()
    }
}

async fn receive_messages(mut rx: Receiver<SerializedRecord>) {
    loop {
        let message = rx.recv().await;
        match message {
            Some(msg) => {
                let unserialized_message: Body = serde_json::from_slice(&msg.serialized_record).unwrap();
                println!("Received from mempool:: message signature: {:?}", unserialized_message.signature);
            }
            None => sleep(time::Duration::from_secs(1)),
        }
    }
}
