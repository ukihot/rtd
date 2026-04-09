mod config;
mod hud;
mod medal;
mod pusher;
mod scoring;
mod tray;

use avian2d::prelude::*;
use bevy::prelude::*;

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
        .insert_resource(Gravity(Vec2::new(0.0, -600.0)))
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
    commands.spawn(Camera2d);
}
