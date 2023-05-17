mod application;
use application::smt_helper;
use application::Application;
use std::cmp::min;
use std::io;

pub fn run() {
    let app = make_connection();
    if app.is_err() {
        log_error(app.err().unwrap());
        return;
    }
    let mut app = app.unwrap();
    let mut input = String::new();
    loop {
        log("Commands\nread\nwrite\nlogout\nPlease enter a command:");
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
        input.clear();
        match command.as_ref() {
            "logout" => {
                let result = logout(&mut app);
                if result {
                    break;
                }
            }
            "read" => {
                read(&mut app);
            }
            "write" => {
                write(&mut app);
            }
            _ => {
                log("Invalid command!");
            }
        }
    }
}

fn logout(app: &mut Application) -> bool {
    let result = app.close();
    if result.is_err() {
        log_error(result.err().unwrap());
        return false;
    }
    let map = result.unwrap();
    let code = map
        .get(smt_helper::ATTR_CODE)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    if code != 4001 {
        log(map.get(smt_helper::ATTR_MEANING).unwrap());
        return false;
    }
    true
}

fn write(app: &mut Application) {
    let message = get_input("Please enter your message:".to_string(), "".to_string());
    if message.is_empty() {
        log("Message cannot be empty!");
        return;
    }
    let result = app.write(message);
    if result.is_err() {
        log_error(result.err().unwrap());
        return;
    }
    let map = result.unwrap();
    let code = map
        .get(smt_helper::ATTR_CODE)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    if code != 2001 {
        log(map.get(smt_helper::ATTR_MEANING).unwrap());
    }
}
fn read(app: &mut Application) {
    let result = app.read();
    if result.is_err() {
        log_error(result.err().unwrap());
        return;
    }
    let map = result.unwrap();
    let code = map
        .get(smt_helper::ATTR_CODE)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    if code != 3001 {
        log(map.get(smt_helper::ATTR_MEANING).unwrap());
        return;
    }
    let authors = map.get(smt_helper::ATTR_AUTHORS);
    let texts = map.get(smt_helper::ATTR_TEXT);
    if authors.is_none() || texts.is_none() {
        log("Nothing returned!");
        return;
    }
    let authors = smt_helper::extract_array(authors.unwrap());
    let texts = smt_helper::extract_array(texts.unwrap());
    let size = min(authors.len(), texts.len());
    for i in 0..size {
        log(&format!("{}:{}",authors[i], texts[i]));
    }
}

fn make_connection() -> Result<Application, io::Error> {
    let hostname = get_input(
        "Please enter the hostname".to_string(),
        "localhost".to_string(),
    );
    let portnum_str = get_input(
        "Please enter the port number".to_string(),
        "1234".to_string(),
    );
    let portnum = portnum_str.parse::<u16>().unwrap();
    let mut app = Application::new(hostname, portnum);

    let username = get_input("Please enter your username".to_string(), "user".to_string());
    let password = get_input(
        "Please enter your password".to_string(),
        "password".to_string(),
    );
    let result = app.login(username, password);

    if result.is_err() {
        Err(result.err().unwrap())
    } else {
        Ok(app)
    }
}

fn get_input(question: String, defualt: String) -> String {
    let mut line = String::new();
    log(&format!("{} (defualt: {}): ", question, defualt));
    std::io::stdin().read_line(&mut line).unwrap();
    let mut input = line.trim().to_string();
    if input.is_empty() {
        input = defualt;
    }
    input
}

fn log(message: &str) {
    println!("{}", message);
}
fn log_error(message:io::Error) {
    println!("Error: {}", message);
}
