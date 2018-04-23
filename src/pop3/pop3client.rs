use std::io::prelude::*;
use std::net::TcpStream;


pub fn authenticate<'a>(stream: &'a mut TcpStream, user: &str, pass: &str) {
    let user_command = "USER ".to_owned() + user;
    println!("{}", user_command);
    let buf = stream.write(user_command.as_bytes());


    match buf {
        Ok(buf) => println!("{} Succesfully sent command", buf),
        Err(msg) => println!("{}", msg)
    };

    stream.flush();

    for byte in stream.bytes(){
        match byte {
            Ok(byte) => println!("{}", byte),
            Err(_) => println!("Couldn't read byte")
        }
    }

    let pass_command = "PASS ".to_owned() + pass;
    println!("{}", pass_command);

    let buf2 = stream.write(pass_command.as_bytes());


    match buf2 {
        Ok(buf) => println!("{} Succesfully sent command", buf),
        Err(msg) => println!("{}", msg)
    };


}


pub fn send_stat(stream: &mut TcpStream) {

    stream.write(b"STAT");

    for byte in stream.bytes(){
        match byte {
            Ok(byte) => println!("{}", byte),
            Err(_) => println!("Couldn't read byte")
        }
    }

}