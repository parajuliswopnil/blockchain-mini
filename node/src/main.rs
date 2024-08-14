//! here the code starts

use mempool::{self, get_n_records, get_total_record_count, insert_record, Mempool};
use rpc;
use tokio::sync::mpsc::{self};

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
    let join4 = tokio::spawn(async move {
        get_n_records(mempool).await;
    });

    handles.push(join1);
    handles.push(join2);
    handles.push(join3);
    handles.push(join4);

    for handle in handles {
        handle.await.unwrap()
    }
}
