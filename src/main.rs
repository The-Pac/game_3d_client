mod player;
mod screen_ui;
mod client_network;


use std::f32::consts::PI;
use std::net::IpAddr;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::winit::WinitSettings;
use bevy_rapier3d::geometry::Collider;
use crate::client_network::ClientNetworkPlugin;
use crate::player::PlayerPlugin;
use crate::screen_ui::{ScreenUIPlugin};

pub const MAP_SIZE: f32 = 500.0;
pub const MAP_HEIGHT: f32 = 200.0;
pub const SERVER_ADDRESS: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 4_000;
pub const PROTOCOL_ID: u64 = 1_000;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 1000.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(ScreenUIPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ClientNetworkPlugin)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: 0.1,
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 10.0
        })),
        material: materials.add(Color::rgb(0., 0., 0.).into()),
        transform: Transform::from_xyz(0., 5., 0.),
        ..default()
    });

    commands
        .spawn()
        .insert(Collider::cuboid(MAP_SIZE, 0.1, MAP_SIZE))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: MAP_SIZE
        })),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });
}