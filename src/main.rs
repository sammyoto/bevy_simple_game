use bevy::math::VectorSpace;
use bevy::{color::palettes::css::*, prelude::*};
use bevy::gltf::Gltf;
use bevy::scene::prelude::*;
use std::collections::HashMap;
use bevy::input::mouse::AccumulatedMouseMotion;

// Resource for storing Mesh Handles
#[derive(Resource, Default)]
struct SceneHandles(HashMap<String, Handle<Scene>>);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<SceneHandles>()
    .add_systems(Startup, (startup, load_assets, spawn_player.after(load_assets)))
    .add_systems(Update, (update_camera))
    .run();
}

fn startup(
    mut commands: Commands,
) {
    // Light
    commands.spawn(DirectionalLight::default());
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}

fn spawn_player( 
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let player: Handle<Scene> = assets.load("models/dartman.glb#Scene0");
    commands.spawn(SceneRoot(player));
}

fn load_assets(
    assets: Res<AssetServer>,
    mut scene_handles: ResMut<SceneHandles>,
) {
   let goblin: Handle<Scene> = assets.load("models/goblin.glb#Scene0");
   scene_handles.0.insert("Goblin".to_string(), goblin);
   let dart: Handle<Scene> = assets.load("models/dart.glb#Scene0");
   scene_handles.0.insert("Dart".to_string(), dart);
}

fn update_camera(
    mut camera_transform: Query<&mut Transform, With<Camera3d>>
) {
    camera_transform.single_mut().unwrap().rotate_around(Vec3::ZERO, Quat::from_rotation_y(0.02));
}