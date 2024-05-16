#![allow(non_snake_case)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
};
use serde_derive::Deserialize;
use regex::Regex;


#[derive(Debug, Deserialize)]
struct Settings {
    server: Config,
}

#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    listen_port: u16,
    data_store: String,
    users: String,
    lifetime: String
}

#[derive(Debug, Deserialize)]
struct Database {
    address: String,
    port: u16,
    username: String,
    password: String
}

fn read_config() -> Settings {
    let file_path = "./config/server_config.toml";

    let contents = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", file_path);
            exit(1);
        }
    };

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


fn handle_connection(mut stream: TcpStream) {
    let _request_re = Regex::new(r".*\s/(\S*)\s.*").unwrap();
    let _client_address_re = Regex::new(r".*:\s(.*):.*").unwrap();

    let buf_reader = BufReader::new(&mut stream);

    let mut request_lines = Vec::new();
    for line in buf_reader.lines() {
        let contents = line.unwrap();
        if contents.is_empty() {
            break;
        } else {
            request_lines.push(contents);
        }
    }

    //for value in &request_lines {
    //    println!("{:#?}", value);
    //}

    //println!("{:#?}", &request_line);
    let request = request_lines[0].as_str();
    let client_address = request_lines[4].as_str();

    match request {
        "PATCH / HTTP/1.1" => {
            println!("{:#?}", request);
            println!("{:#?}", client_address);
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        },
        _ => {
            println!("{:#?}", request);
            println!("{:#?}", client_address);
            let response = "HTTP/1.1 405 No\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }


}


fn main() {
    let settings = read_config();
    let socket = format!("{}:{}", settings.server.address, settings.server.listen_port);
    //let socket = format!("{}:{}", "127.0.0.1", "7878");
    let listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

