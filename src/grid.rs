use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::{AppState, HEIGHT, util, WIDTH};
use crate::loading::Textures;
use crate::util::Palette;
use crate::util::size::LEFT_MARGIN;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor)
                // .with_system(update)
            )
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct GridUI;

pub struct Tile {
    pub bg: Palette,
    pub fg: Palette,
    pub index: usize,
    pub flip: (bool, bool),
    pub rotation: u8,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            bg: Palette::Black,
            fg: Palette::White,
            index: 0,
            flip: (false, false),
            rotation: 0,
        }
    }
}

#[derive(Resource)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    pub tiles: HashMap<(usize, usize), (Tile, Entity)>,
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let mut tiles: HashMap<(usize, usize), (Tile, Entity)> = HashMap::new();

    for y in 0..util::size::GRID_Y {
        for x in 0..util::size::GRID_X {
            let id = commands
                .spawn(TextModeSpriteSheetBundle {
                    sprite: TextModeTextureAtlasSprite {
                        bg: Color::BLACK,
                        fg: Color::WHITE,
                        alpha: 1.,
                        index: 0,
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    texture_atlas: textures.mrmotext.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            LEFT_MARGIN + (WIDTH - LEFT_MARGIN - 8. * util::size::GRID_X as f32) / 2. + 8. * x as f32,
                             HEIGHT - (HEIGHT - 8. * util::size::GRID_Y as f32) / 2. - 8. * y as f32,
                            util::z::GRID
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(GridUI)
                .id();
            tiles.insert((x, y), (Tile::default(), id));
        }
    }

    commands.insert_resource(Grid {
        w: util::size::GRID_X,
        h: util::size::GRID_Y,
        tiles,
    });
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GridUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}