use bevy::math::Vec3;
use bevy::prelude::{Assets, ColorMaterial, Commands, Entity, Mesh, Mut, Query, Rectangle, ResMut, Transform};
use crate::snake::snake_components;
use crate::snake::snake_components::SnakeChunk;
use crate::snake::snake_components::{Direction, Head};

pub fn init_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tail_bundle = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_components::BODY_CHUNK_SIZE, snake_components::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_components::COLOR),
        None,
        snake_components::TAIL_START_POSITION,
    );
    let tail_entity = commands.spawn(tail_bundle).id();

    let head_bundle = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_components::BODY_CHUNK_SIZE, snake_components::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_components::COLOR),
        Some(tail_entity),
        snake_components::HEAD_START_POSITION,
    );
    let head_entity = commands.spawn(head_bundle).id();
    commands.entity(head_entity).insert(Head {
        direction: Direction::Up
    });
}

pub fn move_head(
    mut query: Query<(Entity, &mut Transform, &SnakeChunk, &Head)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let head = query.single_mut();
    let position = get_head_next_position(head.3, head.1);

    let new_head = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_components::BODY_CHUNK_SIZE, snake_components::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_components::COLOR),
        Some(head.0),
        position
    );

    let new_head_entity = commands.spawn(new_head).id();
    commands.entity(new_head_entity).insert(Head {
        direction: head.3.direction
    });

    commands.entity(head.0).remove::<Head>();
}

pub fn move_tail(
    mut query: Query<(Entity, &mut SnakeChunk)>,
    mut commands: Commands,
) {
    let mut deleted_tail_entity = None;
    for entity_id in query.iter() {
        if entity_id.1.next_chunk.is_some() {
            continue;
        }

        commands.entity(entity_id.0).despawn();
        deleted_tail_entity = Some(entity_id.0);
    }

    if deleted_tail_entity.is_none() {
        return;
    }

    for mut entity_id in query.iter_mut() {
        if entity_id.1.next_chunk.is_none() {
            continue;
        }

        if entity_id.1.next_chunk.unwrap().index() == deleted_tail_entity.unwrap().index() {
            entity_id.1.next_chunk.take();
        }
    }
}

fn get_head_next_position(head: &Head, transform: Mut<Transform>) -> Vec3 {
    match head.direction {
        Direction::Up => {
            Vec3::new(transform.translation.x, transform.translation.y + snake_components::BODY_CHUNK_SIZE, transform.translation.z)
        }
        Direction::Down => {
            Vec3::new(transform.translation.x, transform.translation.y - snake_components::BODY_CHUNK_SIZE, transform.translation.z)
        }
        Direction::Left => {
            Vec3::new(transform.translation.x - snake_components::BODY_CHUNK_SIZE, transform.translation.y, transform.translation.z)
        }
        Direction::Right => {
            Vec3::new(transform.translation.x + snake_components::BODY_CHUNK_SIZE, transform.translation.y, transform.translation.z)
        }
    }
}
