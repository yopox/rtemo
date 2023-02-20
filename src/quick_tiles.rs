use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use crate::{AppState, util, WIDTH};
use crate::loading::Textures;
use crate::mouse::{Clickable, Clicked, Hover};

pub struct QuickTilesPlugin;

impl Plugin for QuickTilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectTile>()
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor)
                .with_system(update)
                .with_system(on_click)
                .with_system(update_active_tile)
            )
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct QuickTilesUI;

struct QuickTileId {
    index: usize,
    tile: usize,
    entity: Entity,
}

#[derive(Resource)]
struct QuickTiles(Vec<QuickTileId>);

const PREFIX: &str = "quick_tiles_";

#[derive(Component)]
struct ActiveTile;

pub struct SelectTile(pub usize);

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    // Quick tiles
    let mut tiles = Vec::new();
    let dx = 24.;
    for i in 0..64 {
        let id = commands.
            spawn(TextModeSpriteSheetBundle {
                sprite: TextModeTextureAtlasSprite {
                    bg: Color::BLACK,
                    fg: Color::WHITE,
                    alpha: 0.5,
                    index: i,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.mrmotext.clone(),
                transform: Transform::from_xyz(dx + (i % 32) as f32 * 8., 8. - (i / 32) as f32 * 8., util::z::TOOLBAR_ICONS),
                ..Default::default()
            })
            .insert(Clickable {
                w: 8.,
                h: 8.,
                id: format!("{PREFIX}{i}"),
            })
            .insert(QuickTilesUI)
            .id();
        tiles.push(QuickTileId { index: i, tile: i, entity: id, });
    }

    commands.insert_resource(QuickTiles(tiles));

    // Active tile
    commands.
        spawn(TextModeSpriteSheetBundle {
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
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ActiveTile)
        .insert(QuickTilesUI);
}

fn update(
    mut hover: Query<(&mut TextModeTextureAtlasSprite, Option<&Hover>)>,
) {
    for (mut sprite, h) in hover.iter_mut() {
        sprite.alpha = if h.is_some() { 1.0 } else { 0.5 };
    }
}

fn on_click(
    quick_tiles: Res<QuickTiles>,
    mut clicked: EventReader<Clicked>,
    mut select_tile: EventWriter<SelectTile>,
) {
    for Clicked(id) in clicked.iter() {
        if id.contains(PREFIX) {
            let Some(num) = id.strip_prefix(PREFIX) else { continue };
            let Ok(n) = num.parse::<usize>() else { continue };
            let Some(quick_tile) = quick_tiles.0.iter().find(|tile| tile.index == n) else { continue };
            info!("Quick tile {} - {} clicked.", quick_tile.index, quick_tile.tile);
            select_tile.send(SelectTile(quick_tile.tile));
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

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<QuickTilesUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}