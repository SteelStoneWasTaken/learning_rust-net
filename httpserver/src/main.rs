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
            println!("[{}] {}: {}", Local::now(), ip.unwrap(), path.as_str());

            match fs::read(path.as_str()) {
                Ok(contents) => {
                    warp::reply::with_header(
                        contents,
                        "content-type", match path {
                            p if p.ends_with(".css") => "text/css",
                            p if p.ends_with(".js")  => "application/javascript",
                            _                        => "text/html",
                        }
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