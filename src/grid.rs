use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::{AppState, HEIGHT, util, WIDTH};
use crate::loading::Textures;
use crate::mouse::Clickable;
use crate::quick_tiles::Selection;
use crate::util::Palette;
use crate::util::size::LEFT_MARGIN;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GridChanged>()
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor)
                .with_system(update_hover_tile)
                .with_system(update_grid)
            )
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

pub const PREFIX: &str = "grid";

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

#[derive(Component)]
struct HoverTile;

pub struct GridChanged(pub Vec<(usize, usize)>);

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
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    texture_atlas: textures.mrmotext.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            -4. + LEFT_MARGIN + (WIDTH - LEFT_MARGIN - 8. * util::size::GRID_X as f32) / 2. + 8. * x as f32,
                            -4. + HEIGHT - (HEIGHT - 8. * util::size::GRID_Y as f32) / 2. - 8. * y as f32,
                            util::z::GRID
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Clickable {
                    w: 8.,
                    h: 8.,
                    id: format!("{PREFIX}_{x}_{y}"),
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

    // Quick tile
    commands
        .spawn(TextModeSpriteSheetBundle {
            sprite: TextModeTextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.mrmotext.clone(),
            transform: Transform::from_xyz(0., 0., util::z::GRID_HOVER),
            ..Default::default()
        })
        .insert(HoverTile)
        .insert(GridUI);
}

fn update_hover_tile(
    selection: Res<Selection>,
    mut hover_tile: Query<(&mut TextModeTextureAtlasSprite, &mut Visibility, &mut Transform), With<HoverTile>>,
    hovered: Query<&Transform, (With<crate::mouse::Hover>, With<GridUI>, Without<HoverTile>)>
) {
    if let Ok((mut tile, mut visibility, mut position)) = hover_tile.get_single_mut() {
        tile.index = selection.index;
        tile.bg = selection.bg.color();
        tile.fg = selection.fg.color();

        visibility.is_visible = false;
        for pos in hovered.iter() {
            visibility.is_visible = true;
            position.translation.x = pos.translation.x;
            position.translation.y = pos.translation.y;
            break;
        }
    }
}

fn update_grid(
    mut update: EventReader<GridChanged>,
    grid: Res<Grid>,
    mut tile: Query<&mut TextModeTextureAtlasSprite>,
) {
    for GridChanged(vec) in update.iter() {
        for &(x, y) in vec.iter() {
            let Some((t, e)) = grid.tiles.get(&(x, y)) else { continue };
            let Ok(mut grid_tile) = tile.get_mut(*e) else { continue };
            grid_tile.bg = t.bg.color();
            grid_tile.fg = t.fg.color();
            grid_tile.index = t.index;
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GridUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}