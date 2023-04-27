use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, mouse, util};
use crate::grid::{Grid, GridChanged};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clicked};
use crate::quick_tiles::{SelectColor, Selection, SelectTile};
use crate::toolbar::SelectedTool;
use crate::tools::Tools;

pub(crate) struct PencilPlugin;

impl Plugin for PencilPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_system(update.in_set(OnUpdate(AppState::Editor)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

#[derive(Component)]
struct PencilUI;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 2,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Pencil,
            shortcut: 'p',
            priority: util::tool_priority::PENCIL,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Pencil),
            hover_click: false,
        });
}

fn update(
    tool: Res<SelectedTool>,
    keys: Res<Input<KeyCode>>,
    mut selection: ResMut<Selection>,
    mut clicks: EventReader<Clicked>,
    mut ev_tile: EventWriter<SelectTile>,
    mut ev_color: EventWriter<SelectColor>,
    mut grid: ResMut<Grid>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    if tool.0 != Tools::Pencil { clicks.clear(); return; }
    for Clicked(id, right_button) in clicks.iter() {
        if let ButtonId::Grid(x, y) = id {
            let Some((ref mut tile, _)) = grid.tiles.get_mut(&(*x, *y)) else { continue };

            if *right_button {
                // Tile info -> Selection
                selection.bg = tile.bg;
                ev_color.send(SelectColor(tile.bg, true));
                selection.fg = tile.fg;
                ev_color.send(SelectColor(tile.fg, false));
                if !keys.pressed(KeyCode::LShift) {
                    selection.index = tile.index;
                    ev_tile.send(SelectTile(tile.index));
                    selection.rotation = tile.rotation;
                    selection.flip = tile.flip.0;
                }
            } else {
                // Selection -> Tile info
                tile.bg = selection.bg;
                tile.fg = selection.fg;
                if !keys.pressed(KeyCode::LShift) {
                    tile.index = selection.index;
                    tile.rotation = selection.rotation;
                    tile.flip.0 = selection.flip;
                }

                grid_changed.send(GridChanged(vec![(*x, *y)]));
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<PencilUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}