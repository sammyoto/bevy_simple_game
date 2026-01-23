use bevy::{color::palettes::css::*, prelude::*};
use std::collections::HashMap;
use bevy::input::mouse::AccumulatedMouseMotion;

// Resource for storing Mesh Handles
#[derive(Resource, Default)]
struct SceneHandles(HashMap<String, Handle<Scene>>);

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct CameraOrbit {
    radius: f32,
    pitch: f32,
    yaw: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        Self {
            radius: 15.0,
            pitch: 0.5,
            yaw: 0.0,
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<SceneHandles>()
    .init_resource::<CameraOrbit>()
    .add_systems(Startup, (startup, load_assets, spawn_player, spawn_camera).chain())
    .add_systems(Update, (move_player, move_camera).chain())
    .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(40.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // Light
    commands.spawn(DirectionalLight::default());
}

fn spawn_player( 
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let player: Handle<Scene> = assets.load("models/dartman.glb#Scene0");
    commands.spawn((
        SceneRoot(player),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player
    ));
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0),
    ));
}

fn load_assets(
    assets: Res<AssetServer>,
    mut scene_handles: ResMut<SceneHandles>,
) {
   let goblin: Handle<Scene> = assets.load("models/goblin.glb#Scene0");
   scene_handles.0.insert("Goblin".to_string(), goblin);
   let dart: Handle<Scene> = assets.load("models/dartman_dart.glb#Scene0");
   scene_handles.0.insert("Dart".to_string(), dart);
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let speed = 50.0;
    let mut delta = Vec3::ZERO;
    if key.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if key.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if key.pressed(KeyCode::KeyW) {
        delta.z += 1.0;
    }
    if key.pressed(KeyCode::KeyS) {
        delta.z -= 1.0;
    }

    // Move Player
    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let mut to_move = forward + right;
    to_move.y = 0.0;
    to_move = to_move.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * speed;
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut orbit: ResMut<CameraOrbit>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    // Update orbit angles from mouse movement
    orbit.yaw -= mouse_motion.delta.x * 0.005;
    orbit.pitch -= mouse_motion.delta.y * 0.005;
    
    // Clamp pitch to prevent flipping
    orbit.pitch = orbit.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
    
    // Calculate camera position
    let cos_pitch = orbit.pitch.cos();
    let x = orbit.radius * orbit.yaw.cos() * cos_pitch;
    let y = orbit.radius * orbit.pitch.sin();
    let z = orbit.radius * orbit.yaw.sin() * cos_pitch;
    
    camera.translation = player.translation + Vec3::new(x, y, z);
    camera.look_at(player.translation + Vec3::new(0.0, 2.0, 0.0), Vec3::Y);
}