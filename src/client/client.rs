#![allow(non_snake_case)]
#![allow(deprecated)]
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2};
use std::process::exit;
use serde_derive::Deserialize;
use ureq::Response;
use std::{thread, time::Duration, str};
//use base64::{engine::general_purpose::URL_SAFE, write::EncoderWriter};
use base64::encode;


// Parent struct for the client and server settings
#[derive(Debug, Deserialize)]
struct Settings {
    client: Client,
    server: Server,
}

// Struct which holds the client-specific settings
#[derive(Debug, Deserialize)]
struct Client {
    update_interval: u64,
    password: String,
    a_record: String,
    DNS_provider: String,
}

// Struct which holds the server-specific settings
#[derive(Debug, Deserialize)]
struct Server {
    address: String,
    port: u16,
}



fn generate_hash(password: String) -> String{
    // Generate the hash here using Argon2id.
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    println!("{}", &hash);
    hash

    // Returning a dummy string along with the salt for testing.
    //("this-is-not-hashed-yet".to_string(), salt)
}

fn encode_string(input: String) -> String {
    let encoding = encode(input);
    encoding
}


fn read_config() -> Settings {
    let file_path = "./config/client_config.toml";

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

    // Generate the password hash
    let password_hash = generate_hash(settings.client.password);
    let encoded_hash = encode_string(password_hash);

    let encoded_a_record = encode_string(settings.client.a_record);
    let encoded_provider = encode_string(settings.client.DNS_provider);

    // Build a new duration out of the interval time from the config file
    // Used for the thread::sleep() in the core loop
    let interval = Duration::new(settings.client.update_interval, 0);

    // Form the socket using the paramaters from the config file
    // First assignment is for the registration packet.
    // NOTE: Swap the socket to pass all 3 parameters encoded together? Reduces number of hashing
    // processes required.
    let mut socket = format!("http://{}:{}/{}/{}/{}", settings.server.address, settings.server.port, encoded_hash, encoded_a_record, encoded_provider);

    // Send out the register packet and action the response.
    let register: Response = ureq::put(&socket)
        .call()?;

    match register.status() {
        201 => {
            println!("Client successfully added to the server");
        },
        _ => {
            println!("Client registration has failed");
            exit(1);
        }
    }

    // Re-assign the socket for hello packets.
    socket = format!("http://{}:{}/{}", settings.server.address, settings.server.port, encoded_hash);

    // Core loop that sends a new packet in increments specified in the config file.
    loop {
        // Fire off a patch request
        let hello: Response = ureq::patch(&socket)
            .call()?;
            //.into_string()?;

        // Debug print
        println!("{:#?}", hello);
        
        // Debug outputs currently. Convert to logging later.
        match hello.status() {
            200 => {
                println!("Request success");
            },
            _ => {
                println!("Request fail");
                break;
            },
        }    

        thread::sleep(interval);
    }

    Ok(())
}
