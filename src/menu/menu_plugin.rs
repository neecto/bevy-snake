use bevy::app::Update;
use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin};

use crate::menu::menu_systems;
use crate::shared::game_state::GameState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_systems::setup_menu)
            .add_systems(
                Update,
                (menu_systems::update_menu, menu_systems::menu_action)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), menu_systems::cleanup_menu)
            .add_systems(
                OnEnter(GameState::GameOver),
                menu_systems::setup_gameover_screen,
            )
            .add_systems(
                Update,
                (menu_systems::update_menu, menu_systems::gameover_action)
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                menu_systems::cleanup_menu
            );
    }
}
