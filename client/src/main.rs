use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::time::{sleep, Duration};
use std::io;

#[tokio::main]
async fn main() {
    let mut stream: TcpStream;
    loop {
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(s) => {
                println!("Conected!");
                stream = s;
                break;
            },
            Err(_) => {
                println!("Retrying in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
            }
        };
    }
    
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("{INVALID_OP}");
        
        match input.trim() {
            "ping" => {
                stream.write_all(b"ping").await.unwrap();
                println!("ping sent!");
                
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).await.unwrap();
                println!("Returned: {}", String::from_utf8_lossy(&buffer[..n]));
            }
            _ => println!("CNF")
        }
    }
}
