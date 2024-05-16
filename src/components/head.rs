use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Head {
    pub direction: Direction
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match &self {
            Direction::Up => {Direction::Up}
            Direction::Down => {Direction::Down}
            Direction::Left => {Direction::Left}
            Direction::Right => {Direction::Right}
        }
    }
}

impl Copy for Direction {}