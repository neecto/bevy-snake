use crate::components::head::{Direction, Head};
use crate::components::snake_chunk::{SnakeChunk};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Entity, Mesh, Mut, Query, Rectangle, ResMut, Transform, With};
use crate::components::snake_chunk;

pub fn move_head(
    mut query: Query<(Entity, &mut Transform, &SnakeChunk, &Head)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut head = query.single_mut();
    let position = get_head_next_position(head.3, head.1);

    let new_head = SnakeChunk::get_spawn_bundle(
        meshes.add(Rectangle::new(snake_chunk::BODY_CHUNK_SIZE, snake_chunk::BODY_CHUNK_SIZE)).into(),
        materials.add(snake_chunk::COLOR),
        Some(head.0),
        position
    );

    let new_head_entity = commands.spawn(new_head).id();
    commands.entity(new_head_entity).insert(Head {
        direction: head.3.direction
    });

    commands.entity(head.0).remove::<Head>();
}

fn get_head_next_position(head: &Head, transform: Mut<Transform>) -> Vec3 {
    match head.direction {
        Direction::Up => {
            Vec3::new(transform.translation.x, transform.translation.y + snake_chunk::BODY_CHUNK_SIZE, transform.translation.z)
        }
        Direction::Down => {
            Vec3::new(transform.translation.x, transform.translation.y - snake_chunk::BODY_CHUNK_SIZE, transform.translation.z)
        }
        Direction::Left => {
            Vec3::new(transform.translation.x - snake_chunk::BODY_CHUNK_SIZE, transform.translation.y, transform.translation.z)
        }
        Direction::Right => {
            Vec3::new(transform.translation.x + snake_chunk::BODY_CHUNK_SIZE, transform.translation.y, transform.translation.z)
        }
    }
}
