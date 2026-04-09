use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    config::*,
    scoring::{MedalScored, MedalSpawned},
};

pub struct MedalPlugin;

impl Plugin for MedalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DropCooldown>()
            .add_systems(Startup, spawn_indicator)
            .add_systems(
                Update,
                (move_indicator, drop_medal, check_medal_fall, draw_guide),
            );
    }
}

// ---------- Components ----------

#[derive(Component)]
pub struct Medal;

impl Medal {
    /// スコア判定: 右壁を越えて落下したか
    pub fn is_scoring_position(pos: Vec2) -> bool {
        pos.x > TRAY_WIDTH / 2.0 + 20.0 && pos.y < TRAY_FLOOR_Y + RIGHT_WALL_HEIGHT
    }

    /// 場外判定: KILL_Y以下に落ちたか
    pub fn is_out_of_bounds(pos: Vec2) -> bool {
        pos.y < KILL_Y
    }
}

#[derive(Component)]
pub struct DropIndicator;

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

fn spawn_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::srgb(1.0, 1.0, 0.4));
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, -12.0),
            Vec2::new(-8.0, 8.0),
            Vec2::new(8.0, 8.0),
        ))),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, MEDAL_DROP_Y + 20.0, 2.0),
        DropIndicator,
    ));
}

fn move_indicator(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DropIndicator>>,
) {
    if let Ok(mut tf) = query.single_mut() {
        let speed = 200.0;
        if keyboard.pressed(KeyCode::ArrowLeft) {
            tf.translation.x -= speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            tf.translation.x += speed * time.delta_secs();
        }
        tf.translation.x = tf
            .translation
            .x
            .clamp(-MEDAL_DROP_X_RANGE, MEDAL_DROP_X_RANGE);
    }
}

fn drop_medal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<DropCooldown>,
    time: Res<Time>,
    indicator_q: Query<&Transform, With<DropIndicator>>,
    mut spawned: MessageWriter<MedalSpawned>,
) {
    cooldown.tick(time.delta());

    if keyboard.just_pressed(KeyCode::Space) && cooldown.ready() {
        cooldown.consume();
        let drop_x = indicator_q.single().map(|t| t.translation.x).unwrap_or(0.0);

        let gold = materials.add(Color::srgb(1.0, 0.84, 0.0));
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(MEDAL_RADIUS))),
            MeshMaterial2d(gold),
            Transform::from_xyz(drop_x, MEDAL_DROP_Y, 1.0),
            RigidBody::Dynamic,
            Collider::circle(MEDAL_RADIUS),
            Restitution::new(0.3),
            Friction::new(0.8),
            Medal,
        ));
        spawned.write(MedalSpawned);
    }
}

fn check_medal_fall(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Medal>>,
    mut scored: MessageWriter<MedalScored>,
) {
    for (entity, tf) in &query {
        let pos = tf.translation.truncate();
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

fn draw_guide(indicator_q: Query<&Transform, With<DropIndicator>>, mut gizmos: Gizmos) {
    if let Ok(tf) = indicator_q.single() {
        let x = tf.translation.x;
        for y in (TRAY_FLOOR_Y as i32..MEDAL_DROP_Y as i32).step_by(20) {
            gizmos.circle_2d(
                Vec2::new(x, y as f32),
                2.0,
                Color::srgba(1.0, 1.0, 0.4, 0.3),
            );
        }
    }
}
