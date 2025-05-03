use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::events::StartWorldEvent;
use crate::main_menu::events::JoinWorldEvent;
use crate::main_menu::style::*;
//use crate::AppState;

pub fn interact_with_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<MenuButton>)
    >
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_start_world_button_system(
    mut button_query : Query<
        &Interaction,
        (Changed<Interaction>, With<StartWorldButton>)
    >,
    mut start_world_event_writer: EventWriter<StartWorldEvent>
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            start_world_event_writer.write(StartWorldEvent());
        }
    }
}

pub fn interact_with_join_world_button_system(
    mut button_query : Query<
        &Interaction,
        (Changed<Interaction>, With<JoinWorldButton>)
    >,
    mut join_world_event_writer: EventWriter<JoinWorldEvent>
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            join_world_event_writer.write(JoinWorldEvent());
        }
    }
}