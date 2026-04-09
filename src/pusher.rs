use avian3d::prelude::*;
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
    pub position_progress: f32, // 0.0 (最大突出=トレイ被覆) から 1.0 (最大引っ込み=壁側)
}

impl Pusher {
    pub fn new() -> Self {
        Self {
            direction: 1.0,         // 最初は壁側へ引く
            position_progress: 0.0, // 最大突出状態から開始
        }
    }

    /// 現在の被覆率 (0.0〜PUSHER_MAX_RATIO)
    pub fn coverage(&self) -> f32 {
        // progress=0 → 最大被覆(PUSHER_MAX_RATIO), progress=1 → 被覆0
        PUSHER_MAX_RATIO * (1.0 - self.position_progress)
    }

    /// 現在の速度 (px/s)
    pub fn speed_info(&self) -> f32 {
        self.direction * PUSHER_SPEED * Self::max_retract()
    }

    /// 移動ロジック: 位置進行度を更新し、方向を決定
    pub fn update_position(&mut self, delta: f32) {
        self.position_progress += self.direction * PUSHER_SPEED * delta;

        if self.position_progress >= 1.0 {
            self.position_progress = 1.0;
            self.direction = -1.0;
        } else if self.position_progress <= 0.0 {
            self.position_progress = 0.0;
            self.direction = 1.0;
        }
    }

    /// 現在のX位置
    pub fn get_x_position(&self) -> f32 {
        // progress=0: 最大突出（プッシャー左端がトレイ左端に揃う）
        // progress=1: 最大引っ込み（壁側へ引き、トレイ床が露出）
        Self::extended_x() + self.position_progress * Self::max_retract()
    }

    /// 最大引っ込み距離（トレイ床の PUSHER_MAX_RATIO 分だけ露出させる）
    fn max_retract() -> f32 {
        PUSHER_MAX_RATIO * TRAY_WIDTH
    }

    /// 最大突出時のプッシャー中心X
    /// プッシャーは右壁の下から来て左方向へ突出する
    /// PUSHER_MAX_RATIO=0.62 → トレイ床の62%を被覆
    fn extended_x() -> f32 {
        // retracted: プッシャー左端 = トレイ右端 → center = TRAY_WIDTH/2 + PUSHER_WIDTH/2
        // extended: そこから max_retract 分だけ左へ
        TRAY_WIDTH / 2.0 + PUSHER_WIDTH / 2.0 - Self::max_retract()
    }
}

// ---------- Systems (thin: query → delegate → apply) ----------

fn spawn_pusher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pusher_color = materials.add(Color::srgb(0.6, 0.6, 0.7));
    let initial_x = Pusher::extended_x(); // 最大突出位置
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PUSHER_WIDTH, PUSHER_HEIGHT, TRAY_DEPTH))),
        MeshMaterial3d(pusher_color),
        Transform::from_xyz(initial_x, PUSHER_Y, 0.0),
        RigidBody::Kinematic,
        Collider::cuboid(PUSHER_WIDTH, PUSHER_HEIGHT, TRAY_DEPTH),
        LinearVelocity::default(),
        Friction::new(0.4),
        Pusher::new(),
    ));
}

fn pusher_movement(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut LinearVelocity, &mut Pusher)>,
) {
    for (tf, mut vel, mut pusher) in &mut query {
        pusher.update_position(time.delta_secs());
        let target_x = pusher.get_x_position();

        // 現在位置と目標位置の差分から速度を計算
        // Transformは直接書き換えず、LinearVelocityのみで物理エンジンに移動を任せる
        let dt = time.delta_secs();
        if dt > 0.0 {
            vel.x = (target_x - tf.translation.x) / dt;
        }
    }
}
