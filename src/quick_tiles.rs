use std::slice::SliceIndex;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use strum::IntoEnumIterator;

use crate::{AppState, util};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clickable, Clicked, Hover};
use crate::toolbar::SelectedTool;
use crate::tools::Tools;
use crate::util::Palette;

pub struct QuickTilesPlugin;

impl Plugin for QuickTilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectTile>()
            .add_event::<SelectColor>()
            .add_event::<QuickTilesChanged>()
            .insert_resource(Selection {
                index: util::misc::DEFAULT_TILE,
                bg: Palette::E,
                fg: Palette::A,
                flip: false,
                rotation: 0,
            })
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_systems(
                (update, on_click,
                 update_tiles_index, update_active_tile,
                 update_colors, update_range)
                    .in_set(OnUpdate(AppState::Editor))
            )
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

#[derive(Component)]
struct QuickTilesUI;

#[derive(Component)]
struct QuickTile;

#[derive(Component)]
struct ColorButton;

#[derive(Component)]
struct QuickTileId {
    index: usize,
    tile: usize,
    entity: Entity,
}

#[derive(Resource)]
struct QuickTiles(Vec<QuickTileId>);

struct QuickTilesChanged;

#[derive(Component)]
struct ActiveTile;

pub struct SelectTile(pub usize);
pub struct SelectColor(pub Palette, pub bool);

