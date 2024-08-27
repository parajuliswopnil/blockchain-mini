//! here the code starts

use core::time;
use std::{sync::Arc, thread::sleep};

use mempool::{
    self, get_n_records, get_total_record_count, insert_record, Mempool, SerializedRecord,
};
use rpc::{self, Body};
use tokio::sync::mpsc::{self, Receiver};
use vm::Executor;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let mempool = Mempool::new();
    let executor = Arc::new(Executor::new(10, 1000));
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

    let executor_cloned = executor.clone();
    let (record_tx, record_rx) = mpsc::channel(32);
    let join4 = tokio::spawn(async move {
        get_n_records(executor_cloned.get_tx_limit_per_block(), mempool, record_tx).await;
    });

    let executor_cloned = executor.clone();
    let join5 = tokio::spawn(async move {
        receive_messages(record_rx, executor_cloned).await;
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

async fn receive_messages(mut rx: Receiver<Vec<SerializedRecord>>, executor: Arc<Executor>) {
    _ = executor;
    loop {
        // let executor = executor.clone();
        let message = rx.recv().await;
        match message {
            Some(msg) => {
                for m in msg {
                    let unserialized_message: Body =
                        serde_json::from_slice(&m.serialized_record).unwrap();
                    println!(
                        "Received from mempool:: {:?}",
                        unserialized_message.signature
                    );
                }
            }
            None => {
                println!("none");
                sleep(time::Duration::from_secs(1))
            }
        }
    }
}
