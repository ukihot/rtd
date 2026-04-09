mod config;
mod hud;
mod medal;
mod pusher;
mod scoring;
mod tray;

use avian3d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::config::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RTD".into(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(Gravity(Vec3::new(0.0, -600.0, 0.0)))
        .add_plugins((
            scoring::ScoringPlugin,
            tray::TrayPlugin,
            pusher::PusherPlugin,
            medal::MedalPlugin,
            hud::HudPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 900.0,
            },
            near: -2000.0,
            far: 2000.0,
            ..OrthographicProjection::default_3d()
        }),
        // 斜め上から俰瞰、プッシャー付近にフォーカス
        Transform::from_xyz(300.0, 500.0, 600.0).looking_at(Vec3::new(0.0, PUSHER_Y, 0.0), Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::new(0.0, PUSHER_Y, 0.0),
            ..default()
        },
    ));

    // ディレクショナルライト（太陽光）
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));

    // アンビエントライト（環境光）
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        affects_lightmapped_meshes: false,
    });
}
