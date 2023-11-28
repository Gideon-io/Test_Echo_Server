use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:1234";
fn main() {
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        //if we have a stream result that is Ok
        println!("connected to echo server {}: {}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        //write hello world message
        let message = "hello world";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent: {}", message);

        //read the result
        let mut buffer = [0; 1024]; //storing the bytes into the buffer
        let len = stream.read(&mut buffer).unwrap(); //recieves in bytes
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);


    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}