use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayingState {
    #[default]
    Paused,
    Playing,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct RightPaddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct CourtLine;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct InitialTransform(pub Vec3);

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub computer: u32,
}

#[derive(Event)]
pub struct RoundEnd {
    pub winner: PaddleSide,
}

#[derive(PartialEq)]
pub enum PaddleSide {
    Left,
    Right,
}

pub struct PongGamePlugin;

impl Plugin for PongGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayingState>()
            .init_resource::<Score>()
            .add_event::<RoundEnd>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    spawn_game_elements,
                    spawn_player,
                    spawn_pause_text,
                    spawn_score_text,
                ),
            )
            .add_systems(
                Update,
                (handle_pause_key, handle_escape_key)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (move_player, move_ball, move_ai_paddle, handle_round_end)
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayingState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_game);
    }
}

const COURT_HEIGHT: f32 = 300.0; // Increased from 250.0

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-600., 0., 1.)),
        InitialTransform(Vec3::new(-600., 0., 1.)),
        Player,
        LeftPaddle,
    ));
}

fn spawn_game_elements(mut commands: Commands) {
    // Spawn center line
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(2.0, 600.0)), // Thin vertical line
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        CourtLine,
    ));

    // Spawn top and bottom walls
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(1200.0, 2.0)), // Thin horizontal line
            ..default()
        },
        Transform::from_translation(Vec3::new(0., COURT_HEIGHT, 0.)),
        CourtLine,
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(1200.0, 2.0)), // Thin horizontal line
            ..default()
        },
        Transform::from_translation(Vec3::new(0., -COURT_HEIGHT, 0.)),
        CourtLine,
    ));

    // Spawn ball
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 20.0)), // Square ball
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        InitialTransform(Vec3::new(0., 0., 1.)),
        Ball,
        Velocity {
            direction: Vec2::new(-1.0, 0.25).normalize(),
            speed: 550.0,
        },
    ));

    // Spawn right paddle
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(600., 0., 1.)),
        InitialTransform(Vec3::new(600., 0., 1.)),
        RightPaddle,
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<
        &mut Transform,
        (With<LeftPaddle>, Without<RightPaddle>, Without<Ball>),
    >,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 500.;
    let movement = Vec3::new(
        0.,
        actions.player_movement.unwrap().y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        let new_y = (player_transform.translation.y + movement.y).clamp(
            -COURT_HEIGHT + 50.0, // Half of paddle height (100/2)
            COURT_HEIGHT - 50.0,  // Half of paddle height (100/2)
        );
        player_transform.translation.y = new_y;
    }
}

fn check_paddle_collision(ball_transform: &Transform, paddle_transform: &Transform) -> bool {
    let ball_size = Vec2::new(20.0, 20.0);
    let paddle_size = Vec2::new(20.0, 100.0);

    let ball_min =
        ball_transform.translation - Vec3::new(ball_size.x / 2.0, ball_size.y / 2.0, 0.0);
    let ball_max =
        ball_transform.translation + Vec3::new(ball_size.x / 2.0, ball_size.y / 2.0, 0.0);

    let paddle_min =
        paddle_transform.translation - Vec3::new(paddle_size.x / 2.0, paddle_size.y / 2.0, 0.0);
    let paddle_max =
        paddle_transform.translation + Vec3::new(paddle_size.x / 2.0, paddle_size.y / 2.0, 0.0);

    ball_min.x <= paddle_max.x
        && ball_max.x >= paddle_min.x
        && ball_min.y <= paddle_max.y
        && ball_max.y >= paddle_min.y
}

fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Transform, (Or<(With<LeftPaddle>, With<RightPaddle>)>, Without<Ball>)>,
    mut round_end_events: EventWriter<RoundEnd>,
) {
    for (mut transform, mut velocity) in &mut ball_query {
        let movement_vec = velocity.direction * velocity.speed * time.delta_secs();
        transform.translation.x += movement_vec.x;
        transform.translation.y += movement_vec.y;

        // Bounce off top and bottom walls
        if transform.translation.y.abs() > COURT_HEIGHT {
            transform.translation.y = transform.translation.y.signum() * COURT_HEIGHT;
            velocity.direction.y *= -1.0;
        }

        // Check for paddle collisions
        for paddle_transform in &paddle_query {
            if check_paddle_collision(&transform, paddle_transform) {
                // Move the ball out of the paddle
                let paddle_x = paddle_transform.translation.x;
                let ball_x = transform.translation.x;
                let ball_size = 20.0;
                let paddle_size = 20.0;

                // Move ball to the edge of the paddle based on which side it hit
                if ball_x < paddle_x {
                    transform.translation.x = paddle_x - (paddle_size / 2.0 + ball_size / 2.0);
                } else {
                    transform.translation.x = paddle_x + (paddle_size / 2.0 + ball_size / 2.0);
                }

                velocity.direction.x *= -1.0;
                // Add a slight vertical angle based on where the ball hits the paddle
                let relative_intersect_y =
                    (transform.translation.y - paddle_transform.translation.y) / 50.0;
                velocity.direction.y = relative_intersect_y.clamp(-0.8, 0.8);
                velocity.direction = velocity.direction.normalize();
                break;
            }
        }

        // Check if ball left the screen
        if transform.translation.x > 600.0 {
            round_end_events.send(RoundEnd {
                winner: PaddleSide::Left,
            });
        } else if transform.translation.x < -600.0 {
            round_end_events.send(RoundEnd {
                winner: PaddleSide::Right,
            });
        }
    }
}

