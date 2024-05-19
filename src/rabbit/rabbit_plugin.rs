use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};

use crate::rabbit::rabbit_systems;
use crate::shared::game_state::GameState;

pub struct RabbitPlugin;
impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), rabbit_systems::init_rabbit)
            .add_systems(
                Update,
                rabbit_systems::update_rabbit.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), rabbit_systems::cleanup);
    }
}
