use avian2d::prelude::*;
use bevy::prelude::*;

use crate::config::*;

pub struct TrayPlugin;

impl Plugin for TrayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tray);
    }
}

fn setup_tray(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall_color = materials.add(Color::srgb(0.35, 0.25, 0.15));
    let floor_color = materials.add(Color::srgb(0.4, 0.3, 0.2));

    // 床
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(TRAY_WIDTH, WALL_THICKNESS))),
        MeshMaterial2d(floor_color),
        Transform::from_xyz(0.0, TRAY_FLOOR_Y, 0.0),
        RigidBody::Static,
        Collider::rectangle(TRAY_WIDTH, WALL_THICKNESS),
        Friction::new(0.6),
    ));

    // 左壁（高い）
    let left_wall_height = 350.0;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, left_wall_height))),
        MeshMaterial2d(wall_color.clone()),
        Transform::from_xyz(
            -TRAY_WIDTH / 2.0,
            TRAY_FLOOR_Y + left_wall_height / 2.0,
            0.0,
        ),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, left_wall_height),
    ));

    // 右壁（低い＝落下エッジ）
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, RIGHT_WALL_HEIGHT))),
        MeshMaterial2d(wall_color),
        Transform::from_xyz(
            TRAY_WIDTH / 2.0,
            TRAY_FLOOR_Y + RIGHT_WALL_HEIGHT / 2.0,
            0.0,
        ),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, RIGHT_WALL_HEIGHT),
    ));

    // 落下ゾーン表示
    let drop_zone_color = materials.add(Color::srgba(1.0, 0.84, 0.0, 0.15));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(60.0, 30.0))),
        MeshMaterial2d(drop_zone_color),
        Transform::from_xyz(TRAY_WIDTH / 2.0 + 20.0, TRAY_FLOOR_Y - 10.0, 0.0),
    ));
}
