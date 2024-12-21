use warp::Filter;
use std::fs;

#[tokio::main]
async fn main() {
    let ping = warp::filters::addr::remote()
        .and(warp::path::full())
        .map(|ip: Option<std::net::SocketAddr>, path: warp::path::FullPath| {
            
            let path = format!("public{}", path.as_str());
            println!("User connected: {}\nRequested path: {}\n--------------",
            ip.unwrap(), path.as_str());
            
            match fs::read_to_string(path.as_str()) {
                Ok(html) => warp::reply::html(html),
                Err(_) => warp::reply::html("<h1>Error loading the page</h1>".to_string()),
            }
        });

    warp::serve(ping)
        .tls()
        .cert_path("cert.pem")
        .key_path("key.pem")
        .run(([0, 0, 0, 0], 3030))
        .await;
}
