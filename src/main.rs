#![allow(non_snake_case)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn read_config() -> (){
    ()
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

    let server_address = "127.0.0.1";
    let server_port = "7878";
    let socket = format!("{}:{}", server_address, server_port);

    let listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

