use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const S1_SERVER_CONNECTION: &str = "127.0.0.1:8000";
const S2_SERVER_CONNECTION: &str = "127.0.0.1:8001";
//karin
#[tokio::main]
async fn main() {
    //create a bind so it can listen to incoming connections, if successful, returns a listener
    let listener = TcpListener::bind(S2_SERVER_CONNECTION).await.unwrap();

    println!("S2 Server is listening to connections");

    //if the listener is successful, we then accept the incoming messages
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        println!("A connection is established from the client");
        tokio::spawn(async move {handle_connection(stream).await});
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //create a buffer to store the message from the stream and then convert it to a String so we can read it
    let mut buff = [0;1024];

    let len = stream.read(&mut buff).await.unwrap();

    let message = String::from_utf8_lossy(&buff[..len]);

    println!("Message from client: {}", message);

    //call server S1
    let s1_message = call_s1(message.to_string()).await;
    println!("Message processed from the client: {}", s1_message);

    //echo the message back

    let _ = stream.write_all(s1_message.as_bytes()).await;

    println!("Successfully sent back the message to the client");

}

async fn call_s1(message: String) -> String {

    println!("Connecting to S1 server: {}", S1_SERVER_CONNECTION);

    if let Ok(mut stream) = TcpStream::connect(S1_SERVER_CONNECTION).await {
        //if we have a stream result that is Ok
        println!("connected to S1 server {}: {}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        
        //write hello world message
        let _ = stream.write_all(message.as_bytes()).await;
        
        println!("Sent to S1: {}", message);

        //read the result
        let mut buffer = [0; 1024]; //storing the bytes into the buffer
        let len = stream.read(&mut buffer).await.unwrap(); //we have a read here so its an interaction with the socket, another interaction with the network so needs .await
        let message = String::from_utf8_lossy(&buffer[..len]);
        println!("received: {}", message);

        return message.to_string();
        
    }
    else {
        format!("error")
    }
}