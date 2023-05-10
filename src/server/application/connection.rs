use std::net::TcpStream;
use std::io::{self,Write,BufRead};

pub struct Connection{
    reader:io::BufReader<TcpStream>,
    writer:io::LineWriter<TcpStream>,
    connection:TcpStream
}
impl Connection{
    pub fn connect(connection:TcpStream) -> Result<Connection,std::io::Error>{
        let reader = io::BufReader::new(connection.try_clone()?);
        let writer = io::LineWriter::new(connection.try_clone()?);
        Ok(Connection{reader,writer,connection})
    }
    pub fn recieve_message(&mut self)->io::Result<String>{
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
    pub fn send_message(&mut self,message:String)->io::Result<String>{
        self.writer.write(message.as_bytes())?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
    pub fn drop(&mut self)->io::Result<()>{
        self.connection.shutdown(std::net::Shutdown::Both)
    }

} 