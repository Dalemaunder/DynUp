# DynUp
A self-hosted Dynamic DNS server with an accompanying client.

## ToDo:
- [ ] MVP of the server
	- [ ] Can read the config file
    - [x] Can receive updates from a client
    - [ ] Can write updates to a local database
    - [ ] Can send API queries to a DNS server to update an A record
       - [ ] BIND

- [ ] MVP of the client
    - [ ] Can read the config file
    - [ ] Can send updates to a server

- [ ] Add features to server
    - [x] Can send basic response codes to client
        - [x] 200 OK
        - [x] 405 No
    - [ ] Can send advanced response codes to client
    - [ ] Can perform database cleanups based on the configured lifetime setting
    - [ ] Can receive updates from other routers
        - [ ] Cisco IOS
    - [ ] Can talk to additional DNS providers
        - [ ] Cloudflare
        - [ ] \[...\]

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
ToDo

### API
#### BIND
ToDo
#### Cloudflare
ToDo