fn move_ai_paddle(
    time: Res<Time>,
    ball_query: Query<&Transform, (With<Ball>, Without<LeftPaddle>, Without<RightPaddle>)>,
    mut paddle_query: Query<
        &mut Transform,
        (With<RightPaddle>, Without<LeftPaddle>, Without<Ball>),
    >,
) {
    if let Ok(ball_transform) = ball_query.get_single() {
        let ball_y = ball_transform.translation.y;
        let ai_speed = 250.0;

        for mut paddle_transform in &mut paddle_query {
            let target_y = ball_y;
            let current_y = paddle_transform.translation.y;
            let diff = target_y - current_y;

            // Move towards the ball's y position
            let movement = diff.signum() * ai_speed * time.delta_secs();
            let new_y = (current_y + movement).clamp(
                -COURT_HEIGHT + 50.0, // Half of paddle height (100/2)
                COURT_HEIGHT - 50.0,  // Half of paddle height (100/2)
            );
            paddle_transform.translation.y = new_y;
        }
    }
}

fn handle_escape_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn handle_pause_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<PlayingState>>,
    mut next_state: ResMut<NextState<PlayingState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match current_state.get() {
            PlayingState::Paused => next_state.set(PlayingState::Playing),
            PlayingState::Playing => next_state.set(PlayingState::Paused),
        }
    }
}

fn spawn_pause_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Play/Pause (space)"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        PauseText,
    ));
}

fn spawn_score_text(mut commands: Commands, score: Res<Score>) {
    // Player score text
    commands.spawn((
        Text::new(format!("Player - {}", score.player)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(COURT_HEIGHT + 20.0),
            left: Val::Px(300.0), // Positioned to the left of center
            ..default()
        },
        ScoreText,
    ));

    // Computer score text
    commands.spawn((
        Text::new(format!("Computer - {}", score.computer)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(COURT_HEIGHT + 20.0),
            right: Val::Px(300.0), // Positioned to the right of center
            ..default()
        },
        ScoreText,
    ));
}

fn handle_round_end(
    mut round_end_events: EventReader<RoundEnd>,
    mut dynamic_elements: Query<
        (&mut Transform, &InitialTransform),
        Or<(With<Ball>, With<LeftPaddle>, With<RightPaddle>)>,
    >,
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<PlayingState>>,
    mut score_text_query: Query<(&mut Text, &Node), With<ScoreText>>,
) {
    for event in round_end_events.read() {
        // Update score
        match event.winner {
            PaddleSide::Left => score.player += 1,
            PaddleSide::Right => score.computer += 1,
        }

        // Update score text
        for (mut text, node) in &mut score_text_query {
            if node.left == Val::Px(300.0) {
                // Player score (left side)
                text.0 = format!("Player - {}", score.player);
            } else {
                // Computer score (right side)
                text.0 = format!("Computer - {}", score.computer);
            }
        }

        // Reset all dynamic elements to their initial positions
        for (mut transform, initial) in &mut dynamic_elements {
            transform.translation = initial.0;
        }

        // Reset ball velocity
        if let Some(mut velocity) = ball_query.get_single_mut().ok() {
            velocity.direction = Vec2::new(-1.0, 0.25).normalize();
        }

        // Pause game
        next_state.set(PlayingState::Paused);
    }
}

fn cleanup_game(
    mut commands: Commands,
    game_entities: Query<
        Entity,
        Or<(
            With<Ball>,
            With<LeftPaddle>,
            With<RightPaddle>,
            With<Player>,
            With<CourtLine>,
            With<PauseText>,
            With<ScoreText>,
        )>,
    >,
) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
