use bevy::prelude::*;
use crate::AppState;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()
            .add_system_set(SystemSet::on_update(AppState::Editor)
                .with_system(update)
            )
            .add_system_set(SystemSet::on_exit(AppState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct MouseUI;

#[derive(Component)]
pub struct Clickable {
    pub w: f32,
    pub h: f32,
    pub id: String,
}

#[derive(Component)]
pub struct Hover;

pub struct Clicked(pub String, pub bool);

/// Assuming [Clickable]-s have the [bevy::sprite::Anchor::BottomLeft] anchor:
/// - adds [Hover] component to entities with [Clickable] & [Transform] being hovered
/// - sends [Clicked] events
fn update(
    mut commands: Commands,
    mut ev: EventWriter<Clicked>,
    buttons: Query<(Entity, &Transform, &Clickable)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    for (e, _, _) in buttons.iter() {
        commands.entity(e).remove::<Hover>();
    }

    let clicked_left = mouse.just_pressed(MouseButton::Left);
    let clicked_right = mouse.just_pressed(MouseButton::Right);
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        for (e, t, c) in buttons.iter() {
            let x = t.translation.x + c.w / 2.;
            let y = t.translation.y + c.h / 2.;
            let hover = (pos.x / 4. - x).abs() <= c.w / 2. && (pos.y / 4. - y).abs() <= c.h / 2.;
            if hover { commands.entity(e).insert(Hover); }
            if hover && (clicked_left || clicked_right) { ev.send(Clicked(c.id.clone(), clicked_right)); }
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