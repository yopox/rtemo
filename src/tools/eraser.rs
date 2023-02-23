use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, grid, mouse, util};
use crate::grid::{Grid, GridChanged};
use crate::loading::Textures;
use crate::mouse::Clicked;
use crate::quick_tiles::Selection;
use crate::toolbar::SelectedTool;

pub(crate) struct EraserPlugin;

impl Plugin for EraserPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor).with_system(update))
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct EraserUI;

pub const NAME: &str = "core/tools/eraser";

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 3,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            name: NAME.to_string(),
            shortcut: 'e',
            priority: util::tool_priority::ERASER,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: NAME.to_string(),
            hover_click: false,
        });
}

fn update(
    tool: Res<SelectedTool>,
    selection: Res<Selection>,
    mut clicks: EventReader<Clicked>,
    mut grid: ResMut<Grid>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    if tool.0 != NAME { clicks.clear(); return; }
    for Clicked(id, _) in clicks.iter() {
        if !id.contains(grid::PREFIX) { continue }
        let s = id.split("_").filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<usize>>();
        let (Some(&x), Some(&y)) = (s.get(0), s.get(1)) else { continue };
        let Some((ref mut tile, _)) = grid.tiles.get_mut(&(x, y)) else { continue };

        // Erase tile
        tile.bg = selection.bg;
        tile.fg = selection.fg;
        tile.index = 0;

        grid_changed.send(GridChanged(vec![(x, y)]));
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<EraserUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}