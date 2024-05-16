use bevy::prelude::{Entity, Resource, Component};

#[derive(Resource)]
pub struct MenuData {
    pub menu_entity: Entity,
}

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Exit,
}
