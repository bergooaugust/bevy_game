use::bevy::prelude::*;
use bevy::{input::keyboard::KeyboardInput, transform::{self, commands}};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, update_player)
    .add_systems(Update, update_positions)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("player.png"),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Player,
        Velocity::default(),
        )
    );
}

fn update_player(
    mut player_query: Query<&mut Velocity, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
){
    for mut velocity in &mut player_query {
        if kb.pressed(KeyCode::ArrowUp){
            velocity.y = 500.0;
        }
        if kb.pressed(KeyCode::ArrowDown){
            velocity.y = -500.0;
        }
        if kb.pressed(KeyCode::ArrowLeft){
            velocity.x = -500.0;
        }
        if kb.pressed(KeyCode::ArrowRight){
            velocity.x = 500.0;
        }
    }
}

fn update_positions(
    mut player_query: Query<(&mut Transform, &Velocity)>, 
    time: Res<Time>,
){
    for mut tfv in &mut player_query {
        tfv.0.translation.x += tfv.1.x * time.delta_seconds();
        tfv.0.translation.y += tfv.1.y * time.delta_seconds();
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec2);