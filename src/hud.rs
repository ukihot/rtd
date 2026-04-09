use bevy::prelude::*;

use crate::{pusher::Pusher, scoring::GameState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud).add_systems(
            Update,
            (
                update_score_text,
                update_medal_count_text,
                update_pusher_info_text,
            ),
        );
    }
}

// ---------- Components (HUD内部のみ) ----------

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MedalCountText;

#[derive(Component)]
struct PusherInfoText;

// ---------- Systems ----------

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("Medals: 0"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(42.0),
            left: Val::Px(10.0),
            ..default()
        },
        MedalCountText,
    ));

    commands.spawn((
        Text::new("Coverage: 0% | Speed: 0 px/s"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.9, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            ..default()
        },
        PusherInfoText,
    ));

    commands.spawn((
        Text::new("H : Left  |  L : Right"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn update_score_text(state: Res<GameState>, mut query: Query<&mut Text, With<ScoreText>>) {
    if state.is_changed() {
        for mut text in &mut query {
            **text = format!("Score: {}", state.score);
        }
    }
}

fn update_medal_count_text(
    state: Res<GameState>,
    mut query: Query<&mut Text, (With<MedalCountText>, Without<ScoreText>)>,
) {
    if state.is_changed() {
        for mut text in &mut query {
            **text = format!("Medals: {}", state.medals_dropped);
        }
    }
}

fn update_pusher_info_text(
    pusher_query: Query<&Pusher>,
    mut text_query: Query<
        &mut Text,
        (
            With<PusherInfoText>,
            Without<ScoreText>,
            Without<MedalCountText>,
        ),
    >,
) {
    if let Ok(pusher) = pusher_query.single() {
        let coverage = pusher.coverage() * 100.0;
        let speed = pusher.speed_info();
        for mut text in &mut text_query {
            **text = format!("Coverage: {coverage:.0}% | Speed: {speed:.0} px/s");
        }
    }
}
