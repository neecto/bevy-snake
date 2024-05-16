use rand::prelude::*;
use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Mesh, Rectangle, ResMut, Vec3};
use crate::rabbit::rabbit_components::Rabbit;
use crate::rabbit::rabbit_components;

pub fn init_rabbit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = thread_rng();
    let x = rng.gen::<f32>() * 800.0;
    let y = rng.gen::<f32>() * 500.0;
    let position = Vec3::new(x, y, 1.);

    let rabbit_bundle = Rabbit::get_spawn_bundle(
        meshes.add(Rectangle::new(rabbit_components::SIZE, rabbit_components::SIZE)).into(),
        materials.add(rabbit_components::COLOR),
        position
    );

    commands.spawn(rabbit_bundle);
}