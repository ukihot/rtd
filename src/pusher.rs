use avian2d::prelude::*;
use bevy::prelude::*;

use crate::config::*;

pub struct PusherPlugin;

impl Plugin for PusherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pusher)
            .add_systems(Update, pusher_movement);
    }
}

// ---------- Component ----------

#[derive(Component)]
pub struct Pusher {
    direction: f32,
    origin_x: f32,
}

impl Pusher {
    pub fn new(origin_x: f32) -> Self {
        Self {
            direction: 1.0,
            origin_x,
        }
    }

    /// 純粋ロジック: 現在位置から速度ベクトルを計算し、方向を更新する
    pub fn compute_velocity(&mut self, current_x: f32) -> Vec2 {
        if current_x > self.origin_x + PUSHER_RANGE {
            self.direction = -1.0;
        } else if current_x < self.origin_x - PUSHER_RANGE {
            self.direction = 1.0;
        }
        Vec2::new(self.direction * PUSHER_SPEED, 0.0)
    }
}

// ---------- Systems (thin: query → delegate → apply) ----------

fn spawn_pusher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pusher_color = materials.add(Color::srgb(0.6, 0.6, 0.7));
    let pusher_x = -TRAY_WIDTH / 4.0;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PUSHER_WIDTH, PUSHER_HEIGHT))),
        MeshMaterial2d(pusher_color),
        Transform::from_xyz(pusher_x, PUSHER_Y, 1.0),
        RigidBody::Kinematic,
        Collider::rectangle(PUSHER_WIDTH, PUSHER_HEIGHT),
        Friction::new(0.4),
        Pusher::new(pusher_x),
    ));
}

fn pusher_movement(mut query: Query<(&mut LinearVelocity, &Transform, &mut Pusher)>) {
    for (mut vel, tf, mut pusher) in &mut query {
        let v = pusher.compute_velocity(tf.translation.x);
        vel.x = v.x;
        vel.y = v.y;
    }
}
