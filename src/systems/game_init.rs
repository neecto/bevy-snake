use bevy::asset::Assets;
use bevy::prelude::{Camera2dBundle, Color, ColorMaterial, Commands, Mesh, Rectangle, ResMut};
use crate::components::head::{Direction, Head};

use crate::components::snake_chunk;
use crate::components::snake_chunk::SnakeChunk;

pub fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let tail_bundle = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_chunk::BODY_CHUNK_SIZE, snake_chunk::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_chunk::COLOR),
        None,
        snake_chunk::TAIL_START_POSITION,
    );
    let tail_entity = commands.spawn(tail_bundle).id();
    println!("Spawned tail");

    let head_bundle = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_chunk::BODY_CHUNK_SIZE, snake_chunk::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_chunk::COLOR),
        Some(tail_entity),
        snake_chunk::HEAD_START_POSITION,
    );
    let head_entity = commands.spawn(head_bundle).id();
    commands.entity(head_entity).insert(Head {
        direction: Direction::Up
    });
    println!("Spawned head");
}
