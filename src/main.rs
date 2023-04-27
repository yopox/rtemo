use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;

use crate::grid::GridPlugin;
use crate::loading::LoadingPlugin;
use crate::mouse::MousePlugin;
use crate::quick_tiles::QuickTilesPlugin;
use crate::toolbar::ToolbarPlugin;
use crate::util::Palette;

mod loading;
mod toolbar;
mod tools;
mod util;
mod mouse;
mod quick_tiles;
mod grid;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Editor,
}

const WIDTH: f32 = 8. * 40.;
const HEIGHT: f32 = 8. * 25.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Palette::D.color()))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WIDTH * 4., HEIGHT * 4.).into(),
                    title: "rtemo".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_state::<AppState>()
        .add_plugin(LoadingPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(ToolbarPlugin)
        .add_plugin(TextModePlugin)
        .add_plugin(MousePlugin)
        .add_plugin(QuickTilesPlugin)
        .add_plugin(tools::ToolsPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.25, 0.25, 1.),
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., 100.),
            ..Default::default()
        },
        ..Default::default()
    });
}