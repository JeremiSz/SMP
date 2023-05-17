use std::collections::HashMap;

pub const ATTR_CODE: &str = "code";
pub const ATTR_MEANING: &str = "meaning";
pub const ATTR_AUTHORS: &str = "authors";
pub const ATTR_TEXT: &str = "texts";
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
    for pair in iter {
        let mut pair_iter = pair.split(&VALUE_DELIMITER);
        let key = pair_iter.next().unwrap();
        let mut value = pair_iter.next().unwrap().to_string();
        let mut entry = pair_iter.next();
        while entry.is_some() {
            value.push_str(&VALUE_DELIMITER);
            value.push_str(entry.unwrap());
            entry = pair_iter.next();
        }
        let key = key.replace("\n", "");
        let value = value.replace("\n", "");
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
