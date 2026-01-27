use bevy::{color::palettes::css::*, prelude::*,
        diagnostic::{FrameCount, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, 
        input::{
            gestures::*,
            mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        },
        window::{
            CursorGrabMode, CursorIcon, CursorOptions, PresentMode, SystemCursorIcon, WindowLevel,
            WindowTheme,
        },
    };
use std::collections::HashMap;
use bevy::prelude::ops::sqrt;

// Resource for storing Mesh Handles
#[derive(Resource, Default)]
struct SceneHandles(HashMap<String, Handle<Scene>>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Dart;

fn main() {
    App::new()
    .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dartman!".into(),
                    name: Some("dartman.app".into()),
                    resolution: (1280, 720).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
        ))
    .init_resource::<SceneHandles>()
    .add_systems(Startup, (startup, load_assets, spawn_player, spawn_camera).chain())
    .add_systems(Update, (player_look, move_player, move_camera, throw_dart, update_darts).chain())
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 180.0)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(5.0, 0.0, 0.0),
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

fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
    mut cursor_moved_reader: MessageReader<CursorMoved>
) {
    // Player is assumed in the center of the screen, want a unit vector from the center of the screen to the mouse
    let center = Vec2::new(640.0, 360.0);
    for cursor in cursor_moved_reader.read() {
        let direction: Vec2 = cursor.position - center;
        let magnitude = sqrt(direction.x.powi(2) + direction.y.powi(2));
        // Now use the unit direction in relation to the player's position, this will make it look in the direction of the mouse
        let unit_direction = Vec2::new(direction.x / magnitude, direction.y / magnitude);
        let look_at_direction = Vec3::new(player.translation.x + unit_direction.x, 0.0, player.translation.z +unit_direction.y);

        player.look_at(look_at_direction, Vec3::Y);
        return;
    }
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let speed = 10.0;
    let mut delta = Vec3::ZERO;
    if key.pressed(KeyCode::KeyA) {
        delta.z += 1.0;
    }
    if key.pressed(KeyCode::KeyD) {
        delta.z -= 1.0;
    }
    if key.pressed(KeyCode::KeyW) {
        delta.x -= 1.0;
    }
    if key.pressed(KeyCode::KeyS) {
        delta.x += 1.0;
    }

    // Move Player
    let mut to_move = delta.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * speed;
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>
) { 
    camera.translation = player.translation + Vec3::new(0.0, 30.0, 0.0);
    camera.look_at(player.translation, Vec3::Y);
}

fn throw_dart(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
    scene_handles: Res<SceneHandles>
) {
    for mouse_button_input in mouse_button_input_reader.read() {
        if mouse_button_input.state.is_pressed() && mouse_button_input.button == MouseButton::Left {
            let mut dart_transform = player.clone();
            dart_transform.scale *= Vec3::new(0.2, 0.2, 0.2);
            dart_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.0));

            commands.spawn((
                SceneRoot(scene_handles.0.get("Dart").unwrap().clone()),
                Transform::from(dart_transform),
                Dart
            ));
        }
    }
}

fn update_darts(
    mut darts: Query<&mut Transform, With<Dart>>,
    time: Res<Time>,
) {
    for mut dart in darts.iter_mut() {
        let dart_forward = dart.forward().as_vec3();
        let dart_speed: f32 = 20.0;
        dart.translation +=  dart_forward * dart_speed * time.delta_secs();
    }
}