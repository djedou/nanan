pub mod services;
pub mod nanan_grpc {
    tonic::include_proto!("/nanan_grpc");
}
use nanan_grpc::{ DockerImage, ContainerToStart, ConnectNetwork, ContainerConfig, CreateNetworkOptions as DocNetOptions, Successed, nanan_server::Nanan};
use acteur::{Acteur};
use tonic::{
    Request, Response, Status
};
use tracing::{info};
use bollard::{
    Docker,
    image::CreateImageOptions,
    network::{CreateNetworkOptions, ConnectNetworkOptions},
    models::{Ipam, IpamConfig, EndpointIpamConfig, EndpointSettings, HostConfig, PortBinding},
    container::{Config, CreateContainerOptions, StartContainerOptions}
};
use std::collections::HashMap;
//use futures_util::stream::select;
//use futures_util::stream::StreamExt;
use futures_util::stream::TryStreamExt;




type ServerResult<T> = Result<Response<T>, Status>;



pub struct NananGrpcServer {
    pub acteur_sys: Acteur,
    pub docker: Docker
}


impl NananGrpcServer {
    pub fn new() -> NananGrpcServer {
        let acteur_sys: Acteur = Acteur::new();
        let Ok(docker) = Docker::connect_with_socket_defaults() else {
            panic!("Unable to connect to Docker Deamon!");
        };
        
        NananGrpcServer {
            acteur_sys,
            docker
        }
    }
}


#[tonic::async_trait]
impl Nanan for NananGrpcServer {
    async fn create_docker_image(&self, docker_image: Request<DockerImage>) -> ServerResult<Successed> {
        //info!("{:?}", docker_image);
        match &self.docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: docker_image.into_inner().name,
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await {
                Ok(_) => Ok(Response::new(Successed {
                    successed: true
                })),
                Err(e) => {
                    println!("Error! {:#?}", e);
                    Ok(Response::new(Successed {
                    successed: false
                })) }
            }
    }

    async fn create_docker_network(&self, docker_network: Request<DocNetOptions>) -> ServerResult<Successed> {
        //info!("{:#?}", docker_network);
        let network = docker_network.into_inner();

        let config = CreateNetworkOptions {
            name: network.name,
            check_duplicate: true,
            driver: network.driver.unwrap(),
            ipam: Ipam {
                config: Some(vec![
                    IpamConfig {
                        subnet: network.ipam.clone().unwrap().config[0].subnet.clone(),
                        gateway: network.ipam.clone().unwrap().config[0].gateway.clone(),
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            },
            ..Default::default()
        };

        match &self.docker
            .create_network(config)
            .await {
                Ok(_) => Ok(Response::new(Successed {
                    successed: true
                })),
                Err(e) => {
                    println!("Error! {:#?}", e);
                    Ok(Response::new(Successed {
                    successed: false
                })) }
            }
    }

    async fn create_docker_container(&self, docker_container: Request<ContainerConfig>) -> ServerResult<Successed> {
        let container = docker_container.into_inner();
        let mut hash: HashMap<String, HashMap<(), ()>> = HashMap::with_capacity(0);
        container.exposed_ports.iter().for_each(|p| {
            hash.insert(p.clone(), HashMap::with_capacity(0));
        });

        let mut port_bindings = HashMap::with_capacity(0);
        if let Some(host_config) = container.host_config {
            host_config.port_bindings.iter().for_each(|p| {
                let bings: Vec<_> = p.external_bindings.iter().map(|i| PortBinding {
                    host_ip: Some(i.host_ip.clone()),
                    host_port: Some(i.host_port.clone()),
                })
                .collect();
    
                port_bindings.insert(
                    p.internal_port.clone(), 
                    Some(bings));
            });
        }

        let config = Config {
            image: Some(container.image),
            cmd:  if !container.cmd.is_empty() {Some(container.cmd)} else { None }, //Some(vec!["/etc/confluent/docker/run"]),
            env: if !container.env.is_empty() {Some(container.env)} else { None },
            exposed_ports: if !container.exposed_ports.is_empty() {Some(hash)} else { None },
            host_config: Some(HostConfig {
                port_bindings: Some(port_bindings),
                ..Default::default()
            }),
            ..Default::default()
        };

        match &self.docker
            .create_container(
            Some(CreateContainerOptions {
                name: container.container_name,
                platform: None,
            }),
            config,
        )
            .await {
                Ok(_) => Ok(Response::new(Successed {
                    successed: true
                })),
                Err(e) => {
                    println!("Error! {:#?}", e);
                    Ok(Response::new(Successed {
                        successed: false
                    })) 
                }
            }
    }

    async fn connect_container_to_network(&self, docker_container_net: Request<ConnectNetwork>) -> ServerResult<Successed> {
        let network = docker_container_net.into_inner();

        let config = ConnectNetworkOptions {
            container: network.container_id,
            endpoint_config: EndpointSettings {
                ipam_config: Some(EndpointIpamConfig {
                    ipv4_address: Some(network.ipv4_address),
                    ..Default::default()
                }),
                ..Default::default()
            }
        };

        match &self.docker
            .connect_network(&network.network_name, config).await
            {
                Ok(_) => Ok(Response::new(Successed {
                    successed: true
                })),
                Err(e) => {
                    println!("Error! {:#?}", e);
                    Ok(Response::new(Successed {
                    successed: false
                })) }
            }
    }

    async fn start_container(&self, docker_container: Request<ContainerToStart>) -> ServerResult<Successed> {
        match &self.docker
        .start_container(&docker_container.into_inner().name, None::<StartContainerOptions<String>>).await
            {
                Ok(_) => Ok(Response::new(Successed {
                    successed: true
                })),
                Err(e) => {
                    println!("Error! {:#?}", e);
                    Ok(Response::new(Successed {
                    successed: false
                })) }
            }
    }
}