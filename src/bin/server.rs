#![allow(non_snake_case)]
#![warn(dead_code)]

// Public Imports
use std::{
    io::prelude::*,
    net::TcpListener,
    process::exit,
};
use serde_derive::Deserialize;


// Module declaration/Private Imports
mod db_access;
mod dns_bindings;
mod http_server;
//use dns_bindings::ProviderType;
use http_server::RequestStatus;


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
    let settings: Settings = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {

            eprintln!("Unable to load data from `{}`", file_path);
            eprintln!("{:#?}", &contents);
            exit(1);
        }
    };
    settings
}


fn main() {
    // TODO: Add error handling here.
    let settings = read_config();

    // TODO: Add error handling here.
    let socket = format!("{}:{}", settings.server.address, settings.server.listen_port);
    let listener = TcpListener::bind(socket).unwrap();

    let db_connection = sqlite::open(":memory:").unwrap();

    db_access::instantiate_db(&db_connection);


    for stream in listener.incoming() {
        // TODO: Add error handling here.
        let mut stream = stream.unwrap();

        //(request_components, socket_components) = http_server::parse_connection(&stream);
        let (request_method, request_URI, client_IP) = http_server::parse_connection(&stream);

        println!("Method: {}", &request_method);
        println!("Method: {}", &request_URI);
        println!("Method: {}", &client_IP);
        // Match the method type from the connection.
        match request_method.as_str() {
            "PATCH" => {
                // TODO: Add error handling here.
                let request_status = http_server::parse_hello(&db_connection, request_URI, client_IP);
                let response; 
                match request_status {
                    RequestStatus::Invalid => {
                        response = "HTTP/1.1 401 Unauthorized\r\n\r\n";
                    },
                    RequestStatus::New => {
                        response = "HTTP/1.1 201 Created\r\n\r\n";
                    },
                    RequestStatus::OutOfDate => {
                        response = "HTTP/1.1 200 Updated\r\n\r\n";
                    },
                    RequestStatus::Current => {
                        response = "HTTP/1.1 200 OK\r\n\r\n";
                    },
                }
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
}
