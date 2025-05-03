use bevy::prelude::*;
use avian2d::prelude::*;

pub struct PlayerPlugin;

const PIXEL_RATIO: f32 = 3.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app : &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server : Res<AssetServer>
) { 
    commands.spawn(Camera2d);
    
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/player/player.png"),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::capsule(8.0, 27.0),
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO))
        //Transform::from_rotation(Quat::from_rotation_z(-45.)).with_scale(Vec3::splat(PIXEL_RATIO)),
    ));
    
    spawn_camera(commands);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d {});
}