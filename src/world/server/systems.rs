use bevy::prelude::*;
/*
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeServerPlugin, NetcodeServerTransport, NetcodeTransportError,
    ServerAuthentication, ServerConfig,
};
*/
use bevy_renet::steam::{
    SteamClientPlugin, SteamClientTransport, SteamServerPlugin, SteamServerTransport, SteamServerConfig, SteamTransportError, AccessPermission
};
use bevy_renet::steam::steamworks::*;
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use bevy_renet::{client_connected, RenetClientPlugin, RenetServerPlugin};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

/*
pub fn setup_server_system(
    mut commands: Commands,
) {
    let server = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);

    // Transport layer setup
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);

    println!("Server initialized!");
}
*/
pub fn setup_server_system(
    mut commands: Commands,
) {
    //Setup steam client
    let (steam_client, single) = Client::init_app(480).unwrap();
    steam_client.networking_utils().init_relay_network_access();

    //Create renet server
    let server: RenetServer = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);

    //Create steam transport
    let access_permission = AccessPermission::Public;
    let steam_transport_config = SteamServerConfig {
        max_clients: 10,
        access_permission
    };
    let steam_transport = SteamServerTransport::new(&steam_client, steam_transport_config).unwrap();
    //commands.insert_resource(steam_transport);

    println!("Server initialized!");
}

pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                let message = bincode::serialize("Hello world!").unwrap();
                server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
            }
        }
    }
    
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let message: String = bincode::deserialize(&message).unwrap();
            println!("Player {} says: {}", client_id, message);
        }
    }
}