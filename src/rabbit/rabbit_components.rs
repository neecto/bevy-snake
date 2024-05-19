use crate::field::field_consts;
use crate::snake::snake_consts;
use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{default, Color, ColorMaterial, Component, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{thread_rng, Rng};

pub const COLOR: Color = Color::rgb(132. / 255., 191. / 255., 104. / 255.);
pub const SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Rabbit;

impl Rabbit {
    pub fn get_next_position() -> Vec3 {
        let mut rng = thread_rng();

        let max_w_pos = (field_consts::FIELD_WIDTH_POSITIONS / 2) as i16;
        let max_h_pos = (field_consts::FIELD_HEIGHT_POSITIONS / 2) as i16;
        let x: i16 = rng.gen_range(-max_w_pos..max_w_pos) * snake_consts::BODY_CHUNK_SIZE as i16;
        let y: i16 = rng.gen_range(-max_h_pos..max_h_pos) * snake_consts::BODY_CHUNK_SIZE as i16;
        let position = Vec3::new(x as f32, y as f32, 1.);
        println!("Rabbit spawned at {}", position);

        position
    }

    pub fn get_spawn_bundle(
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
        position: Vec3,
    ) -> (Self, MaterialMesh2dBundle<ColorMaterial>) {
        (
            Self,
            MaterialMesh2dBundle {
                mesh: mesh,
                transform: Transform::from_translation(position),
                material: material,
                ..default()
            },
        )
    }
}
