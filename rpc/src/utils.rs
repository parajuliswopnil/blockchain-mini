use core::str;
use std::collections::HashMap;

/// this is request struct
#[derive(Debug)]
pub struct Request {
    /// Information about method name, request url and protocol
    pub request_type: Type,
    /// headers
    pub headers: HashMap<String, String>,
    /// content
    pub content: String,
    /// user agent
    pub user_agent: String,
    /// accept
    pub accept: String,
    /// tokens
    pub token: String,
    /// hosted on
    pub host: String,
    /// accepted encoding
    pub accept_encoding: Vec<String>,
    /// connection
    pub connection: String,
    /// content length
    pub content_length: u32,
    /// body
    pub body: String,
}

#[derive(Debug)]
pub struct Type {
    pub method: String,
    pub path: String,
    pub protocol: String,
}

/// to parse the request
pub fn parse_request(line_buffer: [u8; 1024]) -> Request {
    let stringified_request = str::from_utf8(&line_buffer).unwrap();
    let splitted_request: Vec<&str> = stringified_request.split("\n").collect();
    let mut request = Request {
        request_type: Type {
            method: "".to_string(),
            path: "".to_string(),
            protocol: "".to_string(),
        },
        headers: HashMap::new(),
        content: "".to_string(),
        user_agent: "".to_string(),
        accept: "".to_string(),
        token: "".to_string(),
        host: "".to_string(),
        accept_encoding: Vec::new(),
        connection: "".to_string(),
        content_length: 0,
        body: "".to_string(),
    };

    let request_string: Vec<&str> = splitted_request[0].split(" ").collect();
    request.request_type.method = request_string[0].to_string();
    request.request_type.path = request_string[1].to_string();
    request.request_type.protocol = request_string[2].to_string().replace("\r", "");

    let mut body_found = false;
    let mut body = String::new();
    for (_, v) in splitted_request.iter().enumerate() {
        if *v == "{" {
            body.push_str(*v);
            body_found = true;
            continue;
        }
        if body_found && *v != "\0" {
            body.push_str(*v);
        }
    }
    let body = body.replace(" ", "");
    let body = body.replace("\0", "");
    request.body = body;
    return request;
}
