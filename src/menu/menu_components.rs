use bevy::prelude::Component;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Exit,
}

#[derive(Component)]
pub enum GameOverScreenAction {
    PlayAgain,
    Exit,
}

#[derive(Component)]
pub struct MenuCamera;