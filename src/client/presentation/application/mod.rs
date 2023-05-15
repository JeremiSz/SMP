pub mod smt_helper;
mod connection;
use connection::Connection;
use std::collections::HashMap;
use std::io;

pub struct Application{
    connection:Connection
}

impl Application{
    pub fn new(hostname :String, portnum:u16)->Application{
        Application{
            connection:Connection::connect(hostname,portnum).unwrap()
        }
    }

    pub fn login(&mut self,name:String,password:String)->Result<HashMap<String,String>,io::Error>{
        println!("started login {} {}",name,password);
        let message = smt_helper::create_login(name,password);
        println!("made {}",message);
        let response = self.connection.send_message(message)?;
        println!("Got responces");
        let map = smt_helper::parse_response(response);
        println!("parsed responces");
        Ok(map)
    }
    pub fn close(&mut self)->Result<HashMap<String,String>,io::Error>{
        let message = smt_helper::create_logout();
        let response = self.connection.send_message(message)?;
        self.connection.drop()?;
        let map = smt_helper::parse_response(response);
        Ok(map)
    }
    pub fn write(&mut self,text:String)->Result<HashMap<String,String>,io::Error>{
        let text = text.replace(":","").replace(",","").replace("\n","");
        let message = smt_helper::create_write(text);
        let response = self.connection.send_message(message)?;
        let map = smt_helper::parse_response(response);
        Ok(map)
    }
    pub fn read(&mut self)->Result<HashMap<String,String>,io::Error>{
        let message = smt_helper::create_read();
        let response = self.connection.send_message(message)?;
        let map = smt_helper::parse_response(response);
        Ok(map)
    }
}