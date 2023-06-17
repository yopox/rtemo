use bevy::prelude::*;

use crate::AppState;
use crate::tools::Tools;
use crate::util::Palette;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()
            .add_system(update.in_base_set(CoreSet::Last))
            .add_system(cleanup.in_schedule(OnExit(AppState::Editor)));
    }
}

#[derive(Component)]
struct MouseUI;

#[derive(Copy, Clone, Debug)]
pub enum ButtonId {
    Tool(Tools),
    Grid(isize, isize),
    QuickTile(usize),
    QuickColor(Palette),
    Custom(&'static str),
}

#[derive(Component)]
pub struct Clickable {
    pub w: f32,
    pub h: f32,
    pub id: ButtonId,
    pub hover_click: bool,
}

#[derive(Component)]
pub struct Hover;

#[derive(Component)]
struct AlreadyClicked;

pub struct Clicked(pub ButtonId, pub bool);

/// Assuming [Clickable]-s have the [bevy::sprite::Anchor::BottomLeft] anchor:
/// - adds [Hover] component to entities with [Clickable] & [Transform] being hovered
/// - sends [Clicked] events
fn update(
    mut commands: Commands,
    mut ev: EventWriter<Clicked>,
    buttons: Query<(Entity, &Transform, &Clickable, Option<&AlreadyClicked>)>,
    mouse: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
) {
    for (e, _, _, _) in buttons.iter() {
        commands.entity(e).remove::<Hover>();
    }

    let just_clicked_left = mouse.just_pressed(MouseButton::Left);
    let just_clicked_right = mouse.just_pressed(MouseButton::Right);
    let clicked_left = mouse.pressed(MouseButton::Left);
    let released = mouse.just_released(MouseButton::Left);
    let window = windows.get_single().unwrap();
    if let Some(pos) = window.cursor_position() {
        for (e, t, c, already_clicked) in buttons.iter() {
            let x = t.translation.x + c.w / 2.;
            let y = t.translation.y + c.h / 2.;
            let hover = (pos.x / 4. - x).abs() <= c.w / 2. && (pos.y / 4. - y).abs() <= c.h / 2.;
            let mut entity = commands.entity(e);
            if hover { entity.insert(Hover); }
            let do_click = hover && (just_clicked_left || just_clicked_right);
            let do_hover_click = hover && clicked_left && c.hover_click;
            if already_clicked.is_none() && (do_click || do_hover_click) {
                // bevy::log::info!("Clicked {:?}", c.id);
                ev.send(Clicked(c.id.clone(), just_clicked_right));
                entity.insert(AlreadyClicked);
            }
            if released { entity.remove::<AlreadyClicked>(); }
        }
    }

}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<MouseUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}