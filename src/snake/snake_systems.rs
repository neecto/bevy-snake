use crate::rabbit::rabbit_components;
use crate::rabbit::rabbit_components::Rabbit;
use crate::snake::snake_consts;
use crate::snake::snake_components::SnakeChunk;
use crate::snake::snake_components::{Direction, Head};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::math::Vec3;
use bevy::prelude::{Assets, ColorMaterial, Commands, Entity, EventWriter, Mesh, Mut, Query, Rectangle, ResMut, Transform, With};
use crate::field::field_events::RabbitEatenEvent;

pub fn init_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tail_bundle = SnakeChunk::get_spawn_bundle(
        meshes
            .add(Rectangle::new(
                snake_consts::BODY_CHUNK_SIZE,
                snake_consts::BODY_CHUNK_SIZE,
            ))
            .into(),
        materials.add(snake_consts::COLOR),
        None,
        snake_consts::TAIL_START_POSITION,
    );
    let tail_entity = commands.spawn(tail_bundle).id();

    let head_bundle = SnakeChunk::get_spawn_bundle(
        meshes
            .add(Rectangle::new(
                snake_consts::BODY_CHUNK_SIZE,
                snake_consts::BODY_CHUNK_SIZE,
            ))
            .into(),
        materials.add(snake_consts::COLOR),
        Some(tail_entity),
        snake_consts::HEAD_START_POSITION,
    );
    let head_entity = commands.spawn(head_bundle).id();
    commands.entity(head_entity).insert(Head {
        direction: Direction::Up,
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
        meshes
            .add(Rectangle::new(
                snake_consts::BODY_CHUNK_SIZE,
                snake_consts::BODY_CHUNK_SIZE,
            ))
            .into(),
        materials.add(snake_consts::COLOR),
        Some(head.0),
        position,
    );

    let new_head_entity = commands.spawn(new_head).id();
    commands.entity(new_head_entity).insert(Head {
        direction: head.3.direction,
    });

    commands.entity(head.0).remove::<Head>();
}

pub fn move_tail(
    mut commands: Commands,
    head_query: Query<&Transform, With<Head>>,
    mut snake_query: Query<(Entity, &mut SnakeChunk)>,
    rabbit_query: Query<&Transform, With<Rabbit>>,
    mut rabbit_eaten_events: EventWriter<RabbitEatenEvent>
) {
    let is_eating = is_eating_rabbit(head_query, rabbit_query);
    if is_eating {
        rabbit_eaten_events.send_default();
        return;
    }

    let mut deleted_tail_entity = None;
    for entity_id in snake_query.iter() {
        if entity_id.1.next_chunk.is_some() {
            continue;
        }

        commands.entity(entity_id.0).despawn();
        deleted_tail_entity = Some(entity_id.0);
    }

    if deleted_tail_entity.is_none() {
        return;
    }

    for mut entity_id in snake_query.iter_mut() {
        if entity_id.1.next_chunk.is_none() {
            continue;
        }

        if entity_id.1.next_chunk.unwrap().index() == deleted_tail_entity.unwrap().index() {
            entity_id.1.next_chunk.take();
        }
    }
}

pub fn cleanup(
    mut commands: Commands,
    mut snake_query: Query<Entity, With<SnakeChunk>>,
) {
    for chunk in snake_query.iter() {
        commands.entity(chunk).despawn()
    }
}

fn get_head_next_position(head: &Head, transform: Mut<Transform>) -> Vec3 {
    match head.direction {
        Direction::Up => Vec3::new(
            transform.translation.x,
            transform.translation.y + snake_consts::BODY_CHUNK_SIZE,
            transform.translation.z,
        ),
        Direction::Down => Vec3::new(
            transform.translation.x,
            transform.translation.y - snake_consts::BODY_CHUNK_SIZE,
            transform.translation.z,
        ),
        Direction::Left => Vec3::new(
            transform.translation.x - snake_consts::BODY_CHUNK_SIZE,
            transform.translation.y,
            transform.translation.z,
        ),
        Direction::Right => Vec3::new(
            transform.translation.x + snake_consts::BODY_CHUNK_SIZE,
            transform.translation.y,
            transform.translation.z,
        ),
    }
}

fn is_eating_rabbit(
    mut head_query: Query<&Transform, With<Head>>,
    mut rabbit_query: Query<&Transform, With<Rabbit>>,
) -> bool {
    let rabbit = rabbit_query.get_single_mut();

    if rabbit.is_err() {
        return false;
    }

    let rabbit_transform = rabbit.unwrap();
    let head = head_query.single_mut();

    let rabbit_box = Aabb2d::new(
        rabbit_transform.translation.truncate(),
        Vec3::new(
            rabbit_transform.translation.x + rabbit_components::SIZE,
            rabbit_transform.translation.x + rabbit_components::SIZE,
            1.
        ).truncate(),
    );
    let head_box = Aabb2d::new(
        head.translation.truncate(),
        head.scale.truncate() / 2.,
    );

    let intersects = head_box.intersects(&rabbit_box);
    if intersects {
        println!("Rabbit eaten at {}", head.translation);
    }
    intersects
}
