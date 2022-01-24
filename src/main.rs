use bevy::{prelude::*};

#[derive(Component)]
struct SnakeHead;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x:i32,
    y:i32,
}

#[derive(Component)]
struct Size {
    width:f32,
    height:f32,
}

fn main() {
    App::new()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_snake)
    .add_system(snake_movement)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands){
    commands
        .spawn_bundle(SpriteBundle{
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform{
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}

fn snake_movement(keyboard_input: Res<Input<KeyCode>>, mut head_positions: Query<&mut Transform, With<SnakeHead>>,) {
    for mut transform in head_positions.iter_mut(){
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }    
}