use std::collections::HashMap;

const LOGIN_TEMPLATE :String= "command:login,username:{},password:{}";
const WRITE_TEMPLATE :String = "command:write,text:{}";
const READ_TEMPLATE :String = "command:read";
const LOGOUT_TEMPLATE :String = "command:logout";
pub const ATTR_CODE :String = "code";
pub const ATTR_MEANING :String = "meaning";
pub const ATTR_AUTHORS :String = "authors";
pub const ATTR_TEXT :String = "text";
const COMMAND :String = "command";
pub const GENERIC_LOGIN_ERROR : HashMap = HashMap::from([(ATTR_CODE,"1002"),(ATTR_MEANING,"Other login error"),(COMMAND,"login")]);
pub const GENERIC_WRITE_ERROR : HashMap = HashMap::from([(ATTR_CODE,"2002"),(ATTR_MEANING,"Other write error"),(COMMAND,"write")]);
pub const GENERIC_READ_ERROR : HashMap = HashMap::from([(ATTR_CODE,"3002"),(ATTR_MEANING,"Other read error"),(COMMAND,"read")]);
pub const GENERIC_LOGOUT_ERROR : HashMap = HashMap::from([(ATTR_CODE,"4002"),(ATTR_MEANING,"Other logout error"),(COMMAND,"logout")]);
const ENTRY_DELIMITER :String = ",";
const VALUE_DELIMITER :String = ":";

pub fn create_login(username:String,password:String)->String{
    format!(LOGIN_TEMPLATE,username,password)
}
pub fn create_write(text:String)->String{
    format!(WRITE_TEMPLATE,text)
}
pub fn create_read ()->String{
    READ_TEMPLATE
}
pub fn create_logout()->String{
    LOGOUT_TEMPLATE
}

pub fn parse_response(response:String)->HashMap{
    let mut map = HashMap::new();
    let mut iter = response.split(ENTRY_DELIMITER);
    for pair in iter{
        let mut pair_iter = pair.split(VALUE_DELIMITER);
        let key = pair_iter.next().unwrap();
        let value = pair_iter.next().unwrap();
        map.insert(key,value);
    }
    map
}
pub fn extract_array(value:String)->Vec<String>{
    value.split(VALUE_DELIMITER)
}