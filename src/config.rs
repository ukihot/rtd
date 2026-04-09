pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// ── トレイ基本寸法 ──
pub const TRAY_WIDTH: f32 = 400.0; // トレイ床の幅
pub const TRAY_DEPTH: f32 = 480.0; // トレイの奥行き(Z方向)
pub const TRAY_FLOOR_Y: f32 = -200.0;
pub const WALL_THICKNESS: f32 = 10.0;
pub const RIGHT_WALL_THICKNESS: f32 = 50.0;
pub const SIDE_WALL_HEIGHT: f32 = 350.0; // 側壁（右壁・奥壁・手前壁）の高さ

// ── プッシャー ──
pub const PUSHER_WIDTH: f32 = 400.0;
pub const PUSHER_HEIGHT: f32 = WALL_THICKNESS * 4.0;
pub const PUSHER_Y: f32 = TRAY_FLOOR_Y + WALL_THICKNESS / 2.0 + PUSHER_HEIGHT / 2.0 + 2.0;
pub const PUSHER_MAX_RATIO: f32 = 0.38; // 最大突出率（1.0でトレイ全体を覆う）
pub const PUSHER_SPEED: f32 = 0.6; // プッシャーの往復速度（progress/s）
/// プッシャー上端Y（壁の浮き始め）
pub const PUSHER_TOP_Y: f32 = PUSHER_Y + PUSHER_HEIGHT / 2.0;

// ── メダル ──
pub const MEDAL_RADIUS: f32 = 14.0;
pub const MEDAL_HEIGHT: f32 = 5.0;
pub const MEDAL_SPAWN_SPEED: f32 = 300.0; // 搬出速度

// ── コインスロット（壁の投入口） ──
pub const SLOT_WIDTH: f32 = MEDAL_HEIGHT + 8.0; // X方向：メダル薄さ+余裕
pub const SLOT_HEIGHT: f32 = MEDAL_RADIUS * 2.0 + 8.0; // Y方向：メダル直径+余裕
/// スロットX中心：右壁内側面ギリギリ
pub const SLOT_CENTER_X: f32 = TRAY_WIDTH / 2.0 - SLOT_WIDTH / 2.0 - 2.0;
/// スロット下端 = プッシャー上端
pub const SLOT_CENTER_Y: f32 = PUSHER_TOP_Y + SLOT_HEIGHT / 2.0;

// ── スコアリング ──
pub const MEDAL_SCORE: u32 = 10;
/// スコア判定マージン（左壁の外側にメダル半径分出たら得点）
pub const SCORING_MARGIN: f32 = MEDAL_RADIUS + 6.0;
/// スコア判定の最大Y（壁の浮き高さ以下でのみ判定）
pub const SCORING_MAX_Y: f32 = PUSHER_TOP_Y;
/// 場外消滅Y
pub const KILL_Y: f32 = TRAY_FLOOR_Y - 600.0;
