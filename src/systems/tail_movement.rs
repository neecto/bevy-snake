use bevy::asset::ron::Value::Option;
use bevy::prelude::{Commands, Entity, Query};

use crate::components::snake_chunk::SnakeChunk;

pub fn move_tail(
    mut query: Query<(Entity, &mut SnakeChunk)>,
    mut commands: Commands,
) {
    let mut deleted_tail_entity = None;
    for entity_id in query.iter() {
        println!("Checking entity {:#?}", entity_id.1);
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