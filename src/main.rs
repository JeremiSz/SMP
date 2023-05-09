mod client;
mod server;

use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-c" => client::run(),
        "-s" => server::run(),
        _ => println!("Please specify a mode: client or server"),
    }
}