## High Level Overview
Written in Rust,

Initially focused on Cisco IOS as that's where we're lacking DDNS,

Potentially add FortiOS once Cisco's working as FortiNet charge for it,

Initially talk to CloudFlare or local DNS (BIND?) for the updates (both?),

People have mentioned [RFC2136](https://datatracker.ietf.org/doc/html/rfc2136) is relevant for BIND.

### Core Components
#### Client
For when the router doesn't support custom DDNS servers. Sends a small authentication hello packet to the server,
#### Server
Receives the router's updates or the client's hello packet and keeps track of the public IP it was sent from. If the IP changes, update the respective DNS server's A record.

### Additional Components
#### TUI Frontend
A terminal-based frontend for managing the server. Will be used for easily modifying server settings, performing server maintenance (database cleanup, etc), and adding/removing client registrations.
Framework to be used: [Ratatui](https://docs.rs/ratatui/latest/ratatui/)
#### GUI Frontend
As above, but GUI based. Framework for creating the GUI is undetermined as of yet.


### Configuration Format
#### Options
* JSON
* TOML
* YAML

#### Considerations
##### YAML
* Crate for parsing available,
* Okay readability,
* Awkward syntax (personal opinion),
* Comments
##### JSON
* Crate for parsing available,
* Easily read,
* Sane syntax,
* No comments
##### TOML
* Crate for parsing available,
* Very easily read,
* Sane syntax,
* Comments

TOML will be used, primarily based on me groking the syntax, and native support for comments.



## Technical Details
### Client
The client will be a service running on a Linux box (docker at some point?) for when a native client isn't available or appropriate. It sends a hello packet with a hash of the password/secret every *x* seconds.
```toml
[client]
# How frequently the client will send updates
update_interval = 30
# Authentication password. The server matches the hash of this password to the A record that needs updating.
password = "Abcd1234!"

[server]
# IP address, hostname, or domain name to send updates to.
address = 127.0.0.1
# Change the port being sent to from the default.
port = 1234
```

#### Hashing algorithm:
I'm seeing mixed recommendations; A lot of people are recommending Argon2, but some rank scrypt above it. Apparently, sha256 shouldn't be used for passwords?
"_Latacora, 2018:_ In order of preference, use scrypt, argon2, bcrypt, and then if nothing else is available PBKDF2." - https://www.latacora.com/blog/2018/04/03/cryptographic-right-answers/

### Server
The server will need to receive and understand the hello packets from the client, or updates from a supported router's inbuilt utilities (Cisco IOS first).
It will need to read the IP address that sent the packet, match the hash to an A record stored in a local database, then (if the IP has changed) send an API request to update the respective A record with the new IP address.
It will also occasionally perform housekeeping routines that purge entries that haven't been seen for *x* time.

The server will need to persist the following data:
* Password hash
* IP address
* The A record to update
* Last time seen

#### Process
1. Match the received hash against the database,
2. Update the last time seen to *now*,
3. Check if the IP address has changed,
4. Fire off an update to the API

### Database
#### Database options
* [sqlite](https://docs.rs/sqlite/latest/sqlite/) - Basic SQL entries,
* [sled](https://sled.rs/) - Key value store; key = hash, value = array/tuple of respective values,
#### Database considerations
SQLite
* I'm familiar with SQL,
* Can contain both the users and records in separate tables,
Sled
* Very lightweight

### Configuration
```toml
[server]
# Configure the address the server monitors on.
address = "127.0.0.1"
# Set the port to something different.
listen_port = 1234
# Location to store the SQL database.
data_store = "/location/for/database"
# Location to find the list of pre-configured clients.
users = "/location/of/user/list"
# How long to keep entries before being purged.
lifetime = "1d"
```

### Networking
There are multiple network components to this project.
#### Client
The client needs to send the hello packets. It will not be receiving back any data initially. Response codes for client logs to be considered after an MVP is running.
Possible crate(s): reqwest, Ureq.

#### Server
The server needs to be able to receive data on a specific port.
The server needs to be able to send API queries to various other servers.
Possible crate(s): axum
Possible independant webserver(s): nginx

### Protocol(s)
The initial implementation will be done via a very basic REST API. Once a MVP is up and running it can be converted to using sockets (maybe with a custom protocol?). The communication can be broken out into a library so that it can be substituded in the code easily.

## Crates
Serialization/Deserialization: [Serde_derive](https://crates.io/crates/serde_derive)
TOML parsing: [toml](https://docs.rs/toml/latest/toml/)
TUI Framework: [Ratatui](https://docs.rs/ratatui/latest/ratatui/)
HTTP Client: [ureq](https://lib.rs/crates/ureq)
