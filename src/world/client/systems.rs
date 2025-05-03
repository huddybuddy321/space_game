use bevy::asset::uuid::serde::compact::deserialize;
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

const PROTOCOL_ID: u64 = 0;

/*
pub fn setup_client_system(
    mut commands: Commands,
) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    commands.insert_resource(client);
    commands.insert_resource(transport);
}
*/
pub fn setup_client_system(
    mut commands: Commands,
) {
    //Setup steam client
    let (steam_client, single) = Client::init_app(480).unwrap();
    steam_client.networking_utils().init_relay_network_access();

    
    // Create renet client
    let connection_config = ConnectionConfig::default();
    let client = RenetClient::new(connection_config);
    commands.insert_resource(client);

    // Create steam transport
    let server_steam_id = SteamId::from_raw(76561199007142871); // Here goes the steam id of the host
    let steam_transport = SteamClientTransport::new(&steam_client, &server_steam_id).unwrap();
    commands.insert_resource(steam_transport);

    println!("Client initialized!");
}

pub fn receive_message_system(
    mut client: ResMut<RenetClient>
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let message: String = bincode::deserialize(&message).unwrap();
        println!("{}", message);
    }
}

pub fn input_system(
    mut client: ResMut<RenetClient>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        println!("Send hey message!");
        let message = bincode::serialize("Hey server!").unwrap();
        client.send_message(DefaultChannel::ReliableOrdered, message);
    }
}