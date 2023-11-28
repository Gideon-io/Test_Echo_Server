use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::io;

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:8001";
#[tokio::main]
async fn main() {
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    while let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        //if we have a stream result that is Ok
        println!("connected to echo server {}: {}", 
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port()
        );
        
        //write hello world message

        println!("Type what you would like to echo");

        let mut user_input = String::new();
        
        io::stdin().read_line(&mut user_input).unwrap();
        
        let _ = stream.write_all(user_input.as_bytes()).await; //write is an i/o command so it needs to be .await'ed
        //let _ = stream.flush(); flush isn't needed as write_all is used
        println!("sent: {}", user_input);

        //read the result
        let mut buffer = [0; 1024]; //storing the bytes into the buffer
        let len = stream.read(&mut buffer).await.unwrap(); //we have a read here so its an interaction with the socket, another interaction with the network so needs .await
        let message = String::from_utf8_lossy(&buffer[..len]);
        println!("received: {}", message);
        
    } 
}