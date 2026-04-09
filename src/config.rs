pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const TRAY_WIDTH: f32 = 400.0; // トレイ床の幅
pub const TRAY_FLOOR_Y: f32 = -200.0;
pub const WALL_THICKNESS: f32 = 10.0;
pub const RIGHT_WALL_THICKNESS: f32 = 50.0;
pub const TRAY_DEPTH: f32 = 480.0; // トレイの奥行き(Z方向)

pub const PUSHER_WIDTH: f32 = 400.0; // プッシャーはトレイ床の2倍幅
pub const PUSHER_HEIGHT: f32 = WALL_THICKNESS * 4.0;
pub const PUSHER_Y: f32 = TRAY_FLOOR_Y + WALL_THICKNESS / 2.0 + PUSHER_HEIGHT / 2.0 + 2.0;
pub const PUSHER_MAX_RATIO: f32 = 0.62; // 最大突出率（1.0でトレイ全体を覆う）
pub const PUSHER_SPEED: f32 = 0.8; // プッシャーの往復速度（progress/s）

pub const MEDAL_RADIUS: f32 = 14.0;
pub const MEDAL_HEIGHT: f32 = 5.0;
pub const MEDAL_SPAWN_SPEED: f32 = 300.0; // 搬出速度

// コインスロット（壁の投入口）サイズ
pub const SLOT_WIDTH: f32 = MEDAL_HEIGHT + 8.0; // X方向：メダル薄さ+余裕
pub const SLOT_HEIGHT: f32 = MEDAL_RADIUS * 2.0 + 8.0; // Y方向：メダル直径+余裕
pub const SLOT_CENTER_X: f32 = 0.0; // スロットのX中心（トレイ中央）
// スロット下端 = プッシャー上端 → 中心Y = プッシャー上端 + SLOT_HEIGHT/2
pub const SLOT_CENTER_Y: f32 = PUSHER_Y + PUSHER_HEIGHT / 2.0 + SLOT_HEIGHT / 2.0;

pub const MEDAL_SCORE: u32 = 10;
pub const KILL_Y: f32 = -WINDOW_HEIGHT - 200.0; // 十分に落下してから消える

pub const RIGHT_WALL_HEIGHT: f32 = 80.0;
