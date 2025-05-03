mod main_menu;
mod world;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_renet::steam::{
    SteamClientPlugin, SteamClientTransport, SteamServerPlugin, SteamServerTransport, SteamServerConfig, SteamTransportError, AccessPermission
};
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use bevy_renet::{client_connected, RenetClientPlugin, RenetServerPlugin};

use avian2d::prelude::*;
use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};
use serde::{Deserialize, Serialize};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use main_menu::MainMenuPlugin;
use world::WorldPlugin;
//use player::PlayerPlugin;
//use test_level::TestLevelPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InMenu,
    InWorld
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Stellarbase"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(1280.0, 720.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),

            //Renet
            RenetServerPlugin,
            SteamServerPlugin,
            RenetClientPlugin,
            SteamClientPlugin,

            //Debugs
            EguiPlugin {enable_multipass_for_primary_context: true},
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
            //PhysicsDebugPlugin::default()
        ))
        .init_state::<AppState>()
        // User configured plugins
        .add_plugins((MainMenuPlugin, WorldPlugin))
        .run();
}