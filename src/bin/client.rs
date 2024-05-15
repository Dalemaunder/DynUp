#![allow(non_snake_case)]
use std::process::exit;
use serde_derive::Deserialize;
use ureq::Response;


// Parent struct for the client and server settings
#[derive(Debug, Deserialize)]
struct Settings {
    client: Client,
    server: Server,
}

// Struct which holds the client-specific settings
#[derive(Debug, Deserialize)]
struct Client {
    update_interval: u16,
    password: String,
}

// Struct which holds the server-specific settings
#[derive(Debug, Deserialize)]
struct Server {
    address: String,
    port: u16,
}

fn read_config() -> Settings {
    let file_path = "./client_config.toml";

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
    println!("Successfully read the config");
    data
}

fn main () -> Result<(), ureq::Error> {
    // Load the config file
    let settings = read_config();

    // Form the socket using the paramaters from the config file
    let socket = format!("http://{}:{}", settings.server.address, settings.server.port);
    
    // Fire off a head request
    let head: Response = ureq::head(&socket)
        .call()?;
        //.into_string()?;

    // Debug print
    println!("{:#?}", head);
    
    match head.status() {
        200 => {
            println!("Request success");
        },
        _ => {
            println!("Request fail");
        },
    }

    Ok(())
}
