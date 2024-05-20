use sqlite::Connection;

pub fn establish_db_connection() {}
pub fn instantiate_db() {}

pub fn validate_client(_db_connection: &Connection, _hash: &str) -> bool {
    // Add a SQL query here to check the clients table for the hash. If the hash doesn't exist,
    // return false
    true
}
pub fn validate_exists(_db_connection: &Connection, _hash: &str, client_IP: &str) -> bool {true}
pub fn validate_current(_db_connection: &Connection, hash: &str, client_IP: &str, current_time: &str) -> bool {
    println!("Hash: {}", hash); 
    println!("Client IP: {}", client_IP); 
    println!("Current time (UNIX): {}", current_time); 
    false
}

pub fn add_client(_db_connection: &Connection, hash: &str) {}
pub fn remove_client(_db_connection: &Connection, hash: &str) {}

pub fn add_record() {}
pub fn update_record() {}
pub fn remove_record(_db_connection: &Connection, hash: &str) {}
