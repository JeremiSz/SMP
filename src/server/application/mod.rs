mod presentation;
mod message_store;
mod connection;
use std::net::{TcpListener,TcpStream};
use std::io;
use std::sync::{Mutex,Arc};

use self::message_store::MessageStore;

pub fn run(){
    let message_store = Arc::new(Mutex::new(message_store::MessageStore::new()));
    let socket = make_socket();
    if socket.is_err(){
        presentation::error(format!("Failed to bind socket: {:?}",socket.err().unwrap()));
        return;
    }
    let socket = socket.unwrap();
    presentation::log("waiting for connections".to_string());
    loop{
        let connection = socket.accept();
        if connection.is_err(){
            presentation::error(format!("Failed to accept connection: {:?}",connection.err().unwrap()));
            continue;
        }
        let connection = connection.unwrap();

    }
}

fn make_socket()->io::Result<TcpListener>{
    let port = get_input("Port to listen on".to_string(),"8080".to_string());
    let hostname = get_input("Hostname to listen (localhost)".to_string(), "localhost".to_string());
    TcpListener::bind(format!("{}:{}",hostname,port))
}

fn get_input(question:String,default:String)->String{
    let mut input = String::new();
    println!("{} (default: {})",question,default);
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.is_empty(){
        input = default;
    }
    input
}

fn socket_thread(connection:TcpStream,message_store:Arc<Mutex<MessageStore>>){
    let mut done :bool= false;
    let connection = connection::Connection::connect(connection);
    if connection.is_err(){
        presentation::error(format!("Failed to connect to client: {:?}",connection.err().unwrap()));
        return;
    }

    let mut connection = connection.unwrap();
    while !done{
        let request = connection.recieve_message();
        if request.is_err(){
            presentation::error(format!("Failed to recieve message: {:?}",request.err().unwrap()));
            continue;
        }

    }
}