#[derive(Resource)]
pub struct Selection {
    pub index: usize,
    pub bg: Palette,
    pub fg: Palette,
    pub flip: bool,
    pub rotation: u8,
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    // Quick tiles
    let mut tiles = Vec::new();
    let dx = 56.;
    let per_row = util::misc::QUICK_TILES_PER_ROW;
    let rows = util::misc::QUICK_TILES_ROWS;
    for i in 0..(per_row * rows) {
        let id = commands
            .spawn(TextModeSpriteSheetBundle {
                sprite: TextModeTextureAtlasSprite {
                    bg: Color::BLACK,
                    fg: Color::WHITE,
                    alpha: 0.5,
                    index: i,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.mrmotext.clone(),
                transform: Transform::from_xyz(dx + (i % per_row) as f32 * 8., (8. * rows as f32) - (i / per_row) as f32 * 8., util::z::TOOLBAR_ICONS),
                ..Default::default()
            })
            .insert(Clickable {
                w: 8.,
                h: 8.,
                id: ButtonId::QuickTile(i),
                hover_click: true,
            })
            .insert(QuickTile)
            .insert(QuickTilesUI)
            .id();
        tiles.push(QuickTileId { index: i, tile: i, entity: id, });
    }

    commands.insert_resource(QuickTiles(tiles));

    // Active tile
    commands
        .spawn(TextModeSpriteSheetBundle {
            sprite: TextModeTextureAtlasSprite {
                bg: Palette::E.color(),
                fg: Palette::A.color(),
                alpha: 1.,
                index: util::misc::DEFAULT_TILE,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.mrmotext.clone(),
            transform: Transform {
                translation: Vec3::new(32., 16., util::z::TOOLBAR),
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ActiveTile)
        .insert(QuickTilesUI);

    // Palette
    for (i, color) in util::Palette::iter().enumerate() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: color.color(),
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(8. + 8. * (i % 2) as f32, 72. - 8. * (i / 2) as f32, 0.),
                texture: textures.color.clone(),
                ..Default::default()
            })
            .insert(Clickable {
                w: 8.,
                h: 8.,
                id: ButtonId::QuickColor(color),
                hover_click: false,
            })
            .insert(ColorButton)
            .insert(QuickTilesUI);
    }
}

fn update(
    mut hover_tiles: Query<(&mut TextModeTextureAtlasSprite, Option<&Hover>), With<QuickTile>>,
    mut hover_colors: Query<(&mut Sprite, Option<&Hover>), With<ColorButton>>,
) {
    for (mut sprite, h) in hover_tiles.iter_mut() {
        sprite.alpha = if h.is_some() { 1.0 } else { 0.6 };
    }
    for (mut sprite, h) in hover_colors.iter_mut() {
        sprite.color.set_a(if h.is_some() { 0.8 } else { 1.0 });
    }
}

fn on_click(
    quick_tiles: Res<QuickTiles>,
    mut clicked: EventReader<Clicked>,
    mut select_tile: EventWriter<SelectTile>,
    mut select_color: EventWriter<SelectColor>,
    mut selection: ResMut<Selection>,
) {
    for Clicked(id, right) in clicked.iter() {
        match id {
            ButtonId::QuickTile(n) => {
                let Some(quick_tile) = quick_tiles.0.iter().find(|tile| tile.index == *n) else { continue };
                selection.index = quick_tile.tile;
                select_tile.send(SelectTile(quick_tile.tile));
            }
            ButtonId::QuickColor(color) => {
                if *right { selection.bg = *color; }
                else { selection.fg = *color; }
                select_color.send(SelectColor(*color, *right));
            }
            _ => ()
        }
    }
}

fn update_range(
    keys: Res<Input<KeyCode>>,
    tool: Res<SelectedTool>,
    mut tiles: ResMut<QuickTiles>,
    mut changed: EventWriter<QuickTilesChanged>,
) {
    if tool.0 == Tools::Resize { return; }
    let up = keys.just_pressed(KeyCode::Up);
    let down = keys.just_pressed(KeyCode::Down);
    if !up && !down { return; }

    let count = util::misc::QUICK_TILES_PER_ROW * util::misc::QUICK_TILES_ROWS;
    if up {
        tiles.0.iter_mut().for_each(|tile| tile.tile = (tile.tile + util::misc::TILESET_COUNT - count) % util::misc::TILESET_COUNT);
    } else {
        tiles.0.iter_mut().for_each(|tile| tile.tile = (tile.tile + count) % util::misc::TILESET_COUNT);
    }
    changed.send(QuickTilesChanged);
}

fn update_tiles_index(
    mut tiles_changed: EventReader<QuickTilesChanged>,
    mut tiles: ResMut<QuickTiles>,
    mut tile_query: Query<&mut TextModeTextureAtlasSprite, With<QuickTilesUI>>,
) {
    for _ in tiles_changed.iter() {
        for QuickTileId { index, tile, entity } in tiles.0.iter() {
            let Ok(mut t) = tile_query.get_mut(*entity) else { continue };
            t.index = *tile;
        }
        break
    }
    tiles_changed.clear();
}

fn update_active_tile(
    keys: Res<Input<KeyCode>>,
    mut selection: ResMut<Selection>,
    mut select_tile: EventReader<SelectTile>,
    mut tile: Query<&mut TextModeTextureAtlasSprite, With<ActiveTile>>,
    mut window: Query<&mut Window>,
) {
    let mut tile = tile.single_mut();

    if keys.just_pressed(KeyCode::LControl) {
        selection.flip = !selection.flip;
        tile.flip_x = selection.flip;
    } else if keys.just_pressed(KeyCode::LAlt) {
        selection.rotation = (selection.rotation + 1) % 4;
        tile.rotation = selection.rotation;
    }

    for SelectTile(i) in select_tile.iter() {
        tile.index = *i;
        tile.flip_x = selection.flip;
        tile.rotation = selection.rotation;
    }

    window.single_mut().title = format!("rtemo (tile {}, flip {}, rotation {})", tile.index, tile.flip_x, tile.rotation);
}

fn update_colors(
    mut color: EventReader<SelectColor>,
    mut sprites: Query<&mut TextModeTextureAtlasSprite, With<ActiveTile>>,
) {
    for SelectColor(p, bg) in color.iter() {
        let mut sprite = sprites.single_mut();
        if *bg { sprite.bg = p.color(); }
        else { sprite.fg = p.color(); }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<QuickTilesUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}