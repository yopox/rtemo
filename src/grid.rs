use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::{AppState, HEIGHT, util, WIDTH};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clickable};
use crate::quick_tiles::Selection;
use crate::toolbar::SelectedTool;
use crate::tools::Tools;
use crate::util::Palette;
use crate::util::size::LEFT_MARGIN;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GridChanged>()
            .add_event::<GridResized>()
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_systems(
                (update_hover_tile, update_grid, resize_grid)
                .in_set(OnUpdate(AppState::Editor))
            )
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
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

#[derive(Component)]
pub struct TilePos(pub (usize, usize));

#[derive(Resource)]
pub struct Grid {
    pub x0: isize,
    pub y0: isize,
    pub w: usize,
    pub h: usize,
    pub tiles: HashMap<(isize, isize), (Tile, Entity)>,
}

pub struct GridResized;

#[derive(Component)]
struct HoverTile;

#[derive(Resource)]
pub struct HoverTileIndexOverride {
    pub index: usize,
    pub visible: Visibility,
    pub force_x: Option<isize>,
    pub force_y: Option<isize>,
}

pub struct GridChanged(pub Vec<(isize, isize)>);

fn grid_x(x: isize, x0: isize, w: usize) -> f32 { return -4. + LEFT_MARGIN + (WIDTH - LEFT_MARGIN - 8. * w as f32) / 2. + 8. * (x - x0) as f32 }
fn grid_y(y: isize, y0: isize, h: usize) -> f32 { return -8. + HEIGHT - (HEIGHT - 8. * h as f32 - util::size::BOTTOM_MARGIN) / 2. - 8. * (y - y0) as f32 }

fn setup(
    mut commands: Commands,
    mut grid_resized: EventWriter<GridResized>,
    textures: Res<Textures>,
) {
    let mut tiles: HashMap<(isize, isize), (Tile, Entity)> = HashMap::new();

    commands.insert_resource(Grid {
        x0: 0,
        y0: 0,
        w: util::size::GRID_X,
        h: util::size::GRID_Y,
        tiles,
    });
    grid_resized.send(GridResized);

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
    grid: Res<Grid>,
    index_override: Option<Res<HoverTileIndexOverride>>,
    mut hover_tile: Query<(&mut TextModeTextureAtlasSprite, &mut Visibility, &mut Transform), With<HoverTile>>,
    hovered: Query<&Transform, (With<crate::mouse::Hover>, With<GridUI>, Without<HoverTile>)>
) {
    if let Ok((mut tile, mut visibility, mut position)) = hover_tile.get_single_mut() {
        let mut new_vis = Visibility::Inherited;
        let mut force_x = None;
        let mut force_y = None;

        if let Some(index) = index_override {
            tile.index = index.index;
            new_vis = index.visible;
            force_x = index.force_x;
            force_y = index.force_y;
        } else {
            tile.index = if tool.0 == Tools::Eraser { 0 } else { selection.index };
        }

        tile.bg = selection.bg.color();
        tile.fg = selection.fg.color();

        visibility.set_if_neq(Visibility::Hidden);
        if !keys.pressed(KeyCode::LShift) {
            for pos in hovered.iter() {
                visibility.set_if_neq(new_vis);
                position.translation.x = pos.translation.x;
                position.translation.y = pos.translation.y;
                break;
            }
        }

        if let (Some(x), Some(y)) = (force_x, force_y) {
            visibility.set_if_neq(new_vis);
            position.translation.x = grid_x(x, grid.x0, grid.w);
            position.translation.y = grid_y(y, grid.y0, grid.h);
        }
    }
}

#[derive(Component)]
struct GridTile;

fn resize_grid(
    mut commands: Commands,
    mut grid_resized: EventReader<GridResized>,
    textures: Res<Textures>,
    grid: Option<ResMut<Grid>>,
    mut transform: Query<&mut Transform, With<GridTile>>,
) { let Some(mut grid) = grid else { return; };

    if !grid_resized.is_empty() {
        grid_resized.clear();

        let h = grid.h as isize;
        let w = grid.w as isize;

        // Despawn OOB tiles
        let mut to_remove = Vec::new();
        for (&(x, y), (_, id)) in grid.tiles.iter() {
            if x < grid.x0 || x >= grid.x0 + w || y < grid.y0 || y >= grid.y0 + h {
                commands.entity(*id).despawn_recursive();
                to_remove.push((x, y));
            }
        }
        to_remove.iter().for_each(|i| { grid.tiles.remove(i); });

        // Update tiles positions
        for (&(x, y), (_, id)) in grid.tiles.iter() {
            let Ok(mut transform) = transform.get_mut(*id) else { continue };
            transform.translation.x = grid_x(x, grid.x0, grid.w);
            transform.translation.y = grid_y(y, grid.y0, grid.h);
        }

        // Spawn missing tiles
        for y in grid.y0..(grid.y0 + h) {
            for x in grid.x0..(grid.x0 + w) {
                if grid.tiles.contains_key(&(x, y)) { continue }

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
                                grid_x(x, grid.x0, grid.w),
                                grid_y(y, grid.y0, grid.h),
                                util::z::GRID
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Clickable {
                        w: 8.,
                        h: 8.,
                        id: ButtonId::Grid(x, y),
                        hover_click: true,
                    })
                    .insert(GridUI)
                    .insert(GridTile)
                    .id();
                grid.tiles.insert((x, y), (Tile::default(), id));
            }
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
            let Some((t, e)) = grid.tiles.get(&(x as isize, y as isize)) else { continue };
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