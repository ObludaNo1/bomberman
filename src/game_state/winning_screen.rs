use bevy::prelude::*;

use crate::{game_state::GameState, world_entities::FontHandle};

#[derive(Component)]
pub struct WinningMenuScreen;

#[derive(Component)]
pub struct WinningMenuButton;

pub fn spawn_winning_screen(mut commands: Commands, font_handle: Res<FontHandle>) {
    commands
        .spawn((
            WinningMenuScreen,
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
                Text::new("Victory!"),
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

            parent
                .spawn((
                    Button,
                    WinningMenuButton,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
                ))
                .with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font_size: 28.0,
                            font: font_handle.0.clone(),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn despawn_winning_screen(
    mut commands: Commands,
    query: Query<Entity, With<WinningMenuScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_win_screen_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (With<WinningMenuButton>, Changed<Interaction>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

pub fn handle_win_screen_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<WinningMenuButton>),
    >,
) {
    for (interaction, mut bg) in interaction_query.iter_mut() {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.25, 0.25, 0.5, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.35, 0.35, 0.6, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
        };
    }
}
