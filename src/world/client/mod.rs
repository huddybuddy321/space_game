use bevy::prelude::*;
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};

mod systems;
use systems::*;

pub struct WorldClientPlugin;

use crate::AppState;
use crate::world::AuthorityState;

impl Plugin for WorldClientPlugin {
    fn build(&self, app : &mut App) {
        app
        .add_systems(OnEnter(AuthorityState::IsClient), setup_client_system)
        //.add_systems(Update, (receive_message_system, input_system).run_if(resource_exists::<RenetClient>));
        .add_systems(Update, (receive_message_system, input_system).run_if(in_state(AuthorityState::IsClient)));
    }
}