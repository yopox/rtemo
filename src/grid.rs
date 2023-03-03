use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::{AppState, HEIGHT, util, WIDTH};
use crate::loading::Textures;
use crate::mouse::Clickable;
use crate::quick_tiles::Selection;
use crate::toolbar::SelectedTool;
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

#[derive(Component)]
pub struct TilePos(pub (usize, usize));

#[derive(Resource)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    pub tiles: HashMap<(usize, usize), (Tile, Entity)>,
}

#[derive(Component)]
struct HoverTile;

#[derive(Resource)]
pub struct HoverTileIndexOverride {
    pub index: usize,
    pub visible: bool,
    pub force_x: Option<usize>,
    pub force_y: Option<usize>,
}

pub struct GridChanged(pub Vec<(usize, usize)>);

fn grid_x(x: usize) -> f32 { return -4. + LEFT_MARGIN + (WIDTH - LEFT_MARGIN - 8. * util::size::GRID_X as f32) / 2. + 8. * x as f32 }
fn grid_y(y: usize) -> f32 { return -8. + HEIGHT - (HEIGHT - 8. * util::size::GRID_Y as f32 - util::size::BOTTOM_MARGIN) / 2. - 8. * y as f32 }

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
                            grid_x(x),
                            grid_y(y),
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
                    hover_click: true,
                })
                // .insert(TilePos((x, y)))
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

    // Hover tile
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
    tool: Res<SelectedTool>,
    keys: Res<Input<KeyCode>>,
    index_override: Option<Res<HoverTileIndexOverride>>,
    mut hover_tile: Query<(&mut TextModeTextureAtlasSprite, &mut Visibility, &mut Transform), With<HoverTile>>,
    hovered: Query<&Transform, (With<crate::mouse::Hover>, With<GridUI>, Without<HoverTile>)>
) {
    if let Ok((mut tile, mut visibility, mut position)) = hover_tile.get_single_mut() {
        let mut new_vis = true;
        let mut force_x = None;
        let mut force_y = None;

        if let Some(index) = index_override {
            tile.index = index.index;
            new_vis = index.visible;
            force_x = index.force_x;
            force_y = index.force_y;
        } else {
            tile.index = if tool.0.contains(crate::tools::ERASER_TOOL) { 0 } else { selection.index };
        }

        tile.bg = selection.bg.color();
        tile.fg = selection.fg.color();

        visibility.is_visible = false;
        if !keys.pressed(KeyCode::LShift) {
            for pos in hovered.iter() {
                visibility.is_visible = new_vis;
                position.translation.x = pos.translation.x;
                position.translation.y = pos.translation.y;
                break;
            }
        }

        if let (Some(x), Some(y)) = (force_x, force_y) {
            visibility.is_visible = new_vis;
            position.translation.x = grid_x(x);
            position.translation.y = grid_y(y);
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