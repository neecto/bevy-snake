use bevy::prelude::*;
use bevy::window::WindowMode;

use menu::menu_plugin::MenuPlugin;

use crate::field::field_consts::{FIELD_HEIGHT_PX, FIELD_WIDTH_PX};
use crate::field::field_plugin::FieldPlugin;
use crate::rabbit::rabbit_plugin::RabbitPlugin;
use crate::shared::game_state::GameState;
use crate::snake::snake_consts::BODY_CHUNK_SIZE;
use crate::snake::snake_plugin::SnakePlugin;

mod field;
mod menu;
mod rabbit;
mod shared;
mod snake;

const FRAME_DELAY_MS: f64 = 300.0;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "<Bevy Snake>".into(),
            mode: WindowMode::Windowed,
            resizable: false,
            resolution: (
                FIELD_WIDTH_PX as f32 + BODY_CHUNK_SIZE,
                FIELD_HEIGHT_PX as f32 + BODY_CHUNK_SIZE,
            )
                .into(),
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(FRAME_DELAY_MS / 1000.0))
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(MenuPlugin)
        .add_plugins(FieldPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(RabbitPlugin)
        .run();
}
