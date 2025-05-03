use bevy::prelude::*;

mod systems;
use systems::*;

pub struct WorldServerPlugin;

use crate::world::AuthorityState;

impl Plugin for WorldServerPlugin {
    fn build(&self, app : &mut App) {
        app
        //Spawn menu when entering main menu
        .add_systems(OnEnter(AuthorityState::IsHost), setup_server_system)
        .add_systems(Update, server_update_system.run_if(in_state(AuthorityState::IsHost)));
    }
}