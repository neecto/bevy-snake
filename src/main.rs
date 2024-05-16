use bevy::prelude::*;

use crate::systems::head_movement::move_head;
use crate::systems::tail_movement::move_tail;
use systems::game_init;

mod components;
mod plugins;
mod resources;
mod systems;

const MOVEMENT_DELAY_MS: f64 = 300.0;

fn main() {
    let background_color: Color = Color::rgb_u8(66, 80, 95);

    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(MOVEMENT_DELAY_MS / 1000.0))
        .insert_resource(ClearColor(background_color))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, game_init::init)
        .add_systems(FixedUpdate, (move_head, move_tail).chain())
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                systems::input::handle_direction_input,
            ),
        )
        .run();
}
