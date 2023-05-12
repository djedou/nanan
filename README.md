# Nanan: Docker Containers Orchestrator in Rust

# Steps:  
## Zookeeper:  
### Step 1:
    Create zookeeper docker image.  
### Step 2:
    Creae zookeeper docker network  
    192.168.255.8/29 => 255.9 - 255.16
### Step 3:  
    Create zookeeper docker container 
### Step 4: 
    Connect zookeeper to zookeeper network
### Step 4:
    Run zookeeper docker container

## Broker:  
### Step 1:
    Create Kafka docker image  
### Step 2:
    Creae kafka docker network  
    192.168.255.0/29 => 255.1 - 255.7

### Step 3:  
    Create kafka_primary docker container 

### Some usefull commands:
docker inspect icc_zookeeper_1 - f "{{json .NetworkSettings.Networks }}"  
docker inspect icc_kafka_primary - f "{{json .NetworkSettings.Networks }}"  
docker inspect icc_kafka_secondary - f "{{json .NetworkSettings.Networks }}"  
  
docker network rm icc_zookeeper_net  
  
docker network disconnect bridge icc_zookeeper_1  
docker network disconnect bridge icc_kafka_primary  
docker network disconnect bridge icc_kafka_secondary  