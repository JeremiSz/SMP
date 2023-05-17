# SMT Protocol
Simple application for exchanging messages between users.

## Server
To run server, pass "-s" to executable. 

To run from source, use "cargo run -- -s".

The host address and port number have to then be provided as promted.

## Client
To run client, pass "-c" to executable. 

To run from source, use "cargo run -- -c".

The user will be promted for the address of the server, the port number, a user name and a password. 

### Commands
* Read - Downloads all messages from server and displays them.
* Write - Uploads message to server
* Logout - Disconnects client from server