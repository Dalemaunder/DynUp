#![allow(non_snake_case)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
};
use serde_derive::Deserialize;


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


fn read_config() -> Settings {
    let file_path = "./server_config.toml";

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
    let buf_reader = BufReader::new(&mut stream);
    //let http_request: Vec<_> = buf_reader
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .take_while(|line| !line.is_empty())
    //    .collect();
    //
    //println!("Request: {:#?}", http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{:#?}", &request_line);

    match request_line.as_str() {
        "HEAD / HTTP/1.1" => {
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        },
        _ => {
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

