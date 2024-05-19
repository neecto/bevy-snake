use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Component, default, Entity, Handle, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component, Debug)]
pub struct SnakeChunk {
    pub next_chunk: Option<Entity>
}

impl SnakeChunk {
    pub fn get_spawn_bundle(
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
        next_chunk: Option<Entity>,
        position: Vec3
    ) -> (
        Self,
        MaterialMesh2dBundle<ColorMaterial>,
    ) {
        (
            Self {
                next_chunk: next_chunk
            },
            MaterialMesh2dBundle {
                mesh: mesh,
                transform: Transform::from_translation(position),
                material: material,
                ..default()
            }
        )
    }
}

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
            Direction::Up => { Direction::Up}
            Direction::Down => { Direction::Down}
            Direction::Left => { Direction::Left}
            Direction::Right => { Direction::Right}
        }
    }
}

impl Copy for Direction {}