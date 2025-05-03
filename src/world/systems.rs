use bevy::prelude::*;
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeServerPlugin, NetcodeServerTransport, NetcodeTransportError,
    ServerAuthentication, ServerConfig,
};
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use bevy_renet::{client_connected, RenetClientPlugin, RenetServerPlugin};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};
use serde::{Deserialize, Serialize};

use crate::main_menu::events::{JoinWorldEvent, StartWorldEvent};
use crate::AppState;
use bevy::app;

use crate::world::AuthorityState;

pub fn handle_start_world_event_system(
    mut commands: Commands,
    start_world_event_reader: EventReader<StartWorldEvent>,
    app_state_next_state: ResMut<NextState<AppState>>,
    mut authority_state_next_state: ResMut<NextState<AuthorityState>>
) {
    if !start_world_event_reader.is_empty() {
        //Start world
        authority_state_next_state.set(AuthorityState::IsHost);
        setup_world(app_state_next_state);
    }
}

pub fn handle_join_world_event_system(
    join_world_event_reader: EventReader<JoinWorldEvent>,
    app_state_next_state: ResMut<NextState<AppState>>,
    mut authority_state_next_state: ResMut<NextState<AuthorityState>>
) {
    if !join_world_event_reader.is_empty() {
        //Join world
        setup_world(app_state_next_state);
        authority_state_next_state.set(AuthorityState::IsClient);
    }
}

pub fn setup_world(
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    app_state_next_state.set(AppState::InWorld);
}