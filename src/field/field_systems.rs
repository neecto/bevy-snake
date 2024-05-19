use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::field::field_components::{Field, FieldCamera};
use crate::field::field_consts::{FIELD_HEIGHT_PX, FIELD_WIDTH_PX};
use crate::shared::game_state::GameState;
use crate::snake::snake_components::Head;

pub fn init_field(mut commands: Commands) {
    let field_color: Color = Color::rgb_u8(66, 80, 95);
    let background_color: Color = Color::rgb_u8(30, 30, 30);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // disable clearing completely (pixels stay as they are)
                // (preserves output from previous frame or camera/pass)
                clear_color: ClearColorConfig::Custom(background_color),
                ..Default::default()
            },
            ..Default::default()
        },
        FieldCamera,
    ));

    let field_transform = Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        scale: Vec3::new(FIELD_WIDTH_PX as f32, FIELD_HEIGHT_PX as f32, 0.0),
        ..default()
    };

    let field_bb = Aabb2d::new(
        field_transform.translation.truncate(),
        field_transform.scale.truncate() / 2.
    );
    println!("Spawned field {:#?}", field_bb);

    commands.spawn((
        SpriteBundle {
            transform: field_transform,
            sprite: Sprite {
                color: field_color,
                ..default()
            },
            ..default()
        },
        Field,
    ));
}

pub fn check_collision(
    field_query: Query<&Transform, With<Field>>,
    head_query: Query<&Transform, With<Head>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let field_transform = field_query.single();
    let field_box = Aabb2d::new(
        field_transform.translation.truncate(),
        field_transform.scale.truncate() / 2.
    );

    let head_transform = head_query.single();
    let head_box = Aabb2d::new(
        head_transform.translation.truncate(),
        head_transform.scale.truncate() / 2.
    );

    if field_box.intersects(&head_box) {
        return;
    }

    game_state.set(GameState::GameOver);
}

pub fn clean_field(
    mut commands: Commands,
    field_query: Query<Entity, With<Field>>,
    cam_query: Query<Entity, With<Camera>>,
) {
    for field in field_query.iter() {
        commands.entity(field).despawn();
    }

    for cam in cam_query.iter() {
        commands.entity(cam).despawn();
    }
}
