use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Color, ColorMaterial, Component, default, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{Rng, thread_rng};

pub const COLOR: Color = Color::rgb(132. / 255.,191. / 255., 104. / 255.);
pub const SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Rabbit;

impl Rabbit {
    pub fn get_next_position() -> Vec3 {
        let mut rng = thread_rng();
        let x = rng.gen::<f32>() * 100.0;
        let y = rng.gen::<f32>() * 100.0;
        let position = Vec3::new(x, y, 1.);

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
