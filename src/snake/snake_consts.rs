use bevy::math::Vec3;
use bevy::prelude::Color;

pub const BODY_CHUNK_SIZE: f32 = 32.0;
pub const HEAD_START_POSITION: Vec3 = Vec3::new(0.0,BODY_CHUNK_SIZE, 1.0);
pub const TAIL_START_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const COLOR: Color = Color::rgb(178. / 255.,73. / 255., 78. / 255.);
