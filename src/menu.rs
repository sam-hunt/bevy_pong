use crate::game::Score;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (handle_button_click, handle_menu_escape_key)
                    .chain()
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component, Clone)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct ScoreText;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>, score: Res<Score>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("Bevy Pong"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            let button_colors = ButtonColors::default();

            // Score display
            children
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },))
                .with_children(|score_children| {
                    score_children.spawn((
                        Text::new(format!("Player - {}", score.player)),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScoreText,
                    ));
                    score_children.spawn((
                        Text::new(format!("Computer - {}", score.computer)),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScoreText,
                    ));
                });

            // Play button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    ButtonColors::default(),
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));

            // Reset Score button (only shown if there are points to reset)
            if score.player > 0 || score.computer > 0 {
                children
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(210.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..Default::default()
                        },
                        BackgroundColor(button_colors.normal),
                        ButtonColors::default(),
                        ResetScore,
                    ))
                    .with_child((
                        Text::new("Reset score"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
            }

            // Quit button
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(210.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    ButtonColors::default(),
                    QuitGame,
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/sam-hunt/bevy_pong"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

#[derive(Component)]
struct QuitGame;

#[derive(Component)]
struct ResetScore;

fn handle_menu_escape_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

fn handle_button_click(
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut score: ResMut<Score>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
            Option<&QuitGame>,
            Option<&ResetScore>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link, quit_game, reset_score) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                } else if quit_game.is_some() {
                    app_exit_events.send(AppExit::Success);
                } else if reset_score.is_some() {
                    score.player = 0;
                    score.computer = 0;
                    // Update score text directly
                    for (i, mut text) in score_text_query.iter_mut().enumerate() {
                        if i == 0 {
                            text.0 = format!("Player - {}", score.player);
                        } else {
                            text.0 = format!("Computer - {}", score.computer);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
