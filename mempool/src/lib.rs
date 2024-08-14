//! Mempool
use core::time;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};

use tokio::sync::mpsc::Receiver;
use rpc::Body;

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
    pub record: Vec<Body>,
}

impl Mempool {
    /// new mempool
    pub fn new() -> Mempool {
        Mempool {
            inner: Arc::new(Mutex::new(MempoolInner { record: Vec::new() })),
        }
    }

    /// insert record
    pub fn insert_record(&self, record: Body) {
        println!("inserting");
        self.inner.lock().unwrap().record.push(record);
    }

    /// get total record count
    pub fn get_total_record_count(&self) -> usize {
        self.inner.lock().unwrap().record.len()
    }

    /// get first n records
    pub fn get_n_records(&self) -> Vec<Body> {
        let mut total_records = self.get_total_record_count();
        let mut return_vector = Vec::new();
        if total_records > 5 {
            total_records = 5;
        }
        for i in 0..total_records {
            return_vector.push(self.inner.lock().unwrap().record[i].clone());
        }
        let replacement = self.inner.lock().unwrap().record.split_off(total_records);
        self.inner.lock().unwrap().record = replacement;
        return return_vector;
    }
}

/// insert record
pub async fn insert_record(mempool: Mempool, mut rx: Receiver<Body>) {
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
        println!("{}", record_count);
        sleep(time::Duration::from_secs(1));
    }
}

/// get first n records
pub async fn get_n_records(mempool: Mempool) {
    loop {
        sleep(time::Duration::from_secs(10));
        println!("get n records is: {:?}", mempool.get_n_records())
    }
}
