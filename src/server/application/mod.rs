mod connection;
mod message_store;
mod presentation;
mod smt_helper;

use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use connection::Connection;

use self::message_store::MessageStore;

pub fn run() {
    let message_store = Arc::new(Mutex::new(message_store::MessageStore::new()));
    let socket = make_socket();
    if socket.is_err() {
        presentation::error(format!(
            "Failed to bind socket: {:?}",
            socket.err().unwrap()
        ));
        return;
    }
    let socket = socket.unwrap();
    presentation::log("waiting for connections".to_string());
    loop {
        let connection = socket.accept();
        if connection.is_err() {
            presentation::error(format!(
                "Failed to accept connection: {:?}",
                connection.err().unwrap()
            ));
            continue;
        }
        let connection = connection.unwrap();
        let new_message_store = message_store.clone();

        thread::spawn(move || {
            socket_thread(connection.0, new_message_store);
        });
    }
}

fn make_socket() -> io::Result<TcpListener> {
    let port = get_input("Port to listen on".to_string(), "1234".to_string());
    let hostname = get_input("Hostname to listen".to_string(), "localhost".to_string());
    TcpListener::bind(format!("{}:{}", hostname, port))
}

fn get_input(question: String, default: String) -> String {
    let mut input = String::new();
    println!("{} (default: {})", question, default);
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.is_empty() {
        input = default;
    }
    input
}

fn socket_thread(connection: TcpStream, message_store: Arc<Mutex<MessageStore>>) {
    let connection = Connection::connect(connection);
    if connection.is_err() {
        presentation::error(format!(
            "Failed to connect to client: {:?}",
            connection.err().unwrap()
        ));
        return;
    }

    let mut connection = connection.unwrap();
    let request = connection.recieve_message();
    if request.is_err() {
        presentation::error(format!(
            "Failed to recieve message: {:?}",
            request.err().unwrap()
        ));
        return;
    }
    let request = request.unwrap();
    let request = smt_helper::parse(request);
    let username = is_valid_login(&mut connection, request);
    if username.is_err() {
        presentation::error(format!(
            "Failed to validate login: {:?}",
            username.err().unwrap()
        ));
        return;
    }
    let username = username.unwrap();
    let mut done = username.is_none();
    let username = username.unwrap_or("".to_string());
    let result = connection.send_message(smt_helper::successful_login());
    if result.is_err() {
        presentation::error(format!(
            "Failed to send message: {:?}",
            result.err().unwrap()
        ));
        return;
    }

    while !done {
        let request = connection.recieve_message();
        if request.is_err() {
            presentation::error(format!(
                "Failed to recieve message: {:?}",
                request.err().unwrap()
            ));
            continue;
        }
        let request = smt_helper::parse(request.unwrap());
        match request.get(smt_helper::COMMAND).unwrap() as &str {
            smt_helper::COMMAND_READ => read(request, &mut connection, &message_store),
            smt_helper::COMMAND_WRITE => write(request, username.clone(),&mut connection, &message_store),
            smt_helper::COMMAND_LOGOUT => {
                let result = logout(request, &mut connection);
                if result.is_err() {
                    done = false;
                    io::Result::Err(result.err().unwrap())
                } else {
                    done = result.is_err();
                    io::Result::Ok(())
                }
            }
            _ => {
                let message = connection.send_message(smt_helper::get_error(1002));
                if message.is_err() {
                    io::Result::Err(message.err().unwrap())
                } else {
                    io::Result::Ok(())
                }
            }
        }
        .expect("Failed to process request");
    }
    presentation::log("closing connection".to_string());
}
fn is_valid_login(
    socket: &mut Connection,
    request: HashMap<String, String>,
) -> io::Result<Option<String>> {
    if !request.contains_key(smt_helper::COMMAND)
        || request.get(smt_helper::COMMAND).unwrap() != smt_helper::COMMAND_LOGIN
    {
        socket.send_message(smt_helper::get_error(1003))?;
        Ok(None)
    } else if !request.contains_key(smt_helper::LOGIN_USERNAME)
        || request.get(smt_helper::LOGIN_USERNAME).unwrap().is_empty()
    {
        socket.send_message(smt_helper::get_error(1004))?;
        Ok(None)
    } else {
        Ok(Some(
            request.get(smt_helper::LOGIN_USERNAME).unwrap().to_string(),
        ))
    }
}
fn read(
    request: HashMap<String, String>,
    connection: &mut Connection,
    message_store: &Arc<Mutex<MessageStore>>,
) -> io::Result<()> {
    if request.len() > 1 {
        connection.send_message(smt_helper::get_error(3003))?;
    }
    let store = message_store.lock().unwrap();
    let authors = store.get_authors();
    let texts = store.get_texts();
    connection.send_message(smt_helper::successful_read(authors, texts))?;
    Ok(())
}

fn write(
    request: HashMap<String, String>,
    username:String,
    connection: &mut Connection,
    message_store: &Arc<Mutex<MessageStore>>,
) -> io::Result<()> {
    if !request.contains_key(smt_helper::WRITE_TEXT)
        || request.get(smt_helper::WRITE_TEXT).unwrap().is_empty()
    {
        connection.send_message(smt_helper::get_error(2003))?;
    }
    let text = request.get(smt_helper::WRITE_TEXT).unwrap().to_string();
    message_store.lock().unwrap().add_message(username, text);
    connection.send_message(smt_helper::successful_write())?;
    Ok(())
}
fn logout(request: HashMap<String, String>, connection: &mut Connection) -> io::Result<bool> {
    if request.len() > 1 {
        connection.send_message(smt_helper::get_error(4003))?;
    }
    connection.send_message(smt_helper::successful_logout())?;
    Ok(true)
}
