use std::collections::HashMap;

pub const COMMAND :&'static str = "command";
pub const LOGIN_USERNAME:&'static str = "username";
pub const WRITE_TEXT:&'static str = "text";
pub const COMMAND_LOGIN:&'static str = "login";
pub const COMMAND_READ:&'static str = "read";
pub const COMMAND_WRITE:&'static str = "write";
pub const COMMAND_LOGOUT:&'static str = "logout";

pub fn parse(response:String)->HashMap<String,String>{
    let mut map = HashMap::new();
    let iter = response.split(",");
    for pair in iter{
        let mut pair_iter = pair.split(":");
        let key = pair_iter.next().unwrap();
        let value = pair_iter.next().unwrap();
        map.insert(String::from(key),String::from(value));
    }
    map
}

pub fn get_error(code:u16)->String{
    match code{
        1002 => make_error(COMMAND_LOGIN,code,"Other login error"),
        1003 => make_error(COMMAND_LOGIN,code,"Login not first command"),
        1004 => make_error(COMMAND_LOGIN, code, "Unkown username"),
        1005 => make_error(COMMAND_LOGIN, code, "Invalid password"),
        2002 => make_error(COMMAND_WRITE, code, "Other write error"),
        2003 => make_error(COMMAND_WRITE,code,"Missing text attribute"),
        3002 => make_error(COMMAND_READ, code, "Other read error"),
        3003 => make_error(COMMAND_READ, code, "Invalid read attribute"),
        4002 => make_error(COMMAND_LOGOUT, code, "Other logout error"),
        4003 => make_error(COMMAND_LOGOUT, code, "Had invalid attributes"),
        _ => make_error(COMMAND, code, "unkown error")
    }
}

pub fn successful_write()->String{
    format!("command:{},code:{},meaning:{}",COMMAND_WRITE,2001,"Message recieved successfully")
}
pub fn successful_read(authors:&Vec<String>,texts:&Vec<String>)->String{
    let mut authors_string = String::new();
    let mut texts_string = String::new();
    for author in authors{
        authors_string.push_str(author);
        authors_string.push_str(":");
    }
    for text in texts{
        texts_string.push_str(text);
        texts_string.push_str(":");
    }
    format!("command:{},code:3001,authors:{},texts:{}",COMMAND_READ,authors_string,texts_string)
}

fn make_error(command:&str,code:u16,meaning:&str)->String{
    format!("command:{},code:{},meaning:{}",command,code,meaning)
}