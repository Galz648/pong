//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Player {
    id: i16,
    score: i8,
}

#[derive(Component)]
struct Ball {}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn update() {
    println!("");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..Default::default()
        })
        .insert(Ball {});

    // Rectangle 1 (Move to the left, narrow)
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(25.0, 100.0)), // Narrow the width
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 100.0, 0.0)), // Move to the left and up
            ..Default::default()
        })
        .insert(Player { id: 1, score: 0 });

    // Rectangle 2 (Move to the right, narrow)
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(25.0, 100.0)), // Narrow the width
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(300.0, -100.0, 0.0)), // Move to the right and down
            ..Default::default()
        })
        .insert(Player { id: 2, score: 0 });
}
