use crate::components::head::{Direction, Head};
use crate::components::snake_chunk;
use crate::components::snake_chunk::SnakeChunk;
use bevy::prelude::KeyCode::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
use bevy::prelude::{ButtonInput, KeyCode, Query, Res};

pub fn handle_direction_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Head>,
) {
    let mut q = query.get_single_mut();

    if q.is_err() {
        return;
    }

    let mut head = q.unwrap();

    if keyboard_input.pressed(ArrowUp)
        && !matches!(head.direction, Direction::Down)
    {
        head.direction = Direction::Up;
    }

    if keyboard_input.pressed(ArrowDown)
        && !matches!(head.direction, Direction::Up)
    {
        head.direction = Direction::Down;
    }

    if keyboard_input.pressed(ArrowLeft)
        && !matches!(head.direction, Direction::Right)
    {
        head.direction = Direction::Left;
    }

    if keyboard_input.pressed(ArrowRight)
        && !matches!(head.direction, Direction::Left)
    {
        head.direction = Direction::Right;
    }
}
