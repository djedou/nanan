use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use nanan_web_api::router;


async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match router(req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::new(Body::from("Hello World")))
    }
    //Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    info!("Server: http://{}", "192.168.1.3:3007");
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([192, 168, 1, 3], 3007));
    let make_service = make_service_fn(|_conn| async move {
        Ok::<_, Infallible>(service_fn(handle))
    });
    
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}