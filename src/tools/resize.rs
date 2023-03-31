use bevy::prelude::*;
use bevy::reflect::List;
use bevy::sprite::Anchor;

use crate::{AppState, mouse, util};
use crate::grid::{Grid, GridResized};
use crate::loading::Textures;
use crate::mouse::ButtonId;
use crate::toolbar::SelectedTool;
use crate::tools::Tools;

pub(crate) struct ResizePlugin;

impl Plugin for ResizePlugin {
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
                    index: 9,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Resize,
            shortcut: 'r',
            priority: util::tool_priority::RESIZE,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Resize),
            hover_click: false,
        });
}

fn update(
    tool: Res<SelectedTool>,
    keys: Res<Input<KeyCode>>,
    grid: Option<ResMut<Grid>>,
    mut sprite: Query<(&crate::toolbar::Tool, &mut TextureAtlasSprite)>,
    mut resize_grid: EventWriter<GridResized>,
) {
    if tool.0 != Tools::Resize { return; }
    let Some(mut grid) = grid else { return; };

    let mut resized = true;
    if keys.pressed(KeyCode::LShift) {
        if keys.just_pressed(KeyCode::LShift) {
            for (tool, mut sprite) in sprite.iter_mut() {
                if tool.id == Tools::Resize { sprite.index = 10; }
            }
        }

        // Crop grid
        if keys.just_pressed(KeyCode::Left) {
            if grid.w == 1 { return; }
            grid.w -= 1;
        } else if keys.just_pressed(KeyCode::Right) {
            if grid.w == 1 { return; }
            grid.w -= 1;
            grid.x0 += 1;
        } else if keys.just_pressed(KeyCode::Up) {
            if grid.h == 1 { return; }
            grid.h -= 1;
        } else if keys.just_pressed(KeyCode::Down) {
            if grid.h == 1 { return; }
            grid.h -= 1;
            grid.y0 += 1;
        } else {
            resized = false;
        }
    } else {
        if keys.just_released(KeyCode::LShift) {
            for (tool, mut sprite) in sprite.iter_mut() {
                if tool.id == Tools::Resize { sprite.index = 9; }
            }
        }

        // Extend grid
        if keys.just_pressed(KeyCode::Left) {
            if grid.w == 24 { return; }
            grid.x0 -= 1;
            grid.w += 1;
        } else if keys.just_pressed(KeyCode::Right) {
            if grid.w == 24 { return; }
            grid.w += 1;
        } else if keys.just_pressed(KeyCode::Up) {
            if grid.h == 24 { return; }
            grid.y0 -= 1;
            grid.h += 1;
        } else if keys.just_pressed(KeyCode::Down) {
            if grid.h == 24 { return; }
            grid.h += 1;
        } else {
            resized = false;
        }
    }

    if resized {
        resize_grid.send(GridResized);
    }
}

fn cleanup() {

}