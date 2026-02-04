use bevy::{prelude::*, 
        input::{
            mouse::{MouseButtonInput},
        },
        window::{
            PresentMode,
            WindowTheme,
        },
    };
use std::collections::{HashMap, HashSet};
use bevy::prelude::ops::sqrt;
use rand::prelude::*;

// Resource for storing Mesh Handles
#[derive(Resource, Default)]
struct SceneHandles(HashMap<String, Handle<Scene>>);

#[derive(Resource, Default)]
struct SpawnTimer(f32);

#[derive(Resource, Default)]
struct GameScore {
    player_deaths: u32,
    goblin_kills: u32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Goblin;

#[derive(Component)]
struct Dart;

#[derive(Component)]
struct Arena;

#[derive(Component)]
struct CollisionBox {
    size: Vec3,
}

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
    .init_resource::<SpawnTimer>()
    .init_resource::<GameScore>()
    .add_systems(Startup, (startup, load_assets, spawn_player, spawn_camera).chain())
    .add_systems(Update, 
        (
            player_look, 
            move_player, 
            move_camera, 
            spawn_goblins, 
            update_goblins, 
            throw_dart, 
            update_darts, 
            check_goblin_dart_collision,
            check_goblin_player_collision,
            update_game_stats
        ).chain())
    .run();
}

fn startup(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    assets: Res<AssetServer>,
) {
    // Init Spawn Timer
    spawn_timer.0 = 0.0;
    // Arena
    let arena: Handle<Scene> = assets.load("models/dartman_arena.glb#Scene0");
    commands.spawn((
        SceneRoot(arena),
        Transform::from_xyz(0.0, -5.0, 0.0),
        Arena
    ));
    // Light
    commands.spawn(
        (DirectionalLight
        {  
            illuminance: 9000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        )
    );
    // Text with one section
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Player Deaths: 0\nGoblin Kills: 0"),
        Underline,
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(Justify::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
    ));
}

