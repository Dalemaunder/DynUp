use std::fmt;


pub enum ProviderType {
    BIND,
    CloudFlare,
}
impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProviderType::BIND => write!(f, "BIND"),
            ProviderType::CloudFlare => write!(f, "CloudFlare"),
        }
    }
}

impl ProviderType {
    pub fn update(&self, client_IP: &str, client_a_record: &str) {
        match *self {
            ProviderType::BIND => update_bind(client_IP, client_a_record),
            ProviderType::CloudFlare => update_cloudflare(client_IP, client_a_record),
        }
    }
}

fn update_bind(_client_IP: &str, _client_a_record: &str){
    println!("update_bind() has been run") 
    // TODO: BIND API calls to update records.
}

fn update_cloudflare(_client_IP: &str, _client_a_record: &str) {
    println!("update_cloudflare() has been run") 
    // TODO: CloudFlare API calls to update records.
}
