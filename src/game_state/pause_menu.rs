use bevy::prelude::*;

use crate::{game_state::PlayingState, world_entities::FontHandle};

#[derive(Component)]
pub struct PauseMenuScreen;

pub fn pause_on_esc(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(PlayingState::Pause);
    }
}

pub fn resume_on_esc(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(PlayingState::Playing);
    }
}

pub fn spawn_pause_menu(mut commands: Commands, font_handle: Res<FontHandle>) {
    commands
        .spawn((
            PauseMenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Paused"),
                TextFont {
                    font_size: 64.0,
                    font: font_handle.0.clone(),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Press ESC to resume"),
                TextFont {
                    font_size: 32.0,
                    font: font_handle.0.clone(),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
            ));
        });
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuScreen>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
