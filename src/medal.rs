use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    config::*,
    scoring::{MedalScored, MedalSpawned},
};

pub struct MedalPlugin;

impl Plugin for MedalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DropCooldown>()
            .add_systems(Update, (drop_medal, check_medal_fall));
    }
}

// ---------- Components ----------

#[derive(Component)]
pub struct Medal;

impl Medal {
    /// スコア判定: 左壁を越えて落下したか
    pub fn is_scoring_position(pos: Vec3) -> bool {
        pos.x < -TRAY_WIDTH / 2.0 - SCORING_MARGIN && pos.y < SCORING_MAX_Y
    }

    /// 場外判定: KILL_Y以下に落ちたか
    pub fn is_out_of_bounds(pos: Vec3) -> bool {
        pos.y < KILL_Y
    }
}

// ---------- Resource ----------

#[derive(Resource)]
pub struct DropCooldown(Timer);

impl Default for DropCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.3, TimerMode::Once))
    }
}

impl DropCooldown {
    pub fn tick(&mut self, delta: std::time::Duration) {
        self.0.tick(delta);
    }

    pub fn ready(&self) -> bool {
        self.0.is_finished()
    }

    pub fn consume(&mut self) {
        self.0.reset();
    }
}

// ---------- Systems (thin: orchestrate only) ----------

fn drop_medal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<DropCooldown>,
    time: Res<Time>,
    mut spawned: MessageWriter<MedalSpawned>,
) {
    cooldown.tick(time.delta());

    // h: 奥壁(Z-)から搬入、l: 手前壁(Z+)から搬入
    // スポーン位置: 壁の外側面ギリギリ（壁厚分だけオフセット、埋もれない）
    let (z, vel_z) = if keyboard.just_pressed(KeyCode::KeyH) && cooldown.ready() {
        (
            -TRAY_DEPTH / 2.0 - WALL_THICKNESS / 2.0 - MEDAL_RADIUS,
            MEDAL_SPAWN_SPEED,
        )
    } else if keyboard.just_pressed(KeyCode::KeyL) && cooldown.ready() {
        (
            TRAY_DEPTH / 2.0 + WALL_THICKNESS / 2.0 + MEDAL_RADIUS,
            -MEDAL_SPAWN_SPEED,
        )
    } else {
        return;
    };

    cooldown.consume();

    let slot_center_y = SLOT_CENTER_Y;

    let gold = materials.add(Color::srgb(1.0, 0.84, 0.0));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(MEDAL_RADIUS, MEDAL_HEIGHT))),
        MeshMaterial3d(gold),
        // メダルを縦にする（90°回転でコイン投入口スタイル）
        Transform::from_xyz(SLOT_CENTER_X, slot_center_y, z)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        RigidBody::Dynamic,
        Collider::cylinder(MEDAL_RADIUS, MEDAL_HEIGHT),
        Mass(20.0),
        Restitution::new(0.3),
        Friction::new(0.35),
        LinearVelocity(Vec3::new(0.0, 0.0, vel_z)),
        Medal,
    ));
    spawned.write(MedalSpawned);
}

fn check_medal_fall(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Medal>>,
    mut scored: MessageWriter<MedalScored>,
) {
    for (entity, tf) in &query {
        let pos = tf.translation;
        if Medal::is_scoring_position(pos) {
            scored.write(MedalScored {
                points: MEDAL_SCORE,
            });
            commands.entity(entity).despawn();
        } else if Medal::is_out_of_bounds(pos) {
            commands.entity(entity).despawn();
        }
    }
}
