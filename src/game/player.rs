use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app // Add event with arena.
            // .add_event::<PlayerSpawnEvent>()
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_player));
        // .add_system_set(SystemSet::on_update(AppState::Game).
        // with_system(movement));
    }
}
pub fn spawn_player(mut commands: Commands) {
    let column = 1;
    let pos_x = TileScreen::column_to_coord(column);

    commands
        .spawn((Car { column }, Player))
        .with_children(draw_car)
        .insert(anchor_sprite(pos_x, PLAYER_Y));
}
