use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Waiting for client...");
    
    let (mut socket, _) = listener.accept().await.unwrap();
    println!("Client connected!");
    
    let mut buffer = vec![0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Cliente desconectado. Desligando!");
                return;
            }
            Ok(n) => {
                let input = String::from_utf8_lossy(&buffer[..n]).to_string();
                /*
                if !is_online {
                    tokio::spawn(async move{
                        start(input).await;
                    }); 
                }
                */
                println!("{input} recived! - returning pong!");
                socket.write_all(b"pong").await.unwrap();
            }
            Err(e) => {
                println!("Erro: {e}")
            }
        }
    }
}