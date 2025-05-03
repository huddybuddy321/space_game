use bevy::prelude::*;
use avian2d::prelude::*;

pub struct TestLevelPlugin;

impl Plugin for TestLevelPlugin {
    fn build(&self, app : &mut App) {
        app.add_systems(Startup, spawn_test_level);
    }
}

const GROUND_WIDTH: f32 = 500.0;
const GROUND_HEIGHT: f32 = 25.0;

fn spawn_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) { 
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(GROUND_WIDTH, GROUND_HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(100., 100., 100.)))),
        RigidBody::Static,
        Collider::rectangle(GROUND_WIDTH, GROUND_HEIGHT),
        Transform::from_xyz(0.0, -100.0, 0.0)
    ));
}