#![allow(non_snake_case)]
use std::net::TcpListener;

fn main() {

    let server_address = "127.0.0.1";
    let server_port = "7878";
    let socket = format!("{}:{}", server_address, server_port);

    let listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}

