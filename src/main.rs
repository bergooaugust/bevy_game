use bevy::prelude::*;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

const PLAYER_MOVEMENT_SPEED: f32 = 50.0;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(PixelCameraPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, player_controls)
    .add_systems(Update, apply_velocity)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawn PixelCamera
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize { 
            width: 320, height: 180
        },
        PixelViewport,
    ));

    //Spawn player
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player,
        Velocity::default(),
        BoundingBox{
            size_x: 8.0, 
            size_y: 18.0, 
            ..default()},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -20.0, 0.0),
                scale: Vec3::new(100.0, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
        BoundingBox{
            size_x: 10.0, 
            size_y: 100.0, 
            min_point: Vec2::new(-50.0, -25.0),
            max_point: Vec2::new(50.0, -15.0)
        },
    ));
}

fn player_controls(
    mut player_query: Query<&mut Velocity, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
){
    for mut velocity in &mut player_query {
        let mut x_changed: bool = false;
        let mut y_changed: bool = false;
        if kb.pressed(KeyCode::ArrowUp){
            velocity.y = PLAYER_MOVEMENT_SPEED;
            y_changed = true;
        }
        if kb.pressed(KeyCode::ArrowDown){
            velocity.y = -PLAYER_MOVEMENT_SPEED;
            y_changed = true;
        }
        if kb.pressed(KeyCode::ArrowLeft){
            velocity.x = -PLAYER_MOVEMENT_SPEED;
            x_changed = true;
        }
        if kb.pressed(KeyCode::ArrowRight){
            velocity.x = PLAYER_MOVEMENT_SPEED;
            x_changed = true;
        }
        if !x_changed {
            velocity.x = 0.0;
        }
        if !y_changed {
            velocity.y = 0.0;
        }
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &mut BoundingBox)>, 
    mut bb_query: Query<&BoundingBox, Without<Velocity>>,
    time: Res<Time>,
){
    for mut tfv in &mut query {
        let prev_trans = tfv.0.translation.clone();
        tfv.0.translation.x += tfv.1.x * time.delta_seconds();
        tfv.0.translation.y += tfv.1.y * time.delta_seconds();
        update_bounding_boxes(&mut tfv.2, &tfv.0);
        for bb in &mut bb_query{
            if check_bb_overlap(& tfv.2, &bb) {
                println!("Detected!");
                tfv.0.translation = prev_trans;
                break;
            }
        }
    }
}

fn check_bb_overlap(box_1: &BoundingBox, box_2: &BoundingBox) -> bool
{
    if box_1 == box_2 
    {
        return false;
    }
    if box_1.max_point.x < box_2.min_point.x || box_1.min_point.x > box_2.max_point.x
    {
        return false; 
    }  
    if box_1.max_point.y < box_2.min_point.y || box_1.min_point.y > box_2.max_point.y
    {
        return false; 
    }
    return true;
}

fn update_bounding_boxes(bb: &mut BoundingBox, tf: &Transform)
{
    bb.min_point.x = tf.translation.x - bb.size_x / 2.0;
    bb.min_point.y = tf.translation.y - bb.size_y / 2.0;
    bb.max_point.x = tf.translation.x + bb.size_x / 2.0;
    bb.max_point.x = tf.translation.y + bb.size_y / 2.0;
}

#[derive(Component, Default, PartialEq)]
struct BoundingBox{
    size_x: f32,
    size_y: f32,
    min_point: Vec2,
    max_point: Vec2,
}

#[derive(Component)]
struct Player;

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec2);