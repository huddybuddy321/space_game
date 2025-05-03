pub mod components;
pub mod events;
mod systems;
mod style;

use events::*;
use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app : &mut App) {
        app
        //Add events
        .add_event::<StartWorldEvent>()
        .add_event::<JoinWorldEvent>()
        
        //Spawn menu when entering main menu
        .add_systems(OnEnter(AppState::InMenu), spawn_main_menu_system)
        //Update main menu
        .add_systems(Update, (interact_with_button_system, interact_with_start_world_button_system, interact_with_join_world_button_system).run_if(in_state(AppState::InMenu)))
        //Close menu when entering world
        .add_systems(OnExit(AppState::InMenu), despawn_main_menu_system);
    }
}