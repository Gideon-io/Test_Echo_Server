use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};



const S1_SERVER_CONNECTION: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    //create a bind so it can listen to incoming connections, if successful, returns a listener
    let listener = TcpListener::bind(S1_SERVER_CONNECTION).await.unwrap();

    println!("Server is listening to connections");

    //if the listener is successful, we then accept the incoming messages
    loop {
        let (stream, _) = listener.accept().await.unwrap();

        println!("A connection established from S2");

        tokio::spawn(async move {handle_connection(stream).await});
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //create a buffer to store the message from the stream and then convert it to a String so we can read it
    let mut buff = [0;1024];

    let len = stream.read(&mut buff).await.unwrap();

    let message = String::from_utf8_lossy(&buff[..len]);

    println!("Message from s2: {}", message);

    //echo the message back

    let _ = stream.write_all(message.as_bytes()).await;

    println!("Successfully sent back the message to the S2");

    
}