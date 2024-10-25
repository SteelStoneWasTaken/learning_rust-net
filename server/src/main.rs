use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8882").await.unwrap();
    println!("Waiting for client...");
    
    let (mut socket, _) = listener.accept().await.unwrap();
    println!("Client connected!");
    
    
    
    let mut content = Vec::new();
    let mut buffer  = vec![0; 1];
    loop {
       match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Cliente desconectado. Desligando!");
                return;
            }
            Ok(n) => {
                content.extend_from_slice(&buffer[..n]);
                
                if String::from_utf8_lossy(&content).chars().last() == Some(';') {
                    content.pop();
                    println!("{} recived! - returning pong!", String::from_utf8_lossy(&content));
                    socket.write_all(b"pong").await.unwrap();
                    
                    content.clear();
                }
            }
            Err(e) => {
                println!("Erro: {e}")
            }
        }
    }
}