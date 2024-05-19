use bevy::app::AppExit;
use bevy::prelude::*;
use crate::menu::menu_utils;
use crate::menu::menu_resources::{MenuData};
use crate::menu::menu_components::*;
use crate::menu::menu_utils::{spawn_gameover_screen, spawn_menu};
use crate::shared::game_state::GameState;

pub fn setup_menu(mut commands: Commands) {
    let background_color: Color = Color::rgb_u8(66, 80, 95);
    commands.spawn((Camera2dBundle {
        camera: Camera {
            // disable clearing completely (pixels stay as they are)
            // (preserves output from previous frame or camera/pass)
            clear_color: ClearColorConfig::Custom(background_color),
            ..Default::default()
        },
        ..Default::default()
    }, MenuCamera));

    let menu_entity = spawn_menu(&mut commands);
    commands.insert_resource(MenuData { menu_entity });
}

pub fn setup_gameover_screen(mut commands: Commands) {
    let background_color: Color = Color::rgb_u8(66, 80, 95);
    commands.spawn((Camera2dBundle {
        camera: Camera {
            // disable clearing completely (pixels stay as they are)
            // (preserves output from previous frame or camera/pass)
            clear_color: ClearColorConfig::Custom(background_color),
            ..Default::default()
        },
        ..Default::default()
    }, MenuCamera));

    let menu_entity = spawn_gameover_screen(&mut commands);
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
pub fn gameover_action(
    interaction_query: Query<
        (&Interaction, &GameOverScreenAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverScreenAction::Exit => {
                    app_exit_events.send(AppExit);
                }
                GameOverScreenAction::PlayAgain => {
                    game_state.set(GameState::InGame);
                }
            }
        }
    }

}

pub fn cleanup_menu(
    mut commands: Commands,
    menu_data: Res<MenuData>,
    query: Query<Entity, With<MenuCamera>>
) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
    let camera = query.single();
    commands.entity(camera).despawn();
}