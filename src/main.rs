use bevy::{color::palettes::css::*, prelude::*};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, startup)
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
        MeshMaterial3d(materials.add(Color::from(RED))),
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
        Transform::from_xyz(4.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}