fn spawn_player( 
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player: Handle<Scene> = assets.load("models/dartman.glb#Scene0");
    commands.spawn((
        SceneRoot(player),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        CollisionBox {
            size: Vec3::new(3.0, 13.0, 3.0),
        },
        Mesh3d(meshes.add(Cuboid::new(3.0, 13.0,3.0))),
        MeshMaterial3d(materials.add(Color::hsla(180.0, 0.5, 0.5, 0.1)))
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
    let to_move = delta.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * speed;
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>
) { 
    camera.translation = player.translation + Vec3::new(0.0, 40.0, 0.0);
    camera.look_at(player.translation, Vec3::Y);
}

fn throw_dart(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
    scene_handles: Res<SceneHandles>,
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

fn spawn_goblins(
    mut commands: Commands,
    scene_handles: Res<SceneHandles>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn goblins every 3 seconds
    if time.elapsed_secs() - spawn_timer.0 <= 3.0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let spawn_number = rng.gen_range(1..10);
    let sides = ["TOP", "BOTTOM", "LEFT", "RIGHT"];

    for _ in 0..spawn_number {
        let side = sides[rng.gen_range(0..sides.len())];
        let mut x: f32 = 0.0;
        let mut z: f32 = 0.0;

        if side == "TOP" {
            x = rng.gen_range(-90.0..90.0);
            z = -90.0
        } else if side == "BOTTOM" {
            x = rng.gen_range(-90.0..90.0);
            z = 90.0
        } else if side == "LEFT" {
            x = -90.0;
            z = rng.gen_range(-90.0..90.0);
        } else if side == "RIGHT" {
            x = 90.0;
            z = rng.gen_range(-90.0..90.0);
        }

        commands.spawn((
            SceneRoot(scene_handles.0.get("Goblin").unwrap().clone()),
            Transform::from_xyz(x, 0.0, z),
            Goblin,
            CollisionBox {
                size: Vec3::new(3.0, 13.0, 3.0),
            },
            Mesh3d(meshes.add(Cuboid::new(3.0, 13.0,3.0))),
            MeshMaterial3d(materials.add(Color::hsla(180.0, 0.5, 0.5, 0.1)))
        ));
    }

    spawn_timer.0 = time.elapsed_secs();
}

fn update_goblins(
    mut goblins: Query<&mut Transform, (With<Goblin>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
    time: Res<Time>
) {
    for mut goblin in goblins.iter_mut() {
        // Look at player
        goblin.look_at(player.translation, Vec3::Y);
        // Move towards player
        let forward = goblin.forward().as_vec3();
        let speed: f32 = 10.0;
        goblin.translation +=  forward * speed * time.delta_secs();
    }
}

fn check_goblin_dart_collision(
    mut commands: Commands,
    goblins: Query<(Entity, &Transform, &CollisionBox), With<Goblin>>,
    darts: Query<(Entity, &Transform), With<Dart>>,
    mut game_score: ResMut<GameScore>,
) {
    let mut goblin_despawn_list: Vec<Entity> = Vec::new();
    let mut dart_despawn_list: Vec<Entity> = Vec::new();

    // Find all colliding goblins and darts
    for (goblin_entity, goblin_transform, goblin_collision) in goblins.iter() {
        for (dart_entity, dart_transform) in darts.iter() {
            if transform_in_volume((goblin_transform, goblin_collision), dart_transform) {
                goblin_despawn_list.push(goblin_entity);
                dart_despawn_list.push(dart_entity);
            }
        }
    }

    let goblin_despawn_set: HashSet<Entity> = goblin_despawn_list.into_iter().collect();
    let dart_despawn_set: HashSet<Entity> = dart_despawn_list.into_iter().collect();

    for goblin_entity in goblin_despawn_set {
        commands.entity(goblin_entity).despawn();
        game_score.goblin_kills += 1;
    }
    for dart_entity in dart_despawn_set {
        commands.entity(dart_entity).despawn();
    }
}

fn check_goblin_player_collision(
    mut commands: Commands,
    goblins: Query<(Entity, &Transform, &CollisionBox), With<Goblin>>,
    player: Single<(&Transform, &CollisionBox), With<Player>>,
    mut game_score: ResMut<GameScore>
) {
    let mut goblin_despawn_list: Vec<Entity> = Vec::new();

    for (goblin_entity, goblin_transform, goblin_collision) in goblins.iter() {
        if volume_in_volume((goblin_transform, goblin_collision), (player.0, player.1)) {
            goblin_despawn_list.push(goblin_entity);
        }
    }

    for goblin_entity in goblin_despawn_list {
        commands.entity(goblin_entity).despawn();
        game_score.player_deaths += 1;
    }
}

fn transform_in_volume(
    volume: (&Transform, &CollisionBox),
    transform: &Transform,
) -> bool {
    // check if within x bounds
    if transform.translation.x < volume.0.translation.x - volume.1.size.x / 2.0 || transform.translation.x > volume.0.translation.x + volume.1.size.x / 2.0 {
        return false;
    // check if within z bounds
    } else if transform.translation.z < volume.0.translation.z - volume.1.size.z / 2.0 || transform.translation.z > volume.0.translation.z + volume.1.size.z / 2.0 {
        return false;
    }
    
    true
}

fn volume_in_volume(
    volume1: (&Transform, &CollisionBox),
    volume2: (&Transform, &CollisionBox),
) -> bool {
    let mut coords_to_check: Vec<Vec3> = Vec::new();

    // UPPER
    // top left upper
    coords_to_check.push(Vec3::new(volume1.0.translation.x - (volume1.1.size.x / 2.0), volume1.0.translation.y, volume1.0.translation.z + (volume1.1.size.z / 2.0)));

    // top right upper
    coords_to_check.push(Vec3::new(volume1.0.translation.x + (volume1.1.size.x / 2.0), volume1.0.translation.y, volume1.0.translation.z + (volume1.1.size.z / 2.0)));

    // bottom left upper
    coords_to_check.push(Vec3::new(volume1.0.translation.x - (volume1.1.size.x / 2.0), volume1.0.translation.y, volume1.0.translation.z - (volume1.1.size.z / 2.0)));

    // bottom right upper
    coords_to_check.push(Vec3::new(volume1.0.translation.x + (volume1.1.size.x / 2.0), volume1.0.translation.y, volume1.0.translation.z - (volume1.1.size.z / 2.0)));

    // LOWER
    // top left lower
    coords_to_check.push(Vec3::new(volume1.0.translation.x - (volume1.1.size.x / 2.0), 0.0, volume1.0.translation.z + (volume1.1.size.z / 2.0)));

    // top right lower
    coords_to_check.push(Vec3::new(volume1.0.translation.x + (volume1.1.size.x / 2.0), 0.0, volume1.0.translation.z + (volume1.1.size.z / 2.0)));

    // bottom left lower
    coords_to_check.push(Vec3::new(volume1.0.translation.x - (volume1.1.size.x / 2.0), 0.0, volume1.0.translation.z - (volume1.1.size.z / 2.0)));

    // bottom right lower
    coords_to_check.push(Vec3::new(volume1.0.translation.x + (volume1.1.size.x / 2.0), 0.0, volume1.0.translation.z - (volume1.1.size.z / 2.0)));

    for coord in coords_to_check {
        let mut transform = volume2.0.clone();
        transform.translation = coord;
        if transform_in_volume(volume2, &transform) {
            return true;
        }
    }

    false
}

fn update_game_stats(
    mut text: Single<&mut Text, With<Text>>,
    game_score: Res<GameScore>,
) {
    text.0 = format!("Player Deaths: {}\n Goblin Kills: {}", game_score.player_deaths, game_score.goblin_kills);
}