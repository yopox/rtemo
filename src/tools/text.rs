use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, mouse, util};
use crate::grid::{Grid, GridChanged, HoverTileIndexOverride};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clicked};
use crate::quick_tiles::Selection;
use crate::toolbar::{SelectedTool, UpdateToolbar};
use crate::tools::Tools;

pub(crate) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_systems((update, on_click, on_type).in_set(OnUpdate(AppState::Editor)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));

    }
}

#[derive(Component)]
struct TextUI;

#[derive(Resource)]
struct TextCursorState {
    start_x: usize,
    start_y: usize,
    current_x: usize,
    current_y: usize,
    frame: usize,
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 7,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Text,
            shortcut: 't',
            priority: util::tool_priority::TEXT,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Text),
            hover_click: false,
        });
}

fn update(
    mut commands: Commands,
    mut ev: EventReader<UpdateToolbar>,
    mut state: Option<ResMut<TextCursorState>>,
    mut cursor: Option<ResMut<HoverTileIndexOverride>>,
    selected: Res<SelectedTool>,
) {
    for UpdateToolbar in ev.iter() {
        if selected.0 == Tools::Text {
            commands.insert_resource(HoverTileIndexOverride {
                index: 927,
                visible: Visibility::Inherited,
                force_x: None,
                force_y: None,
            });
        } else {
            commands.remove_resource::<HoverTileIndexOverride>();
            commands.remove_resource::<TextCursorState>();
        }
    }

    if let Some(mut state) = state {
        state.frame += 1;
        state.frame = state.frame % 60;
        if let Some(mut cursor) = cursor {
            cursor.visible = if state.frame < 30 { Visibility::Inherited } else { Visibility::Hidden };
        }
    }
}

fn on_click(
    mut commands: Commands,
    tool: Res<SelectedTool>,
    mut clicks: EventReader<Clicked>,
    mut hover_override: Option<ResMut<HoverTileIndexOverride>>,
) {
    if tool.0 != Tools::Text { clicks.clear(); return; }
    for Clicked(id, _) in clicks.iter() {
        if let ButtonId::Grid(x, y) = id {
            commands.insert_resource(TextCursorState {
                start_x: *x,
                start_y: *y,
                current_x: *x,
                current_y: *y,
                frame: 0,
            });

            if let Some(ref mut hover_override) = hover_override {
                hover_override.force_x = Some(*x);
                hover_override.force_y = Some(*y);
            }
        }
    }
}

fn on_type(
    mut state: Option<ResMut<TextCursorState>>,
    mut grid: ResMut<Grid>,
    mut grid_changed: EventWriter<GridChanged>,
    mut hover_override: Option<ResMut<HoverTileIndexOverride>>,
    selection: Res<Selection>,
    keys: Res<Input<KeyCode>>,
) {
    let Some(mut state) = state else { return };
    for key in keys.get_just_pressed() {
        if let Some(char) = util::get_char(key) {
            let (x, y) = (state.current_x, state.current_y);
            let Some(index) = util::char_to_tile(char) else { continue };
            let Some((ref mut tile, _)) = grid.tiles.get_mut(&(x, y)) else {continue};

            // Update grid
            tile.index = index;
            tile.bg = selection.bg;
            tile.fg = selection.fg;
            grid_changed.send(GridChanged(vec![(x, y)]));

            // Update state and cursor
            state.current_x += 1;
            if let Some(ref mut hover_override) = hover_override {
                hover_override.force_x = Some(state.current_x);
            }
        } else if *key == KeyCode::Return {
            state.current_x = state.start_x;
            state.current_y += 1;

            // Update cursor
            if let Some(ref mut hover_override) = hover_override {
                hover_override.force_x = Some(state.current_x);
                hover_override.force_y = Some(state.current_y);
            }
        } else if *key == KeyCode::Space {
            // Update state and cursor
            state.current_x += 1;
            if let Some(ref mut hover_override) = hover_override {
                hover_override.force_x = Some(state.current_x);
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TextUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}