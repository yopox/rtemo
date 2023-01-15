mod pencil;
mod eraser;
mod pick;
mod text;

use bevy::prelude::*;

pub use eraser::NAME as ERASER_TOOL;
pub use pencil::NAME as PENCIL_TOOL;
pub use pick::NAME as PICK_TOOL;
pub use text::NAME as TEXT_TOOL;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(pencil::PencilPlugin)
            .add_plugin(eraser::EraserPlugin)
            .add_plugin(pick::PickPlugin)
            .add_plugin(text::TextPlugin)
        ;
    }
}