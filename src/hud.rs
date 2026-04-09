use bevy::prelude::*;

use crate::scoring::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, (update_score_text, update_medal_count_text));
    }
}

// ---------- Components (HUD内部のみ) ----------

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MedalCountText;

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
        Text::new("← → : Move  |  Space : Drop Medal"),
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
