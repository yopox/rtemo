use std::collections::HashMap;

use bevy::prelude::*;
use bevy::reflect::List;
use bevy::sprite::Anchor;
use strum::IntoEnumIterator;

use crate::{AppState, grid, mouse, util};
use crate::grid::{Grid, GridResized, Tile, Zoom};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clicked};
use crate::tools::Tools;
use crate::util::Palette;

pub(crate) struct ImportPlugin;

impl Plugin for ImportPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_system(update.in_set(OnUpdate(AppState::Editor)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 12,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Import,
            shortcut: 'i',
            priority: util::tool_priority::IMPORT,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Import),
            hover_click: false,
        });
}

fn update(
    mut commands: Commands,
    mut clicked: EventReader<Clicked>,
    mut grid: Option<ResMut<Grid>>,
    mut grid_resized: EventWriter<GridResized>,
    textures: Res<Textures>,
    zoom: Res<Zoom>,
) {
    let Some(mut grid) = grid else { return; };
    for Clicked(id, right) in clicked.iter() {
        if *right { continue }
        let ButtonId::Tool(Tools::Import) = id else { continue };

        let Ok(clipboard) = cli_clipboard::get_contents() else { continue };

        grid.tiles.values().for_each(|(_, id)| commands.entity(*id).despawn_recursive());
        grid.tiles.clear();

        let palette = Palette::iter().collect::<Vec<Palette>>();

        let mut new_tiles = HashMap::new();

        for line in clipboard.lines() {
            let Some(tile) = parse_tile(line) else { continue; };

            let t = Tile {
                bg: palette[tile.3],
                fg: palette[tile.4],
                index: tile.2,
                flip: (tile.5, false),
                rotation: tile.6,
            };

            let id = grid::spawn_tile(
                &mut commands,
                tile.0 as isize, tile.1 as isize,
                &t, &grid, &textures, &zoom
            );

            new_tiles.insert((tile.0 as isize, tile.1 as isize), (t, id));
            // println!("{} {} {} {} {} {} {}", tile.0, tile.1, tile.2, tile.3, tile.4, tile.5, tile.6);
        }

        let Some(max_x) = new_tiles.keys().map(|(x, _)| *x).max() else { continue };
        let Some(max_y) = new_tiles.keys().map(|(_, y)| *y).max() else { continue };
        grid.x0 = 0;
        grid.y0 = 0;
        grid.w = max_x as usize + 1;
        grid.h = max_y as usize + 1;

        for ((x, y), (t, e)) in new_tiles.iter() {
            grid.tiles.insert((*x, max_y - *y), (t.to_owned(), *e));
        }

        grid_resized.send(GridResized);
    }
}

fn parse_tile(line: &str) -> Option<util::TILE> {
    let mut line = line.trim().strip_prefix("(")?;
    line = line.strip_suffix("),")?;

    let split = line.split(", ").collect::<Vec<&str>>();
    if split.len() != 7 { return None; }

    let x = split[0].parse::<util::X>().ok()?;
    let y = split[1].parse::<util::Y>().ok()?;
    let index = split[2].parse::<util::INDEX>().ok()?;
    let bg = split[3].parse::<util::BG>().ok()?;
    let fg = split[4].parse::<util::FG>().ok()?;
    let flip = split[5].parse::<util::FLIP>().ok()?;
    let rotation = split[6].parse::<util::ROTATION>().ok()?;

    return Some((x, y, index, bg, fg, flip, rotation));
}

fn cleanup() {

}