use avian3d::prelude::*;
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_color = materials.add(Color::srgb(0.35, 0.25, 0.15));
    let floor_color = materials.add(Color::srgb(0.4, 0.3, 0.2));

    // 床
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(TRAY_WIDTH, WALL_THICKNESS, TRAY_DEPTH))),
        MeshMaterial3d(floor_color),
        Transform::from_xyz(0.0, TRAY_FLOOR_Y, 0.0),
        RigidBody::Static,
        Collider::cuboid(TRAY_WIDTH, WALL_THICKNESS, TRAY_DEPTH),
        Friction::new(0.6),
    ));

    // 右壁 — トレイ床の右端に立ち、プッシャー上面から浮かせる（プッシャーがその下をくぐる）
    let side_wall_height = 350.0;
    let side_wall_bottom = PUSHER_Y + PUSHER_HEIGHT / 2.0;
    let right_wall_x = TRAY_WIDTH / 2.0 + RIGHT_WALL_THICKNESS / 2.0;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            RIGHT_WALL_THICKNESS,
            side_wall_height,
            TRAY_DEPTH,
        ))),
        MeshMaterial3d(wall_color.clone()),
        Transform::from_xyz(right_wall_x, side_wall_bottom + side_wall_height / 2.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(RIGHT_WALL_THICKNESS, side_wall_height, TRAY_DEPTH),
    ));

    // 奥壁・手前壁の幅: トレイ左端から右壁右端まで
    let right_wall_right = right_wall_x + RIGHT_WALL_THICKNESS / 2.0;
    let tray_left = -TRAY_WIDTH / 2.0;
    let front_back_width = right_wall_right - tray_left;
    let front_back_center_x = (tray_left + right_wall_right) / 2.0;

    // コインスロット（投入口）の位置計算
    let slot_center_y = SLOT_CENTER_Y;
    let slot_top = slot_center_y + SLOT_HEIGHT / 2.0;
    let slot_bottom = slot_center_y - SLOT_HEIGHT / 2.0;
    let slot_left = SLOT_CENTER_X - SLOT_WIDTH / 2.0;
    let slot_right = SLOT_CENTER_X + SLOT_WIDTH / 2.0;

    let wall_bottom_y = side_wall_bottom;
    let wall_top_y = side_wall_bottom + side_wall_height;

    // 壁を4パーツに分割してスロット穴を作る（奥壁・手前壁それぞれ）
    for z_sign in [-1.0_f32, 1.0] {
        let wall_z = z_sign * (TRAY_DEPTH / 2.0 + WALL_THICKNESS / 2.0);

        // 下部: フル幅、壁底〜スロット下端
        let bottom_h = slot_bottom - wall_bottom_y;
        if bottom_h > 0.0 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(front_back_width, bottom_h, WALL_THICKNESS))),
                MeshMaterial3d(wall_color.clone()),
                Transform::from_xyz(front_back_center_x, wall_bottom_y + bottom_h / 2.0, wall_z),
                RigidBody::Static,
                Collider::cuboid(front_back_width, bottom_h, WALL_THICKNESS),
            ));
        }

        // 上部: フル幅、スロット上端〜壁頂
        let top_h = wall_top_y - slot_top;
        if top_h > 0.0 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(front_back_width, top_h, WALL_THICKNESS))),
                MeshMaterial3d(wall_color.clone()),
                Transform::from_xyz(front_back_center_x, slot_top + top_h / 2.0, wall_z),
                RigidBody::Static,
                Collider::cuboid(front_back_width, top_h, WALL_THICKNESS),
            ));
        }

        // 左部: 壁左端〜スロット左端、スロット高さ帯
        let left_w = slot_left - tray_left;
        if left_w > 0.0 {
            let left_cx = tray_left + left_w / 2.0;
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(left_w, SLOT_HEIGHT, WALL_THICKNESS))),
                MeshMaterial3d(wall_color.clone()),
                Transform::from_xyz(left_cx, slot_center_y, wall_z),
                RigidBody::Static,
                Collider::cuboid(left_w, SLOT_HEIGHT, WALL_THICKNESS),
            ));
        }

        // 右部: スロット右端〜壁右端、スロット高さ帯
        let right_w = right_wall_right - slot_right;
        if right_w > 0.0 {
            let right_cx = slot_right + right_w / 2.0;
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(right_w, SLOT_HEIGHT, WALL_THICKNESS))),
                MeshMaterial3d(wall_color.clone()),
                Transform::from_xyz(right_cx, slot_center_y, wall_z),
                RigidBody::Static,
                Collider::cuboid(right_w, SLOT_HEIGHT, WALL_THICKNESS),
            ));
        }
    }
}
