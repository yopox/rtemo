use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, mouse, util};
use crate::loading::Textures;
use crate::mouse::ButtonId;
use crate::tools::Tools;

pub(crate) struct PickPlugin;

impl Plugin for PickPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_system(update.in_set(OnUpdate(AppState::Editor)))
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

#[derive(Component)]
struct PickUI;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 4,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.icons.clone(),
                transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS),
                ..Default::default()
            }
        ))
        .insert(crate::toolbar::Tool {
            id: Tools::Eraser,
            shortcut: 'i',
            priority: util::tool_priority::PICK,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: ButtonId::Tool(Tools::Eraser),
            hover_click: false,
        });
}

fn update() {}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<PickUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}