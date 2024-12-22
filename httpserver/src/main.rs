use warp::Filter;
use chrono::Local;
use std::fs;

#[tokio::main]
async fn main() {
    warp::serve(
        /* */ warp::filters::addr::remote()
        .and( warp::path::full())

        .map(|ip: Option<std::net::SocketAddr>, path: warp::path::FullPath| {
            let path = format!("public{}", path.as_str());
            if let Some(ip) = ip {
                    println!("[{}] {}: {}", Local::now(), ip, path);
            }

            match fs::read(path.as_str()) {
                Ok(contents) => {
                    warp::reply::with_header(
                        contents,
                        "content-type",
                             if path.ends_with(".css") {"text/css"}
                        else if path.ends_with(".js") {"application/javascript"}
                        else {"text/html"}
                    )
                },
                Err(_) => warp::reply::with_header(
                    "<h1>404: Not found!</h1>".as_bytes().to_vec(),
                    "content-type",
                    "text/html"
                ),
            }
        }))
    .tls()
    .key_path("key.pem")
    .cert_path("cert.pem")
    .run(([0, 0, 0, 0], 3030))
    .await;
}