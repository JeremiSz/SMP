use std::collections::HashMap;

type ErrorResponce<'a> = ((&'a str,&'a str),(&'a str,&'a str),(&'a str,&'a str));

pub const ATTR_CODE :&str = "code";
pub const ATTR_MEANING :&str = "meaning";
pub const ATTR_AUTHORS :&str = "authors";
pub const ATTR_TEXT :&str = "text";
const COMMAND :&str = "command";
pub const GENERIC_LOGIN_ERROR : ErrorResponce= (
    (ATTR_CODE,"1002"),
    (ATTR_MEANING,"Other login error"),
    (COMMAND,"login")
    );
pub const GENERIC_WRITE_ERROR : ErrorResponce = (
    (ATTR_CODE,"2002"),
    (ATTR_MEANING,"Other write error"),
    (COMMAND,"write")
    );
pub const GENERIC_READ_ERROR : ErrorResponce= (
    (ATTR_CODE,"3002"),
    (ATTR_MEANING,"Other read error"),
    (COMMAND,"read")
);
pub const GENERIC_LOGOUT_ERROR : ErrorResponce = (
    (ATTR_CODE,"4002"),
    (ATTR_MEANING,"Other logout error"),
    (COMMAND,"logout")
    );
const ENTRY_DELIMITER :&str = ",";
const VALUE_DELIMITER :&str = ":";

pub fn create_login(username:String,password:String)->String{
    format!("command:login,username:{},password:{}",username,password)
}
pub fn create_write(text:String)->String{
    format!("command:write,text:{}",text)
}
pub fn create_read ()->String{
    String::from("command:read")
}
pub fn create_logout()->String{
    String::from("command:logout")
}

pub fn parse_response(response:String)->HashMap<String,String>{
    let mut map = HashMap::new();
    let iter = response.split(&ENTRY_DELIMITER);
    for pair in iter{
        let mut pair_iter = pair.split(&VALUE_DELIMITER);
        let key = pair_iter.next().unwrap();
        let value = pair_iter.next().unwrap();
        map.insert(String::from(key),String::from(value));
    }
    map
}
pub fn extract_array(value:&String)->Vec<String>{
    value.split(&VALUE_DELIMITER).map(|s| s.to_string()).collect()
}