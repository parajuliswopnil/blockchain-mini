//! this is to make the compiler happy
use std::sync::Arc;
mod utils;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, BufStream},
    net::{TcpListener, TcpStream},
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub async fn server(sender: tokio::sync::mpsc::Sender<String>) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let arc_sender = Arc::new(sender);

    loop {
        let cloned_sender = arc_sender.clone();
        let (socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            process(socket, cloned_sender).await;
        });
    }
}

/// this processes the request
pub async fn process(socket: TcpStream, sender: Arc<tokio::sync::mpsc::Sender<String>>) {
    let mut stream = BufStream::new(socket);
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let request = parse_request(buffer);
    let body = Body::parse_from_raw_body(request.body);
    sender.send(body.signature).await.unwrap();
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
