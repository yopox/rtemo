use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, mouse, util};
use crate::loading::Textures;

pub(crate) struct PickPlugin;

impl Plugin for PickPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Editor).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Editor).with_system(update))
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct PickUI;

pub const NAME: &str = "core/tools/pick";

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
            name: NAME.to_string(),
            shortcut: 'i',
            priority: util::tool_priority::PICK,
        })
        .insert(mouse::Clickable {
            w: 16.0,
            h: 16.0,
            id: NAME.to_string(),
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