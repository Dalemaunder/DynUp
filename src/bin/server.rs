#![allow(non_snake_case)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use serde_derive::Deserialize;
use regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;



// Parent struct. Required by Serde.
#[derive(Debug, Deserialize)]
struct Settings {
    server: Config,
}

// Struct for the config of the server itself.
// TODO: Add defaults
#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    listen_port: u16,
    data_store: String,
    users: String,
    lifetime: String
}

// Struct for the Database config. Used for the DB connection, whether it's local or remote.
#[derive(Debug, Deserialize)]
struct Database {
    address: String,
    port: u16,
    username: String,
    password: String,
    authenticate: bool,
}



fn read_config() -> Settings {
    let file_path = "./config/server_config.toml";
    
    // Load the config file into a variable.
    // TODO: Make the error handling here better.
    let contents = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", file_path);
            exit(1);
        }
    };


    // Parse the config contents into the relevant structs.
    // TODO: Make the error handling here better.
    let data: Settings = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {

            eprintln!("Unable to load data from `{}`", file_path);
            eprintln!("{:#?}", &contents);
            exit(1);
        }
    };
    data
}


// Skeleton
fn db_add_new_entry() {

}


// Skeleton
fn db_update_entry() {

}


// Skeleton
fn db_handle_entry (hash: &str, salt: &str, client_IP: &str, current_time: Duration) {
    // SQLite connection here

    // Query the records table for the hash
    // If it doesn't exist:
    db_add_new_entry();
    // If it does exist:
    db_update_entry();
}


fn handle_hello (client_auth: &str, client_IP: &str) {
    // Split the client_auth string into a named tuple. This was surprisingly annoying to do.
    let (hash, salt) = client_auth
        .split("/")                 // Split on the delimiting slash,
        .collect::<Vec<&str>>()     // Collect the halves into a Vector,
        .into_iter()                // Turn the Vector into an iterator,
        .collect_tuple()            // Turn the iterator into a tuple,
        .unwrap();                  // Unpack the tuple from the Option<(String,String)>.

    println!("{}", hash);
    println!("{}", salt);
    println!("{}", client_IP);
    
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("{:#?}", current_time);

    db_handle_entry(hash, salt, client_IP, current_time);

}


// Regexes for parsing requests inside handle_connection().
lazy_static! { static ref REQUEST_RE: regex::Regex = Regex::new(r"(.*)\s/(\S*)\s.*").unwrap(); }
lazy_static! { static ref CLIENT_ADDRESS_RE: regex::Regex = Regex::new(r".*:\s(.*):.*").unwrap(); }

fn handle_connection(mut stream: TcpStream) {
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
    let request = request_lines[0].as_str();
    // Capture 1 should be the HTTP method, capture 2 should be the URI.
    let request_components = REQUEST_RE.captures(&request).unwrap();


    let socket = request_lines[1].as_str();
    // The only capture should be the client's IP address.
    let socket_components = CLIENT_ADDRESS_RE.captures(&socket).unwrap();


    // Match the method type from the connection.
    match &request_components[1] {
        "PATCH" => {
            // TODO: Add error handling here.
            handle_hello(&request_components[2], &socket_components[1]);
            // Only respond with the 200 code after the hello handling has passed.
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        },
        "GET" => {
            // Respond with a 204 code for connection testing purposes.
            // URI is completely ignored.
            let response = "HTTP/1.1 204 Empty\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        },
        _ => {
            // Deny all other request methods.
            let response = "HTTP/1.1 405 No\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }


}


fn main() {
    // TODO: Add error handling here.
    let settings = read_config();

    // TODO: Add error handling here.
    let socket = format!("{}:{}", settings.server.address, settings.server.listen_port);
    let listener = TcpListener::bind(socket).unwrap();


    for stream in listener.incoming() {
        // TODO: Add error handling here.
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

