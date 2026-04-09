use bevy::prelude::*;

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_message::<MedalScored>()
            .add_message::<MedalSpawned>()
            .add_systems(Update, apply_scoring);
    }
}

// ---------- Resource ----------

#[derive(Resource, Default)]
pub struct GameState {
    pub score: u32,
    pub medals_dropped: u32,
}

// ---------- Events ----------

#[derive(Message)]
pub struct MedalScored {
    pub points: u32,
}

#[derive(Message)]
pub struct MedalSpawned;

// ---------- System (thin: just wires events → resource) ----------

fn apply_scoring(
    mut scored: MessageReader<MedalScored>,
    mut spawned: MessageReader<MedalSpawned>,
    mut state: ResMut<GameState>,
) {
    for ev in scored.read() {
        state.score += ev.points;
    }
    for _ in spawned.read() {
        state.medals_dropped += 1;
    }
}
