use bevy::app::{App, Plugin};
use bevy::prelude::{ClearColor, Color, FixedUpdate, in_state, IntoSystemConfigs, OnEnter, OnExit};

use crate::field::field_events::RabbitEatenEvent;
use crate::field::field_systems;
use crate::shared::game_state::GameState;

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        let background_color: Color = Color::rgb_u8(30, 30, 30);

        app
            .insert_resource(ClearColor(background_color))
            .add_systems(OnEnter(GameState::InGame), field_systems::init_field)
            .add_systems(FixedUpdate, field_systems::check_collision.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), field_systems::clean_field)
            .add_event::<RabbitEatenEvent>();
    }
}