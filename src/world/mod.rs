//mod server;
//mod client;

mod server;
mod client;
mod systems;

//use server::ExampleServerPlugin;
//use client::ExampleClientPlugin;

use server::*;
use client::*;

use systems::*;
//use player::PlayerPlugin;

use bevy::prelude::*;

use crate::AppState;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AuthorityState {
    #[default]
    Undecided,
    IsHost,
    IsClient
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app : &mut App) {
        app
        //Handle start world
        //.add_plugins((ExampleServerPlugin, ExampleClientPlugin))
        .init_state::<AuthorityState>()
        .add_plugins((WorldServerPlugin, WorldClientPlugin))
        .add_systems(Update, (handle_start_world_event_system, handle_join_world_event_system).run_if(in_state(AppState::InMenu)));
    }
}