//! this is to make the compiler happy
use core::time;
use std::{fmt::format, sync::Arc, thread::sleep};
mod utils;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, BufStream},
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};
pub use utils::parse_request;
pub use utils::Request;

/// Method that defines the type of http request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// Only read no write
    Get,
    /// Set something for the first time
    Set,
    /// Update something already set
    Update,
    /// Delete something already deleted
    Delete,
}

/// this is the request body
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Body {
    from: String,
    to: String,
    value: String,
    signature: String,
}

impl Body {
    /// casts the body in the form of json string to the Body struct
    pub fn parse_from_raw_body(body_json: String) -> Body {
        let body: Body = serde_json::from_str(&body_json).unwrap();
        return body;
    }
}

/// this is also to make compiler happy
pub async fn server(debug: bool, sender: tokio::sync::mpsc::Sender<Body>) {
    let arc_sender = Arc::new(sender);
    if debug {
        let mut count = 0u32;
        loop {
            let cloned_sender = arc_sender.clone();
            tokio::spawn(async move {
                dummy(count, cloned_sender).await;
            });
            count+=1;
            sleep(time::Duration::from_secs(2));
        }
    }

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let cloned_sender = arc_sender.clone();
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket, cloned_sender).await;
        });
    }
}

async fn dummy(count: u32, sender: Arc<Sender<Body>>) {
    let body = Body{
        from: format!("from"),
        to: format!("to"),
        value: format!("value"),
        signature: format!("signature {count}")
    };
    sender.send(body).await.unwrap();
}

/// this processes the request
pub async fn process(socket: TcpStream, sender: Arc<Sender<Body>>) {
    let mut stream = BufStream::new(socket);
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let request = parse_request(buffer);
    let body = Body::parse_from_raw_body(request.body);
    sender.send(body).await.unwrap();
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    // use super::*;
    #[test]
    fn test() {
        let json_test = json!({"hello": "world"});
        println!("{}", json_test["hello"])
    }
}
