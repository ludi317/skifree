use bevy::prelude::*;
use rand::Rng;

const SKIER_SPEED: f32 = 200.0;
const TREE_COUNT: usize = 50;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Component)]
struct Skier;

#[derive(Component)]
struct Tree;

#[derive(Resource)]
struct GameState {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState { velocity: Vec2::ZERO })
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_trees)
        .add_systems(Update, (skier_movement, move_trees))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let skier_texture = asset_server.load("skier.png");
    commands.spawn((
        SpriteBundle {
            texture: skier_texture,
            transform: Transform::from_xyz(0.0, -SCREEN_HEIGHT / 2.5, 0.0),
            ..default()
        },
        Skier,
    ));
}

fn spawn_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tree_texture = asset_server.load("tree.png");
    let mut rng = rand::thread_rng();

    for _ in 0..TREE_COUNT {
        let x = rng.gen_range(-SCREEN_WIDTH / 2.0..SCREEN_WIDTH / 2.0);
        let y = rng.gen_range(0.0..SCREEN_HEIGHT);

        commands.spawn((
            SpriteBundle {
                texture: tree_texture.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Tree,
        ));
    }
}

fn skier_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Skier>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut velocity = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.x += 1.0;
        }
        game_state.velocity = velocity * SKIER_SPEED;
        transform.translation.x += game_state.velocity.x * time.delta_seconds();
    }
}

fn move_trees(
    mut query: Query<&mut Transform, With<Tree>>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y += SKIER_SPEED * time.delta_seconds();
        if transform.translation.y > SCREEN_HEIGHT / 2.0 {
            transform.translation.y = -SCREEN_HEIGHT / 2.0;
        }
    }
}
