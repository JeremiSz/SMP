use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub struct Connection {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}
impl Connection {
    pub fn connect(connection: TcpStream) -> Result<Connection, std::io::Error> {
        let reader = io::BufReader::new(connection.try_clone()?);
        let writer = io::LineWriter::new(connection.try_clone()?);
        Ok(Connection { reader, writer })
    }
    pub fn recieve_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        println!("Recieved message: {}", &line);
        Ok(line)
    }
    pub fn send_message(&mut self, message: String) -> io::Result<()> {
        self.writer.write(message.as_bytes())?;
        Ok(())
    }
}
