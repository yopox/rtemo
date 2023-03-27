use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppState, HEIGHT, tools, util};
use crate::loading::Textures;
use crate::mouse::{ButtonId, Clicked, Hover};

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedTool(tools::Tools::Pencil))
            .insert_resource(ToolbarItems(vec![]))
            .add_event::<UpdateToolbar>()
            .add_system(setup.in_schedule(OnEnter(AppState::Editor)))
            .add_systems(
                (update_gui, update_toolbar, on_click)
                    .in_set(OnUpdate(AppState::Editor))
            )
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

#[derive(Component)]
pub struct Tool {
    pub id: tools::Tools,
    pub shortcut: char,
    pub priority: u16,
}

#[derive(Resource)]
pub struct SelectedTool(pub tools::Tools);

#[derive(Component)]
struct ToolbarUI;

enum ToolbarItem {
    Tool(Entity),
}

#[derive(Resource)]
struct ToolbarItems(Vec<ToolbarItem>);

pub struct UpdateToolbar;

fn update_toolbar(
    mut ev: EventReader<UpdateToolbar>,
    mut items: ResMut<ToolbarItems>,
    tools: Query<(Entity, &Tool)>,
    mut transform: Query<&mut Transform>,
) {
    for _ in ev.iter() {
        // Collect items
        items.0.clear();

        let mut t = tools
            .iter()
            .collect::<Vec<(Entity, &Tool)>>();
        t.sort_by_key(|(_, t)| t.priority);
        t.iter().for_each(|(e, _)| items.0.push(ToolbarItem::Tool(e.clone())));

        // Display items
        for (n, item) in items.0.iter().enumerate() {
            match *item {
                ToolbarItem::Tool(e) => {
                    if let Ok(mut t) = transform.get_mut(e) {
                        t.translation.x = 8.;
                        t.translation.y = HEIGHT - ((HEIGHT - items.0.len() as f32 * util::size::ICON - 72.) / 2.
                            + n as f32 * util::size::ICON) - util::size::ICON;
                    }
                }
            }
        }
    }
}

#[derive(Component)]
struct HoveredBg;

#[derive(Component)]
struct SelectedBg;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut ev: EventWriter<UpdateToolbar>,
) {
    ev.send(UpdateToolbar);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::rgba(1., 1., 1., 0.25),
                index: 0,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.icons.clone(),
            transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS_BG),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(HoveredBg)
        .insert(ToolbarUI)
    ;

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::rgba(1., 1., 1., 0.5),
                index: 0,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.icons.clone(),
            transform: Transform::from_xyz(0., 0., util::z::TOOLBAR_ICONS_BG),
            ..Default::default()
        })
        .insert(SelectedBg)
        .insert(ToolbarUI)
    ;
}

fn update_gui(
    mut selected: Query<&mut Transform, (With<SelectedBg>, Without<HoveredBg>, Without<Tool>)>,
    mut hovered: Query<(&mut Transform, &mut Visibility), (With<HoveredBg>, Without<SelectedBg>, Without<Tool>)>,
    tools: Query<(&Tool, &Transform, Option<&Hover>)>,
    selected_tool: Res<SelectedTool>,
) {
    let mut t1 = selected.get_single_mut().unwrap();
    let (mut t2, mut v2) = hovered.get_single_mut().unwrap();

    v2.set_if_neq(Visibility::Hidden);

    for (tool, t, h) in tools.iter() {
        if h.is_some() {
            v2.set_if_neq(Visibility::Inherited);
            t2.translation.x = t.translation.x;
            t2.translation.y = t.translation.y;
        }

        if tool.id == selected_tool.0 {
            t1.translation.x = t.translation.x;
            t1.translation.y = t.translation.y;
        }
    }
}

fn on_click(
    mut ev: EventReader<Clicked>,
    mut update: EventWriter<UpdateToolbar>,
    mut selected: ResMut<SelectedTool>,
) {
    for Clicked(id, _) in ev.iter() {
        if let ButtonId::Tool(tool) = id {
            selected.0 = *tool;
            update.send(UpdateToolbar);
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<ToolbarUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}