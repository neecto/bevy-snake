use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Entity, EventReader, Mesh, Query, Rectangle, ResMut, With};

use crate::field::field_events::RabbitEatenEvent;
use crate::rabbit::rabbit_components;
use crate::rabbit::rabbit_components::Rabbit;

pub fn init_rabbit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let position = Rabbit::get_next_position();

    let rabbit_bundle = Rabbit::get_spawn_bundle(
        meshes.add(Rectangle::new(rabbit_components::SIZE, rabbit_components::SIZE)).into(),
        materials.add(rabbit_components::COLOR),
        position
    );

    commands.spawn(rabbit_bundle);
}

pub fn update_rabbit(
    mut commands: Commands,
    mut rabbit_query: Query<Entity, With<Rabbit>>,
    mut eaten_events: EventReader<RabbitEatenEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if eaten_events.is_empty() {
        return;
    }

    let rabbit_entity = rabbit_query.get_single_mut();
    if rabbit_entity.is_ok() {
        commands.entity(rabbit_entity.unwrap()).despawn();
    }

    let rabbit_position = Rabbit::get_next_position();
    println!("Rabbit spawned at {}", rabbit_position);

    let rabbit_bundle = Rabbit::get_spawn_bundle(
        meshes.add(Rectangle::new(rabbit_components::SIZE, rabbit_components::SIZE)).into(),
        materials.add(rabbit_components::COLOR),
        rabbit_position
    );

    commands.spawn(rabbit_bundle);

    eaten_events.clear();
}

pub fn cleanup(
    mut commands: Commands,
    rabbit_query: Query<Entity, With<Rabbit>>
) {
    for rabbit in rabbit_query.iter() {
        commands.entity(rabbit).despawn()
    }
}
