use sqlite::Connection;

pub fn _establish_db_connection() {
    
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

    //let client_query = "
    //   INSERT INTO client VALUES (
    //       '$argon2id$v=19$m=19456,t=2,p=1$ZIB6AlG40RKe1s52Ygan5w$Ut+EM978QdWuVWUicHxrPOhIB4/hzfZoc4SwL3o8zzg',
    //       'test.maunder.tech',
    //       'BIND'
    //       );
    //   ";

    db_connection.execute(table_query).unwrap();
    //db_connection.execute(client_query).unwrap();
    println!("instantiate_db\tDB queries have been run.");
}


pub fn validate_client(db_connection: &Connection, hash: &str) -> bool {
    let query = format!("SELECT 1 FROM client WHERE hash = '{}';", &hash);
    //let query = format!("SELECT * FROM client;");
    let mut result: bool = false;
    db_connection.iterate(query, |pairs| {
        //println!("validate_client\tIterator entered");
        for &(_name, _value) in pairs.iter() {
            //println!("validate_client\t{} = {}", name, value.unwrap());
            result = true;
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
    result
}
pub fn validate_exists(_db_connection: &Connection, _hash: &str, _client_IP: &str) -> bool {true}
pub fn validate_current(_db_connection: &Connection, hash: &str, client_IP: &str, current_time: &str) -> bool {
    println!("validate_current\tHash: {}", hash); 
    println!("validate_current\tClient IP: {}", client_IP); 
    println!("validate_current\tCurrent time (UNIX): {}", current_time); 
    false
}

pub fn add_client(db_connection: &Connection, hash: &str, a_record: &str, dns_provider: &str) {
    let add_client_query = format!("INSERT INTO client VALUES ('{}', '{}', '{}');", &hash, &a_record, &dns_provider);
    db_connection.execute(add_client_query).unwrap();
}
pub fn _remove_client(_db_connection: &Connection, _hash: &str) {}

pub fn _add_record() {}
pub fn _update_record() {}
pub fn _remove_record(_db_connection: &Connection, _hash: &str) {}
