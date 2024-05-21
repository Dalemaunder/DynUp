use sqlite::Connection;

pub fn establish_db_connection() {
    
}
pub fn instantiate_db(db_connection: &Connection) {
    let table_query = "
        CREATE TABLE client (
            hash text,
            a_record text,
            dns_provider text,
            PRIMARY KEY (hash)
       );
       
        CREATE TABLE record (
            hash text,
            last_checkin integer,
            address text,
            PRIMARY KEY (hash)
      );
    ";

    let client_query = "
        INSERT INTO client VALUES (
            '$argon2id$v=19$m=19456,t=2,p=1$ZIB6AlG40RKe1s52Ygan5w$Ut+EM978QdWuVWUicHxrPOhIB4/hzfZoc4SwL3o8zzg',
            'test.maunder.tech',
            'BIND'
            );
        ";

    db_connection.execute(table_query).unwrap();
    db_connection.execute(client_query).unwrap();
    println!("DB queries have been run.");
}

pub fn validate_client(db_connection: &Connection, hash: &str) -> bool {
    //let query = format!("SELECT 1 FROM client WHERE hash = '{}';", &hash);
    let query = format!("SELECT * FROM client;");
    db_connection.iterate(query, |pairs| {
        println!("Iterator entered");
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    })
    .unwrap();
    //println!("DB Result: {:#?}", result);
    //match result {
    //    "1" => {
    //        println!("Client found");
    //        true
    //    },
    //    _ => false
    //}
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
