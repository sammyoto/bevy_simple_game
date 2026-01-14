use bevy::math::VectorSpace;
use bevy::{color::palettes::css::*, prelude::*};
use bevy::gltf::Gltf;
use std::collections::HashMap;
use bevy::input::mouse::AccumulatedMouseMotion;

// Resource for storing Mesh Handles
#[derive(Resource)]
struct MeshHandles(HashMap<String, Handle<Mesh>>);

// Resource for storing Material Handles
#[derive(Resource)]
struct MaterialHandles(HashMap<String, Handle<StandardMaterial>>);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (startup, load_assets))
    .add_systems(Update, (update_camera))
    .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Red Cube
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::default())),
    //     MeshMaterial3d(materials.add(Color::from(BLUE))),
    //     Transform::from_xyz(0.0, 0.0, 0.0)
    // ));
    // Light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(2.0, 3.0, 5.0)
    ));
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}

fn spawn_player(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    commands.spawn((
        Mesh3d(mesh_handles.0.get("Player").unwrap().clone()),
        MeshMaterial3d(material_handles.0.get("Player").unwrap().clone()),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut mesh_handles: ResMut<MeshHandles>,
    mut material_handles: ResMut<MaterialHandles>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player_handle: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0
        }
        .from_asset("models/lowpoly_basic.glb"),
    );

    let dart_handle: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/dart.glb"),
    );

    mesh_handles.0.insert("Player".to_string(), player_handle);
    mesh_handles.0.insert("Dart".to_string(), dart_handle);

    let player_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });
    let dart_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.7, 0.8),
        ..default()
    });

    material_handles.0.insert("Player".to_string(), player_material_handle);
    material_handles.0.insert("Dart".to_string(), dart_material_handle);
}

fn update_camera(
    mut camera_transform: Query<&mut Transform, With<Camera3d>>
) {
    camera_transform.single_mut().unwrap().rotate_around(Vec3::ZERO, Quat::from_rotation_y(0.02));
}