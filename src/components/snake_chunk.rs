use bevy::math::Vec3;
use bevy::prelude::{Color, ColorMaterial, Component, default, Entity, Handle, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub const BODY_CHUNK_SIZE: f32 = 32.0;
pub const HEAD_START_POSITION: Vec3 = Vec3::new(0.0,BODY_CHUNK_SIZE, 1.0);
pub const TAIL_START_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const STEP_SIZE: f32 = BODY_CHUNK_SIZE;
pub const COLOR: Color = Color::rgb(178. / 255.,73. / 255., 78. / 255.);

#[derive(Component)]
pub struct SnakeChunk {
    pub next_chunk: Option<Entity>
}

impl SnakeChunk {
    pub fn get_spawn_bundle(
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
        next_chunk: Option<Entity>,
        position: Vec3
    ) -> (
        Self,
        MaterialMesh2dBundle<ColorMaterial>,
    ) {
        (
            Self {
                next_chunk: next_chunk
            },
            MaterialMesh2dBundle {
                mesh: mesh,
                transform: Transform::from_translation(position),
                material: material,
                ..default()
            }
        )
    }
}
