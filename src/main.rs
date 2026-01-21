use bevy::math::VectorSpace;
use bevy::{color::palettes::css::*, prelude::*};
use bevy::gltf::Gltf;
use bevy::scene::prelude::*;
use std::collections::HashMap;
use bevy::input::mouse::AccumulatedMouseMotion;

// Resource for storing Mesh Handles
#[derive(Resource, Default)]
struct MeshHandles(HashMap<String, Handle<Mesh>>);

// Resource for storing Material Handles
#[derive(Resource, Default)]
struct MaterialHandles(HashMap<String, Handle<StandardMaterial>>);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<MeshHandles>()
    .init_resource::<MaterialHandles>()
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
    mesh_handles: Res<MeshHandles>,
    assets: Res<AssetServer>,
    material_handles: Res<MaterialHandles>
) {
    let scene: Handle<Scene> = assets.load("models/goblin.glb#Scene0");
    commands.spawn(SceneRoot(scene));
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