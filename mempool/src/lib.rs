//! Mempool
use core::time;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};

use rpc::MemoryStorable;
use tokio::sync::mpsc::{Receiver, Sender};

/// Only the records of these record types can be stored in the mempool
#[derive(Debug)]
pub enum RecordType {
    /// transactions data
    Transactions,
}

/// Serialized record is sent to the receiver who calls for the stored data inside the mempool
/// because the Box<dyn MemoryStorable> does not implement Clone or Copy trait hence why cannot
/// be moved from the storage
#[derive(Debug)]
pub struct SerializedRecord {
    /// type of record stored
    pub record_type: RecordType,
    /// serialized record
    pub serialized_record: Vec<u8>,
}

/// Mempool implements this trait
pub trait MempoolTrait {
    /// inserts anything that is storable in mempool
    fn insert_record(&self, record: Box<dyn MemoryStorable>);
    /// gets the total number of records in the mempool
    fn get_total_record_count(&self) -> usize;
    /// gets and deletes the first n records in the mempool
    fn get_n_records(&self, n: usize) -> Vec<SerializedRecord>;
}

/// Mempool
#[derive(Debug, Clone)]
pub struct Mempool {
    /// keeps the record of all the txns
    pub inner: Arc<Mutex<MempoolInner>>,
}

/// MempoolInner
#[derive(Debug)]
pub struct MempoolInner {
    /// records all the transactions that comes to the node
    pub record: Vec<Box<dyn MemoryStorable>>,
}

impl Mempool {
    /// new mempool
    pub fn new() -> Mempool {
        Mempool {
            inner: Arc::new(Mutex::new(MempoolInner { record: Vec::new() })),
        }
    }
}
impl MempoolTrait for Mempool {
    fn insert_record(&self, record: Box<dyn MemoryStorable>) {
        self.inner.lock().unwrap().record.push(record);
    }

    fn get_total_record_count(&self) -> usize {
        self.inner.lock().unwrap().record.len()
    }

    fn get_n_records(&self, n: usize) -> Vec<SerializedRecord> {
        let mut total_records = self.get_total_record_count();
        let mut return_vector = Vec::new();
        if total_records > n {
            total_records = n;
        }
        for (i, v) in self.inner.lock().unwrap().record.iter().enumerate() {
            if i >= total_records {
                return return_vector;
            }
            return_vector.push(SerializedRecord {
                record_type: RecordType::Transactions,
                serialized_record: v.serialize_(),
            })
        }

        let replacement = self.inner.lock().unwrap().record.split_off(total_records);
        self.inner.lock().unwrap().record = replacement;
        return return_vector;
    }
}

/// insert record
pub async fn insert_record(mempool: Mempool, mut rx: Receiver<Box<dyn MemoryStorable>>) {
    loop {
        if let Some(message) = rx.recv().await {
            mempool.insert_record(message);
        }
    }
}

/// get total record count
pub async fn get_total_record_count(mempool: Mempool) {
    loop {
        let record_count = mempool.get_total_record_count();
        println!("# of records in mempool: {}", record_count);
        sleep(time::Duration::from_secs(1));
    }
}

/// get first n records
pub async fn get_n_records(n: u32, mempool: Mempool, tx: Sender<Vec<SerializedRecord>>) {
    let arc_tx = Arc::new(tx);
    loop {
        let cloned_arc_tx = arc_tx.clone();
        let records = mempool.get_n_records(n as usize);
        println!("records length {}", records.len());
        tokio::spawn(async move {
            cloned_arc_tx.send(records).await.unwrap();
        });
        // for record in records {
        //     let cloned_tx = cloned_arc_tx.clone();
        //     tokio::spawn(async move {
        //         cloned_tx.send(record).await.unwrap();
        //     });
        // }
        sleep(time::Duration::from_secs(10));
    }
}
