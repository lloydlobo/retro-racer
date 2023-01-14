// Copied and slightly modified from [wichops/bevy_retro_racing](https://github.com/wichops/bevy_retro_racing/blob/main/src/main.rs)
#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_must_use)]
#![deny(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
#![allow(anonymous_parameters)]
#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::cast_precision_loss)]

mod assets;
mod components;
mod config;
mod game;
mod resources;
mod screen;

use bevy::window::PresentMode;
use prelude::*;

fn main() {
    println!("{WINDOW_WIDTH}, {WINDOW_HEIGHT}");
    let mut app = App::new();

    app.init_resource::<Scoreboard>().init_resource::<GameData>();

    app.insert_resource(ClearColor(Color::hex(BG_COLOR).unwrap()));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "retro racer".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..default()
        },
        ..default()
    }));

    app.add_plugin(AssetsPlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(BotPlugin)
        .add_plugin(PlayerPlugin);

    // For dev only. Use AppState::Game to skip StartMenu, AppGameState::Game.
    // Or AppState::GameMenu & AppGameState::Invalid
    app.add_state(AppState::Game).add_state(AppGameState::Game);

    app.add_startup_system(setup_camera).add_startup_system(setup);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    // handles_ui: ResMut<UiAssets>,
    // handles_audio: ResMut<AudioAssets>,
    mut score_resource: ResMut<Scoreboard>,
) {
    let window = windows.primary_mut();
    window.center_window(MonitorSelection::Current);

    commands.insert_resource(ExplosionSound(asset_server.load("sounds/explosion.ogg")));
    commands.insert_resource(MotorSound(asset_server.load("sounds/motor.ogg")));

    // commands.insert_resource(ExplosionSound(handles_audio.car_explosion.
    // clone())); commands.insert_resource(MotorSound(handles_audio.car_motor.
    // clone()));

    let text_style = TextStyle {
        // font: handles_ui.font.clone(),
        font: asset_server.load("fonts/Calculator.ttf"),
        font_size: FONT_SIZE,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::CENTER_RIGHT;

    score_resource.entities.score = Some(
        commands
            .spawn(
                TextBundle::from_sections([
                    TextSection::new("SCORE\n", text_style.clone()),
                    TextSection::new(score_resource.score.to_string(), text_style.clone()),
                ])
                .with_style(Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexEnd,
                    position: UiRect { top: Val::Px(60f32), right: Val::Px(20f32), ..default() },
                    ..default()
                })
                .with_text_alignment(text_alignment),
            )
            .id(),
    );

    score_resource.entities.highscore = Some(
        commands
            .spawn(
                TextBundle::from_sections([
                    TextSection::new("HIGHSCORE\n", text_style.clone()),
                    TextSection::new(score_resource.highscore.to_string(), text_style),
                ])
                .with_style(Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexEnd,
                    position: UiRect { top: Val::Px(120f32), right: Val::Px(20f32), ..default() },
                    ..default()
                })
                .with_text_alignment(text_alignment),
            )
            .id(),
    );

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            commands.spawn(SpriteBundle {
                sprite: Sprite { color: Color::rgba(0., 0., 0., 0.1), ..default() },
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32).mul_add(TILE_SIZE, SCREEN_X) + HALF_TILE,
                        (y as f32).mul_add(TILE_SIZE, SCREEN_Y) + HALF_TILE,
                        0f32,
                    ),
                    scale: Vec3::new(0.8f32, 0.8f32, 1f32),
                    ..default()
                },
                ..default()
            });
        }
    }
}

mod prelude {
    pub use std::{
        ops::Neg,
        time::Duration,
    };

    pub use bevy::prelude::*;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{
        thread_rng,
        Rng,
    };

    pub use crate::{
        assets::*,
        components::*,
        config::*,
        game::*,
        resources::*,
        screen::*,
    };
}
