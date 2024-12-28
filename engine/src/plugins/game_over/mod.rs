use bevy::{app::{App, Plugin}, asset::{AssetServer, Assets}, color::Color, prelude::{BuildChildren, ChildBuild, Commands, Component, OnEnter, Res, ResMut, Text}, sprite::ColorMaterial, text::{TextColor, TextFont}, ui::{AlignItems, DefaultUiCamera, JustifyContent, Node, Val}, utils::default};

use crate::state::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), show_game_over_screen);
    }
}

#[derive(Component)]
struct GameOverScreen;

fn show_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0), 
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .insert(GameOverScreen)
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}