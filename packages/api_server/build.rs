use std::path::PathBuf;
use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .out_dir(out_dir.clone())
        .build_client(false)
        .type_attribute("NananGrpc.Nanan", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .compile(&["./proto/nanan.proto"], &["./proto"])?;
    /*
    tonic_build::configure()
        .out_dir(out_dir.clone())
        .build_server(false)
        .type_attribute("comptgrpc.ComptableCreationInput", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableIdentification", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableCoordonnees", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableTelecommunication", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableImmatriculation", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableMonnaie", "#[derive(::async_graphql::InputObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableOutput", "#[derive(::async_graphql::SimpleObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.ComptableCreationOutput", "#[derive(::async_graphql::SimpleObject, ::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute("comptgrpc.Licence", "#[derive(::async_graphql::SimpleObject, ::serde::Serialize, ::serde::Deserialize)]")
        .compile(&["./grpc/comptable.proto"], &["./grpc"])?;

    tonic_build::configure()
        .out_dir(out_dir.clone())
        .build_server(false)
        .compile(&["./grpc/licence.proto"], &["./grpc"])?;
    */
    Ok(())
}