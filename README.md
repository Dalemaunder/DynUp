# DynUp
A self-hosted Dynamic DNS server with an accompanying client.

## ToDo:
- [ ] Install Database
    - [ ] Install SQLite
    - [ ] Create credentials
    - [ ] Create the hosts table
    - [ ] Create the users table

- [ ] MVP of the server
	- [x] Can read the config file
    - [x] Can receive updates from a client
    - [x] Can parse the updates
    - [ ] Can write updates to a local database
        - [ ] SQLite
    - [ ] Can send API queries to a DNS server to update an A record
       - [ ] BIND

- [ ] MVP of the client
    - [x] Can read the config file
    - [x] Can send updates to a server
    - [ ] Can pass data to the server
        - [ ] Hostname
        - [ ] Hashed auth string

- [ ] Add features to server
    - [x] Can send basic response codes to client
        - [x] 200 OK
        - [x] 
        - [x] 405 No
    - [ ] Can send advanced response codes to client
    - [ ] Can perform database cleanups based on the configured lifetime setting
    - [ ] Can validate hello requests
    - [ ] Can receive updates from other routers
        - [ ] Cisco IOS
    - [ ] Can talk to additional DNS providers
        - [ ] Cloudflare
        - [ ] \[...\]
    - [ ] Dockerize the server
    - [ ] Add more Database options
        - [ ] \[...\]
    - [ ] Can automatically install and configure the database
        - [ ] Install
        - [ ] Create credentials
        - [ ] Create tables

- [ ] Add user documentation
    - [ ] Installation
    - [ ] Configuration
        - [ ] Basic configuration
        - [ ] API configuration
            - [ ] BIND
            - [ ] Cloudflare
            - [ ] \[...\]

- [ ] Add features to client
    - [ ] Basic logs
    - [ ] Advanced logs
    - [ ] Can authenticate hello packets with the server

- [ ] MVP of the TUI frontend
    - [ ] Can update the config file
    - [ ] Can update the client list
    - [ ] Can manually trigger a DB cleanup

- [ ] MVP of the GUI frontend
    - [ ] Can update the config file
    - [ ] Can update the client list
    - [ ] Can manually trigger a DB cleanup

## Installation
ToDo

## Configuration
### Client
ToDo

### Server
#### Linux
ToDo
#### Docker
ToDo

### API
#### BIND
ToDo
#### Cloudflare
ToDo
