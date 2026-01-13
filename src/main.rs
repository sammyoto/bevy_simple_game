use bevy::{color::palettes::css::*, prelude::*};
use bevy::gltf::Gltf;

/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (startup, load_asset))
    .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Red Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
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

fn load_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dart_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/dart.glb"),
    );

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands.spawn((
        Mesh3d(dart_handle),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}