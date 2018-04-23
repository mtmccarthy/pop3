extern crate pop3;

use pop3::pop3::pop3client::authenticate;
use pop3::pop3::pop3client::send_stat;

use std::io::prelude::*;
use std::net::TcpStream;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let user : &str = &*args[1];
    let pass: &strgit  =&* args[2];
    println!("Connecting to server");

    let mut stream = TcpStream::connect("pop.gmail.com:995").unwrap();
    println!("Connected to server");

    authenticate(&mut stream, user, pass);

    //send_stat(&mut stream)

}
