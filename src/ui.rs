use bevy::prelude::*;

use crate::{
    constants::TOP_MENU_BAR_HEIGHT,
    game_state::GameState,
    world_entities::{InGameEntity, RenderedAreaWidth},
};

fn spawn_top_menu(mut commands: Commands, rendered_area_width: Res<RenderedAreaWidth>) {
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
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Top Menu Bar"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.7, 1.0)),
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ));
                });
        });
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_top_menu);
    }
}
