mod application;
use application::Application;
use std::cmp::min;

pub fn run(){
    let app = make_connection();
    if app.is_err(){
        println!("Error: {}",app.err().unwrap());
        return;
    }
    let mut app = app.unwrap();
    let mut input = String::new();
    loop{
        println!("Please enter a command:");
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_string();
        input.clear();
        match command.as_ref(){
            "logout" => {
                let result = logout();
                if result{break;}
            },
            "read" => {read();},
            "write" => {write();},
            _ => {
                println!("Invalid command!");
            }
        }
    }
}

fn logout()->bool{
    let result = app.close();
    if result.is_err(){
        println!("Error: {}",result.err().unwrap());
        return false;
    }
    let map = result.unwrap();
    let code = map.get(SMThelper::CODE).unwrap().parse::<u16>().unwrap();
    if code != 4001{
        println!("Error: {}",map.get(SMThelper::MESSAGE).unwrap());
        return false;
    }
    true
}

fn write(){
    let message = get_input("Please enter your message:".to_string(),"".to_string());
    if message.is_empty(){
        println!("Message cannot be empty!");
        return;
    }
    let result = app.write(message);
    if result.is_err(){
        println!("Error: {}",result.err().unwrap());
        return;
    }
    let map = result.unwrap();
    let code = map.get(SMThelper::CODE).unwrap().parse::<u16>().unwrap();
    if code != 2001{
        println!("Error: {}",map.get(SMThelper::MESSAGE).unwrap());
    }
}
fn read(){
    let result = app.write(message);
    if result.is_err(){
        println!("Error: {}",result.err().unwrap());
        return;
    }
    let map = result.unwrap();
    let code = map.get(SMThelper::CODE).unwrap().parse::<u16>().unwrap();
    if code != 3001{
        println!("Error: {}",map.get(SMThelper::MESSAGE).unwrap());
        return;
    }
    let authors = map.get(SMThelper::ATTR_AUTHORS);
    let texts = map.get(SMThelper::ATTR_TEXTS);
    if authors.is_none() || texts.is_none(){
        println!("Error: Invalid response!");
        return;
    }
    let authors = authors.unwrap();
    let texts = texts.unwrap();
    let size = min(authors.len(),texts.len());
    for i in 0..size{
        println!("{}:{}",authors[i],texts[i]);
    }
    
}

fn make_connection()->Result<Applcation>{
    let hostname = get_input("Please enter the hostname (default: localhost):".to_string(),"localhost".to_string());
    println!("Please enter the port number (default: 1234):");
    let portnum_str = get_input("Please enter the port number (default: 1234):".to_string(),"1234".to_string());
    let portnum = portnum_str.parse::<u16>().unwrap();
    let mut app = Application::new(hostname,portnum)?;
    
    let username = get_input("Please enter your username (default: user):".to_string(),"user".to_string());
    let password = get_input("Please enter your password (default: password):".to_string(),"".to_string());
    let result = app.login(username,password);

    if result.is_err(){
         Err(result.err().unwrap());
    }
    else{
        Ok(app)
    }
}

fn get_input(question:String,defualt:String){
    let mut line = String::new();
    println!("{}",question);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    if input.is_empty(){input = defualt;}
    input
}