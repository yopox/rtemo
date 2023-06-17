use bevy::prelude::*;

mod pencil;
mod eraser;
mod pick;
mod text;
mod resize;
mod export;
mod import;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ActivateTool>()
            .add_plugin(pencil::PencilPlugin)
            .add_plugin(eraser::EraserPlugin)
            .add_plugin(text::TextPlugin)
            .add_plugin(resize::ResizePlugin)
            .add_plugin(export::ExportPlugin)
            .add_plugin(import::ImportPlugin)
            // .add_plugin(pick::PickPlugin)
        ;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tools {
    Pencil,
    Eraser,
    Pick,
    Text,
    Resize,
    Export,
    Import,
    Custom(&'static str),
    CustomNonSelectable(&'static str),
}

impl Tools {
    pub fn is_selectable(&self) -> bool {
        match self {
            Tools::Export | Tools::Import
            | Tools::CustomNonSelectable(_) => false,
            _ => true,
        }
    }
}

pub struct ActivateTool(Tools);