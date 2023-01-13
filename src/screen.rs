use crate::prelude::*;

/// Used for building the walls and lane grid for cars to spawn in.
#[derive(Default)]
pub struct TileScreen {}

impl TileScreen {
    pub fn column_to_coord(column: usize) -> f32 {
        let padding = PADDING as f32;
        let column = column as f32;

        TILE_SIZE.mul_add(
            //(TILE_SIZE * padding) + ...
            padding,
            HALF_TILE.mul_add(
                //(HALF_TILE * 3f32) + ...
                3_f32,
                column.mul_add(COLUMN_SIZE, SCREEN_X),
            ),
        )
    }

    pub const fn tile_scale() -> Vec3 {
        Vec3::new(0.85f32, 0.85f32, 0f32)
    }
}
