use std::collections::HashMap;

use bevy::prelude::*;
use bevy::reflect::List;
use bevy::sprite::Anchor;
use strum::IntoEnumIterator;

use crate::{AppState, mouse, util};
use crate::grid::{Grid, Tile};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clicked};
use crate::tools::Tools;
use crate::util::Palette;

pub(crate) struct ExportPlugin;

impl Plugin for ExportPlugin {
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
                    index: 11,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Export,
            shortcut: 'r',
            priority: util::tool_priority::EXPORT,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Export),
            hover_click: false,
        });
}

fn update(
    mut clicked: EventReader<Clicked>,
    grid: Option<Res<Grid>>,
) {
    let Some(grid) = grid else { return; };
    for Clicked(id, right) in clicked.iter() {
        if *right { continue }
        let ButtonId::Tool(Tools::Export) = id else { continue };

        // Index palette
        let mut palette: HashMap<Palette, usize> = HashMap::new();
        Palette::iter().enumerate().for_each(|(i, p)| { palette.insert(p, i); });

        // Grid tiles
        let mut tiles = grid.tiles.iter().collect::<Vec<(&(isize, isize), &(Tile, Entity))>>();
        tiles.sort_by_key(|((x, y), _)| (*y - grid.y0) * 32 + (*x - grid.x0));

        let mut export = "[\n".to_string();
        for (&(x, y), (tile, _)) in tiles.iter() {
            let (Some(bg), Some(fg)) = (palette.get(&tile.bg), palette.get(&tile.fg)) else { continue };
            info!("({}, {}, {}, {}, {}, {}, {}),", x - grid.x0, y - grid.y0, tile.index, bg, fg, tile.flip.0, tile.rotation);
            export += &format!("    ({}, {}, {}, {}, {}, {}, {}),\n", x - grid.x0, y - grid.y0, tile.index, bg, fg, tile.flip.0, tile.rotation);
        }
        export += "];";

        cli_clipboard::set_contents(export.to_string()).expect("Couldn't export to clipboard.");
    }
}

fn cleanup() {

}