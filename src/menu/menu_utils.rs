use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    AlignItems, BorderColor, ButtonBundle, Color, Commands, default,
    Entity, FlexDirection, JustifyContent, NodeBundle, Style, TextBundle, TextStyle, UiRect, Val,
};

use crate::menu::menu_data::MenuButtonAction;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(124. / 255., 124. / 255., 124. / 255.);
pub const TEXT: Color = Color::rgb(254. / 255., 229. / 255., 154. / 255.);

pub fn spawn_menu(commands: &mut Commands) -> Entity {
    let container_node = NodeBundle {
        style: Style {
            // center button
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    let title_node = TextBundle::from_section(
        "<Bevy Snake>",
        TextStyle {
            font_size: 40.0,
            color: TEXT,
            ..default()
        },
    );

    let container = commands.spawn(container_node).id();
    let title = commands.spawn(title_node).id();

    let play_button = spawn_button(commands, "Play", MenuButtonAction::Play);
    let exit_button = spawn_button(commands, "Exit", MenuButtonAction::Exit);
    commands
        .entity(container)
        .push_children(&[title, play_button, exit_button]);

    container
}

pub fn spawn_button(commands: &mut Commands, text: &str, action: MenuButtonAction) -> Entity {
    let button_node = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            margin: UiRect {
                top: Val::Px(40.),
                ..default()
            },
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    let button_text_node = TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: TEXT,
            ..default()
        },
    );

    let button = commands.spawn((button_node, action)).id();
    let button_text = commands.spawn(button_text_node).id();

    commands.entity(button).push_children(&[button_text]);
    button
}
