use api_server::{
    NananGrpcServer, nanan_grpc::nanan_server::NananServer
};
use tonic::transport::{Server};
use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let server = NananGrpcServer::new();

    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    info!("Server: http://{}", "192.168.1.3:3007");

    Server::builder()
        //.tls_config(tls)?
        .add_service(NananServer::new(server))
        .serve("192.168.1.3:3007".parse()?)
        .await?;

    Ok(())
}