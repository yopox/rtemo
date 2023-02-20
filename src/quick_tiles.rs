use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use strum::IntoEnumIterator;
use crate::{AppState, util, WIDTH};
use crate::loading::Textures;
use crate::mouse::{Clickable, Clicked, Hover};

pub struct QuickTilesPlugin;

impl Plugin for QuickTilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectTile>()
            .add_event::<SelectColor>()
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor)
                .with_system(update)
                .with_system(on_click)
                .with_system(update_active_tile)
                .with_system(update_colors)
            )
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
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

const PREFIX: &str = "qt_tile_";
const PREFIX_COLOR: &str = "qt_color_";

#[derive(Component)]
struct ActiveTile;

pub struct SelectTile(pub usize);
pub struct SelectColor(pub usize, pub bool);

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    // Quick tiles
    let mut tiles = Vec::new();
    let dx = 56.;
    for i in 0..64 {
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
                transform: Transform::from_xyz(dx + (i % 32) as f32 * 8., 16. - (i / 32) as f32 * 8., util::z::TOOLBAR_ICONS),
                ..Default::default()
            })
            .insert(Clickable {
                w: 8.,
                h: 8.,
                id: format!("{PREFIX}{i}"),
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
                bg: Color::BLACK,
                fg: Color::WHITE,
                alpha: 1.,
                index: 1,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.mrmotext.clone(),
            transform: Transform {
                translation: Vec3::new(32., 8., util::z::TOOLBAR),
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
                transform: Transform::from_xyz(8. + 8. * (i / 8) as f32, 64. - 8. * (i % 8) as f32, 0.),
                texture: textures.color.clone(),
                ..Default::default()
            })
            .insert(Clickable {
                w: 8.,
                h: 8.,
                id: format!("{PREFIX_COLOR}{i}"),
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
) {
    for Clicked(id, right) in clicked.iter() {
        if id.contains(PREFIX) {
            let Some(num) = id.strip_prefix(PREFIX) else { continue };
            let Ok(n) = num.parse::<usize>() else { continue };
            let Some(quick_tile) = quick_tiles.0.iter().find(|tile| tile.index == n) else { continue };
            select_tile.send(SelectTile(quick_tile.tile));
        } else if id.contains(PREFIX_COLOR) {
            let Some(num) = id.strip_prefix(PREFIX_COLOR) else { continue };
            let Ok(n) = num.parse::<usize>() else { continue };
            select_color.send(SelectColor(n, *right));
        }
    }
}

fn update_active_tile(
    mut select_tile: EventReader<SelectTile>,
    mut tile: Query<&mut TextModeTextureAtlasSprite, With<ActiveTile>>,
) {
    for SelectTile(i) in select_tile.iter() {
        let mut tile = tile.single_mut();
        tile.index = *i;
    }
}

fn update_colors(
    mut color: EventReader<SelectColor>,
    mut sprites: Query<&mut TextModeTextureAtlasSprite, With<ActiveTile>>,
) {
    for SelectColor(i, bg) in color.iter() {
        let mut sprite = sprites.single_mut();
        if *bg { sprite.bg = util::Palette::from_usize(*i); }
        else { sprite.fg = util::Palette::from_usize(*i); }
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