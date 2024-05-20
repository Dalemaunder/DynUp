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
