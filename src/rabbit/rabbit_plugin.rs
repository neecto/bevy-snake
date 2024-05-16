use crate::rabbit::rabbit_systems;
use crate::shared::game_state::GameState;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};

pub struct RabbitPlugin;
impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), rabbit_systems::init_rabbit);
    }
}
