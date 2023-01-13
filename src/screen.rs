use crate::prelude::*;

/// Used for building the walls and lane grid for cars to spawn in.
#[derive(Default)]
pub struct TileScreen {}

impl TileScreen {
    pub fn column_to_coord(column: usize) -> f32 {
        let padding = PADDING as f32;
        let column = column as f32;

        SCREEN_X + (column * COLUMN_SIZE) + (HALF_TILE * 3.) + (TILE_SIZE * padding)
    }

    pub fn tile_scale() -> Vec3 {
        Vec3::new(0.85f32, 0.85f32, 0f32)
    }
}
