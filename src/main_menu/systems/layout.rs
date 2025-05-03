use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::style::*;

pub fn spawn_main_menu_system(
    mut commands: Commands,
    asset_server : Res<AssetServer>
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu_system(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
    commands.spawn(Camera2d);

    /*
    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 42.0,
            ..default()
        }
    ));
    */

    commands.spawn((
        MainMenu {},
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            ..default()
        },
        children![(
            StartWorldButton {},
            MenuButton {},
            Button,
            Node {
                width: Val::Px(400.0),
                height: Val::Px(65.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON_COLOR),
            children![(
                Text::new("START WORLD"),
                TextFont {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(1., 1., 1.)),
            )]
        ), (
            JoinWorldButton {},
            MenuButton {},
            Button,
            Node {
                width: Val::Px(400.0),
                height: Val::Px(65.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON_COLOR),
            children![(
                Text::new("JOIN WORLD"),
                TextFont {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(1., 1., 1.)),
            )]
        )]
    ));
}