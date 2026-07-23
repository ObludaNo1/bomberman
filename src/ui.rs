use std::f32;

use bevy::prelude::*;

use crate::{
    constants::TOP_MENU_BAR_HEIGHT,
    game_state::STARTS_PLAYING,
    world_entities::{FontHandle, GamePlayTimer, InGameEntity, RenderedAreaWidth},
};

#[derive(Component)]
struct TopBarMenuText;

fn spawn_top_menu(
    mut commands: Commands,
    rendered_area_width: Res<RenderedAreaWidth>,
    font_handle: Res<FontHandle>,
) {
    commands
        .spawn((
            InGameEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(TOP_MENU_BAR_HEIGHT),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(rendered_area_width.0),
                        height: Val::Px(TOP_MENU_BAR_HEIGHT),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::RowReverse,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TopBarMenuText,
                        Text::new(""),
                        TextFont {
                            font_size: TOP_MENU_BAR_HEIGHT - 40.0,
                            font: font_handle.0.clone(),
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        Node {
                            margin: UiRect::horizontal(Val::Px(30.0)),
                            ..default()
                        },
                    ));
                });
        });
}

fn update_top_bar_text(
    query: Query<(&mut Text, &mut UiTransform, &mut TextColor), With<TopBarMenuText>>,
    game_play_timer: Res<GamePlayTimer>,
) {
    for (mut text, mut transform, mut text_colour) in query {
        *text = Text::new(game_play_timer.text());
        let oscillation =
            (game_play_timer.overtime_duration().as_secs_f32() * 8.0 - f32::consts::PI * 0.5).sin()
                * 0.5
                + 0.5;
        transform.scale = Vec2::splat(
            (oscillation * 20.0 + TOP_MENU_BAR_HEIGHT - 40.0) / (TOP_MENU_BAR_HEIGHT - 40.0),
        );
        text_colour.0 = Color::srgb(
            0.8 + oscillation * 0.2,
            0.8 * (1.0 - oscillation),
            0.8 * (1.0 - oscillation),
        );
    }
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontHandle>()
            .add_systems(STARTS_PLAYING, spawn_top_menu)
            .add_systems(Update, update_top_bar_text);
    }
}
