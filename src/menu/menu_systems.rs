use bevy::app::AppExit;
use bevy::prelude::*;
use crate::menu::menu_utils;
use crate::menu::menu_data::{MenuButtonAction, MenuData};
use crate::menu::menu_utils::spawn_menu;
use crate::shared::game_state::GameState;

pub fn setup_menu(mut commands: Commands) {
    let menu_entity = spawn_menu(&mut commands);
    commands.insert_resource(MenuData { menu_entity });
}

pub fn update_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = menu_utils::PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = menu_utils::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = menu_utils::NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Exit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::InGame);
                }
            }
        }
    }

}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
}