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
        rpc::server(tx).await;
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

// async fn just_another_task(mut rx: Receiver<String>) {
//     for _ in 0..100 {
//         time::sleep(time::Duration::from_secs(1)).await;
//         let message = rx.recv().await;
//         match message {
//             Some(msg) => println!("The message is {}", msg),
//             None => continue,
//         }
//     }
// }
