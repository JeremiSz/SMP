use std::collections::HashMap;

pub const ATTR_CODE: &str = "code";
pub const ATTR_MEANING: &str = "meaning";
pub const ATTR_AUTHORS: &str = "authors";
pub const ATTR_TEXT: &str = "text";
const ENTRY_DELIMITER: &str = ",";
const VALUE_DELIMITER: &str = ":";

pub fn create_login(username: String, password: String) -> String {
    format!(
        "command:login,username:{},password:{}\n",
        username, password
    )
}
pub fn create_write(text: String) -> String {
    format!("command:write,text:{}\n", text)
}
pub fn create_read() -> String {
    String::from("command:read\n")
}
pub fn create_logout() -> String {
    String::from("command:logout\n")
}

pub fn parse_response(response: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let iter = response.split(&ENTRY_DELIMITER);
    println!("response: {}", response);
    for pair in iter {
        println!("pair: {}", pair);
        let mut pair_iter = pair.split(&VALUE_DELIMITER);
        println!("pair_iter: {:?}", pair_iter);
        let key = pair_iter.next().unwrap();
        let value = pair_iter.next().unwrap();
        map.insert(String::from(key), String::from(value));
    }
    map
}
pub fn extract_array(value: &String) -> Vec<String> {
    value
        .split(&VALUE_DELIMITER)
        .map(|s| s.to_string())
        .collect()
}
