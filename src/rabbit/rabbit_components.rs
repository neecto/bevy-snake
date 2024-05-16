use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{default, ColorMaterial, Component, Entity, Transform, Color};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub const COLOR: Color = Color::rgb(132. / 255.,191. / 255., 104. / 255.);
pub const SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Rabbit;

impl Rabbit {
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
