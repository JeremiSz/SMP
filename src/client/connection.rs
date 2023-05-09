use std::net::TcpStream;
use std::io;

pub struct Connection{
    reader:io::BufReader<TcpStream>,
    writer:io::LineWriter<TcpStream>
}
impl Connection{
    pub fn connect(hostname:String,portnum:u16) -> Result<self>{
        let addr = format!("{}:{}",hostname,portnum);
        let connection = TcpStream::connect(addr)?;
        let reader = io::BufReader::new(stream.try_clone()?);
        let writer = io::LineWriter::new(stream)?;
        Connection{reader,writer}
    }
    pub fn send_message(&mut self,message:String)->io::Result<()>{
        self.writer.write(message.as_bytes())?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
    pub fn drop(&mut self){
        self.writer.shutdown();
    }

} 