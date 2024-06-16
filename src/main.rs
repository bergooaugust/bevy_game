use std::default;

use bevy::{app::MainScheduleOrder, ecs::schedule::{ExecutorKind, ScheduleLabel}, prelude::*};
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

const PLAYER_MOVEMENT_SPEED: f32 = 50.0;
const GRAVITY: f32 = 400.0;
const TERMINAL_VELOCITY: f32 = 200.0;


fn main() {
    let mut app = App::new();

    let mut controls_schedule = Schedule::new(PostUpdate);
    controls_schedule.set_executor_kind(ExecutorKind::MultiThreaded);

    app.add_schedule(controls_schedule);

    let mut main_schedule_order = app.world.resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_after(Update, PostUpdate);

    app
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(PixelCameraPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, player_controls)
    .add_systems(PostUpdate, apply_velocity)
    .add_systems(Update, static_response)
    .add_systems(Update, animate_player)
    .add_systems(Update, sprite_direction)
    .run();
}

fn setup(mut commands: Commands, 
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
    //Spawn PixelCamera
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize { 
            width: 220, height: 120
        },
        PixelViewport,
    ));

    //Player animation stuff
    let texture = asset_server.load("player-sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(10.0, 10.0), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    //Spawn player
    commands.spawn((
        SpriteSheetBundle{
            texture,
            atlas: TextureAtlas{
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        },
        Player{
            collidingX: CollisionNormalX::Neutral,
            collidingY: CollisionNormalY::Neutral,
            state: SpriteState::Neutral
        },
        Velocity::default(),
        Acceleration(Vec2::new(0.0, -GRAVITY)),
        BoundingBox{
            size_x: 10.0, 
            size_y: 10.0, 
            ..default()},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                scale: Vec3::new(200.0, 100.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
        BoundingBox{
            size_x: 200.0, 
            size_y: 100.0, 
            min_point: Vec2::new(-100.0, -150.0),
            max_point: Vec2::new(100.0, -50.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 60.0, 0.0),
                scale: Vec3::new(200.0, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::GREEN,
                ..default()
            },
            ..default()
        },
        BoundingBox{
            size_x: 200.0, 
            size_y: 10.0, 
            min_point: Vec2::new(-100.0, 55.0),
            max_point: Vec2::new(100.0, 65.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-105.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 120.0, 0.0),
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
            size_y: 120.0, 
            min_point: Vec2::new(-110.0, -60.0),
            max_point: Vec2::new(-100.0, 60.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(105.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 120.0, 0.0),
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
            size_y: 120.0, 
            min_point: Vec2::new(100.0, -60.0),
            max_point: Vec2::new(110.0, 60.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(20.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::ORANGE_RED,
                ..default()
            },
            ..default()
        },
        BoundingBox{
            size_x: 50.0, 
            size_y: 10.0, 
            min_point: Vec2::new(-5.0, -5.0),
            max_point: Vec2::new(45.0, 5.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-20.0, -20.0, 0.0),
                scale: Vec3::new(50.0, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::ORANGE_RED,
                ..default()
            },
            ..default()
        },
        BoundingBox{
            size_x: 50.0, 
            size_y: 10.0, 
            min_point: Vec2::new(-45.0, -25.0),
            max_point: Vec2::new(5.0, -15.0)
        },
        Static,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::new(500.0, 500.0, 0.0),
                ..default()
            },
            sprite: Sprite{
                color: Color::rgb(0.0, 0.5, 0.8),
                ..default()
            },
            ..default()
        },
    ));
}

fn player_controls(
    mut player_query: Query<(&mut Velocity, &mut Player, &mut Acceleration)>,
    kb: Res<ButtonInput<KeyCode>>,
){
    for mut player in &mut player_query {
        let mut x_change: f32 = 0.0;
        let mut y_change: f32 = 0.0;
        if player.1.collidingY == CollisionNormalY::Down {
            if kb.pressed(KeyCode::ArrowUp){
                y_change += 200.0;
            }
            if kb.pressed(KeyCode::ArrowDown){
                y_change -= 20.0;
            }
            if kb.pressed(KeyCode::ArrowLeft){
                x_change -= PLAYER_MOVEMENT_SPEED;
            }
            if kb.pressed(KeyCode::ArrowRight){
                x_change += PLAYER_MOVEMENT_SPEED;
            }
            if x_change != 0.0 {
                player.1.state = SpriteState::Walking;
            } else {
                player.1.state = SpriteState::Neutral;
            }
            player.0.x = x_change;
            player.0.y = y_change;
            
        } else {
            player.1.state = SpriteState::InAir;
            if kb.pressed(KeyCode::ArrowLeft){
                player.2.x = -0.5 * PLAYER_MOVEMENT_SPEED;
            }
            else if kb.pressed(KeyCode::ArrowRight){
                player.2.x = 0.5 * PLAYER_MOVEMENT_SPEED;
            } else {
                player.2.x = 0.0;
            }
        }
    }
}

fn animate_player(
    mut sprite_query: Query<(&Player, &mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
){
    for (player, mut atlas, mut timer) in &mut sprite_query{
        timer.tick(time.delta());
        if timer.just_finished() {
            match player.state {
                SpriteState::Neutral => {
                    atlas.index = 0;
                },
                SpriteState::InAir => {
                    atlas.index = (atlas.index + 1) % 2 + 2;
                },
                SpriteState::Walking => {
                    atlas.index = (atlas.index + 1) % 2;
                }

            }
        }
    }
}

fn sprite_direction(
    mut sprite_query: Query<(&Velocity, &mut Sprite), With<Player>>
){
    for (velocity, mut sprite) in &mut sprite_query{
        if velocity.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.x > 0.0 {
            sprite.flip_x = false;
        }
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &mut BoundingBox)>, 
    time: Res<Time>,
){
    for mut tfv in &mut query {
        tfv.1.x += tfv.2.x * time.delta_seconds();
        tfv.1.y += tfv.2.y * time.delta_seconds();

        tfv.0.translation.x += tfv.1.x * time.delta_seconds();
        tfv.0.translation.y += tfv.1.y * time.delta_seconds();
        update_bounding_boxes(&mut tfv.3, &tfv.0);
    }
}

fn static_response(
    mut static_query: Query<&BoundingBox, With<Static>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut BoundingBox, &mut Player), (Without<Static>, Changed<Transform>)>, 

) {
    let mut new_col_normal_x: CollisionNormalX = CollisionNormalX::Neutral;
    let mut new_col_normal_y: CollisionNormalY = CollisionNormalY::Neutral;

    for mut tfv in &mut query {  
        for bb in &mut static_query{
            if check_bb_overlap(& tfv.2, &bb) {
                let pv = penetration_vector(& tfv.2, &bb);
                if pv.x != 0.0 {
                    tfv.0.translation.x -= pv.x;
                    tfv.1.x = 0.0;
                    new_col_normal_x = if pv.x < 0.0 {
                        CollisionNormalX::Left
                    } else {
                        CollisionNormalX::Right
                    }
                }
                if pv.y != 0.0 {
                    tfv.0.translation.y -= pv.y;
                    tfv.1.y = 0.0;
                    new_col_normal_y = if pv.y < 0.0 {
                        CollisionNormalY::Down
                    } else {
                        CollisionNormalY::Up
                    } 
                }
            }         
        }
        tfv.3.collidingX = new_col_normal_x;
        tfv.3.collidingY = new_col_normal_y;


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

#[derive(Default, PartialEq, Clone, Copy)]
enum CollisionNormalY{
    #[default]
    Neutral,
    Up,
    Down,
}
#[derive(Default, PartialEq, Clone, Copy)]
enum CollisionNormalX{
    #[default]
    Neutral,
    Left,
    Right,
}
#[derive(Default, PartialEq)]
enum SpriteState{
    #[default]
    Neutral,
    Walking,
    InAir,
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct PostUpdate;

#[derive(Component, Default, PartialEq)]
struct BoundingBox{
    size_x: f32,
    size_y: f32,
    min_point: Vec2,
    max_point: Vec2,
}

#[derive(Component)]
struct Player{
    collidingX: CollisionNormalX,
    collidingY: CollisionNormalY,
    state: SpriteState,
}

#[derive(Component)]
struct Static;

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Default, Deref, DerefMut)]
struct Acceleration(Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);