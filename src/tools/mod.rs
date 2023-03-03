use bevy::prelude::*;

pub use eraser::NAME as ERASER_TOOL;
pub use pencil::NAME as PENCIL_TOOL;

mod pencil;
mod eraser;
mod pick;
mod text;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(pencil::PencilPlugin)
            .add_plugin(eraser::EraserPlugin)
            .add_plugin(text::TextPlugin)
            // .add_plugin(pick::PickPlugin)
        ;
    }
}