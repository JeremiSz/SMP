mod SMThelper;
use connection::Connection;
use std::collections::HashMap;

pub struct Applcation{
    connection:Connection
}

impl Applcation{
    pub fn new(hostname :String, portnum:u16)->self{
        Applcation{
            connection:Connection::connect(hostname,portnum)
        }
    }

    pub fn login(&mut self,name:String,password:String)->Result<HashMap>{
        let message = SMThelper::create_login(name,password);
        let response = self.connection.send_message(message)?;
        let map = SMThelper::parse_response(response);
        Ok(map)
    }
    pub fn close(&mut self)->Result<HashMap>{
        let message = SMThelper::create_logout();
        let response = self.connection.send_message(message)?;
        self.connection.drop();
        let map = SMThelper::parse_response(response);
        Ok(map)
    }
    pub fn write(&mut self,text:String)->Result<HashMap>{
        let text = text.replace(":","").replace(",","").replace("\n","");
        let message = SMThelper::create_write(text);
        let response = self.connection.send_message(message)?;
        let map = SMThelper::parse_response(response);
        Ok(map)
    }
    pub fn read(&mut self)->Result<HashMap>{
        let message = SMThelper::create_read();
        let response = self.connection.send_message(message)?;
        let map = SMThelper::parse_response(response);
        Ok(map)
    }
}