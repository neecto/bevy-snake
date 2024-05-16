use bevy::prelude::*;

use menu::menu_plugin::MenuPlugin;
use crate::rabbit::rabbit_plugin::RabbitPlugin;
use crate::shared::game_state::GameState;
use crate::snake::snake_plugin::SnakePlugin;

mod menu;
mod shared;
mod snake;
mod rabbit;

const FRAME_DELAY_MS: f64 = 300.0;

fn main() {
    let background_color: Color = Color::rgb_u8(66, 80, 95);

    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(FRAME_DELAY_MS / 1000.0))
        .insert_resource(ClearColor(background_color))
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(MenuPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(RabbitPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}