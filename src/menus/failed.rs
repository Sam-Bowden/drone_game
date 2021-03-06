use bevy::prelude::*;
use crate::AppState;

#[derive(Component)]
pub enum MenuButton {
    Retry,
    Return,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Failed",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                });
            parent
                .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                    margin: Rect {
                        top: Val::Percent(4.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Retry",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Retry);
            parent
                .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                    margin: Rect {
                        top: Val::Percent(4.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Return to main menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuButton::Return);
        });
    }

pub fn click(
    mut state: ResMut<State<AppState>>,
    input: Query<(&Interaction, &MenuButton), With<Button>>,
) {
    for (interaction, button) in input.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Retry => state.set(AppState::Game).unwrap(),
                MenuButton::Return => state.set(AppState::MainMenu).unwrap(),
            }
        }
    }
}