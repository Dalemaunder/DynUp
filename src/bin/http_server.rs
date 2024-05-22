use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use base64::decode;
use lazy_static::lazy_static;
use std::net::TcpStream;
use std::io::{prelude::*,BufReader};
use sqlite::Connection;
use std::str;

//mod db_access;
use crate::db_access;

pub enum RequestStatus {
    Invalid,
    New,
    OutOfDate,
    Current,
}

pub fn parse_registration(db_connection: &Connection, client_registration: String, client_IP: String) -> RequestStatus {
    // Split the URI into its constituent parts: hash, a record, DNS provider name
    let registration_parts = client_registration.split("/").collect::<Vec<&str>>();
    // Decode each part from Base64
    let decoded_hash = decode(registration_parts[0]).unwrap();
    let decoded_a_record = decode(registration_parts[1]).unwrap();
    let decoded_provider = decode(registration_parts[2]).unwrap();
    // Reassemble each part from a byte array into a str
    let hash = str::from_utf8(&decoded_hash).unwrap();
    let a_record = str::from_utf8(&decoded_a_record).unwrap();
    let provider = str::from_utf8(&decoded_provider).unwrap();

    // TODO: Add error handling here
    db_access::add_client(&db_connection, &hash, &a_record, &provider);

    RequestStatus::New
}

pub fn parse_hello (db_connection: &Connection, client_auth: String, client_IP: String) -> RequestStatus {
    // The password hash arrives as a base64 encoded string.
    // Decode it and convert it back into a String.
    let decoded = decode(client_auth).unwrap();
    let hash = str::from_utf8(&decoded).unwrap();

    // Get the current UNIX time.
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    //db_access::handle_entry(hash, salt, client_IP, &current_time);

    if ! db_access::validate_client(&db_connection, &hash) {
        RequestStatus::Invalid
    } else if ! db_access::validate_exists(&db_connection, &hash, &client_IP) {
        RequestStatus::New
    } else if ! db_access::validate_current(&db_connection, &hash, &client_IP, &current_time) {
        RequestStatus::OutOfDate
    } else {
        RequestStatus::Current
    }
}

// Regexes for parsing requests inside handle_connection().
lazy_static! { static ref REQUEST_RE: regex::Regex = Regex::new(r"(.*)\s/(\S*)\s.*").unwrap(); }
lazy_static! { static ref CLIENT_ADDRESS_RE: regex::Regex = Regex::new(r".*:\s(.*):.*").unwrap(); }

pub fn parse_connection(mut stream: &TcpStream) -> (String, String, String) {
    let buf_reader = BufReader::new(&mut stream);

    // Load the contents of the request into a vec for easy access.
    // This entire function can definitely be optimised once the MVP is working.
    let mut request_lines = Vec::new();
    for line in buf_reader.lines() {
        let contents = line.unwrap();
        if contents.is_empty() {
            break;
        } else {
            request_lines.push(contents);
        }
    }

    println!("{:#?}", &request_lines);
    // Capture 1 should be the HTTP method, capture 2 should be the URI.
    let request_components = REQUEST_RE
        .captures(request_lines[0].as_str())
        .unwrap();

    // The only capture should be the client's IP address.
    let socket_components = CLIENT_ADDRESS_RE
        .captures(request_lines[1].as_str())
        .unwrap();

    (request_components[1].to_string(), request_components[2].to_string(), socket_components[1].to_string())
}
