use bevy::prelude::*;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

const PLAYER_MOVEMENT_SPEED: f32 = 50.0;
const GRAVITY: f32 = 400.0;
const TERMINAL_VELOCITY: f32 = 200.0;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(PixelCameraPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, player_controls)
    .add_systems(Update, apply_physics)
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
        Acceleration(Vec2::new(0.0, -GRAVITY)),
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
        let mut x_change: f32 = 0.0;
        let mut y_change: f32 = 0.0;
        if kb.pressed(KeyCode::ArrowUp) && velocity.y == 0.0{
            y_change = 200.0;
        }
        if kb.pressed(KeyCode::ArrowDown){
            y_change = -20.0;
        }
        if kb.pressed(KeyCode::ArrowLeft){
            x_change -= PLAYER_MOVEMENT_SPEED;
        }
        if kb.pressed(KeyCode::ArrowRight){
            x_change += PLAYER_MOVEMENT_SPEED;
        }
        if y_change != 0.0 {
            velocity.y = y_change;
        }
        velocity.x = x_change;
    }
}

fn apply_physics(
    mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &mut BoundingBox)>, 
    mut bb_query: Query<&BoundingBox, Without<Velocity>>,
    time: Res<Time>,
){
    for mut tfv in &mut query {
        tfv.1.x += tfv.2.x * time.delta_seconds();
        tfv.1.y += tfv.2.y * time.delta_seconds();

        tfv.0.translation.x += tfv.1.x * time.delta_seconds();
        tfv.0.translation.y += tfv.1.y * time.delta_seconds();

        update_bounding_boxes(&mut tfv.3, &tfv.0);
        for bb in &mut bb_query{
            if check_bb_overlap(& tfv.3, &bb) {
                let pv = penetration_vector(& tfv.3, &bb);
                if pv.x != 0.0 {
                    tfv.0.translation.x -= pv.x;
                    tfv.1.x = 0.0;
                }
                if pv.y != 0.0 {
                    tfv.0.translation.y -= pv.y;
                    tfv.1.y = 0.0;
                }
                break;
            }
        }
    }
}

fn penetration_vector(box_1: &BoundingBox, box_2: &BoundingBox) -> Vec2
{
    let mut min_dist = (box_1.max_point.x - box_2.min_point.x).abs();
    let mut v = Vec2::new(min_dist, 0.0);


    if (box_1.max_point.y - box_2.min_point.y).abs() < min_dist
    {
        min_dist = (box_1.max_point.y - box_2.min_point.y).abs();
        v = Vec2::new(0.0, min_dist);
    }

    if (box_1.min_point.y - box_2.max_point.y).abs() < min_dist
    {
        min_dist = (box_1.min_point.y - box_2.max_point.y).abs();
        v = Vec2::new(0.0, -min_dist);
    }

    if (box_1.min_point.x - box_2.max_point.x).abs() < min_dist
    {
        min_dist = (box_1.min_point.x - box_2.max_point.x).abs();
        v = Vec2::new(-min_dist, 0.0);
    }

    return v;
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
    //println!("Collision detected, own max x and y: {} {}, other max: {} {}", box_1.max_point.x, box_1.max_point.y, box_2.max_point.x, box_2.max_point.y);
    return true;
}

fn update_bounding_boxes(bb: &mut BoundingBox, tf: &Transform)
{
    bb.min_point.x = tf.translation.x - bb.size_x / 2.0;
    bb.min_point.y = tf.translation.y - bb.size_y / 2.0;
    bb.max_point.x = tf.translation.x + bb.size_x / 2.0;
    bb.max_point.y = tf.translation.y + bb.size_y / 2.0;
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

#[derive(Component, Default, Deref, DerefMut)]
struct Acceleration(Vec2);