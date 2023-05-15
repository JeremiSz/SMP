use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub struct Connection {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
    connection: TcpStream,
}
impl Connection {
    pub fn connect(hostname: String, portnum: u16) -> Result<Connection, std::io::Error> {
        let addr = format!("{}:{}", hostname, portnum);
        let connection = TcpStream::connect(addr)?;
        let reader = io::BufReader::new(connection.try_clone()?);
        let writer = io::LineWriter::new(connection.try_clone()?);
        Ok(Connection {
            reader,
            writer,
            connection,
        })
    }
    pub fn send_message(&mut self, message: String) -> io::Result<String> {
        self.writer.write(message.as_bytes())?;
        self.writer.flush()?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
    pub fn drop(&mut self) -> io::Result<()> {
        self.connection.shutdown(std::net::Shutdown::Both)
    }
}
