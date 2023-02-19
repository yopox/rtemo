use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;
use crate::loading::LoadingPlugin;
use crate::mouse::MousePlugin;
use crate::quick_tiles::QuickTilesPlugin;
use crate::toolbar::ToolbarPlugin;

mod loading;
mod toolbar;
mod tools;
mod util;
mod mouse;
mod quick_tiles;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Loading,
    Editor,
}

const WIDTH: f32 = 8. * 40.;
const HEIGHT: f32 = 8. * 25.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("100000").unwrap()))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: WIDTH * 4.,
                    height: HEIGHT * 4.,
                    title: "rtemo".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..Default::default()
                },
                ..default()
            })
        )
        .add_state(AppState::Loading)
        .add_plugin(LoadingPlugin)
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