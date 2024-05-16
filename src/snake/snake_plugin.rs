use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};

use crate::shared::game_state::GameState;
use crate::shared::input_system;
use crate::snake::snake_systems;

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), snake_systems::init_snake)
            .add_systems(
                FixedUpdate,
                (snake_systems::move_head, snake_systems::move_tail)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (input_system::handle_direction_input,).run_if(in_state(GameState::InGame)),
            );
    }
}