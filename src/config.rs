pub const BG_COLOR: &str = "8d9e7b";
pub const FONT_SIZE: f32 = 32.;

pub const TILE_SIZE: f32 = 20.;
pub const HALF_TILE: f32 = 20.;

pub const UI_WIDTH: f32 = 120.;

pub const PADDING: usize = 2;

pub const WINDOW_PADDING: f32 = 20.;
pub const WINDOW_WIDTH: f32 = UI_WIDTH + SCREEN_WIDTH as f32 * TILE_SIZE + WINDOW_PADDING * 2.;
pub const WINDOW_HEIGHT: f32 = SCREEN_HEIGHT as f32 * TILE_SIZE + WINDOW_PADDING * 2.;

pub const SCREEN_WIDTH: usize = 9 + PADDING * 2;
pub const SCREEN_HEIGHT: usize = 20;
pub const SCREEN_X: f32 = WINDOW_WIDTH / -2. + WINDOW_PADDING;
pub const SCREEN_Y: f32 = WINDOW_HEIGHT / -2. + WINDOW_PADDING;
