use bevy::prelude::*;

mod pencil;
mod eraser;
mod pick;
mod text;
mod resize;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(pencil::PencilPlugin)
            .add_plugin(eraser::EraserPlugin)
            .add_plugin(text::TextPlugin)
            .add_plugin(resize::ResizePlugin)
            // .add_plugin(pick::PickPlugin)
        ;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tools {
    Pencil,
    Eraser,
    Pick,
    Text,
    Resize,
    Custom(&'static str),
}