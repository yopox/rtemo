use bevy::prelude::Color;
use strum_macros::EnumIter;

pub mod tool_priority {
    pub const PENCIL: u16 = 0;
    pub const FILL: u16 = 10;
    pub const ERASER: u16 = 20;
    pub const SELECT: u16 = 30;
    pub const PICK: u16 = 40;
    pub const TEXT: u16 = 50;
}

pub mod size {
    pub const ICON: f32 = 16.;
}

pub mod z {
    pub const TOOLBAR: f32 = 1.;
    pub const TOOLBAR_ICONS_BG: f32 = 1.5;
    pub const TOOLBAR_ICONS: f32 = 2.;
}

#[derive(EnumIter)]
pub enum Palette {
    Black,
    White,
    Blue0,
    Blue1,
    Blue2,
    Blue3,
    Green0,
    Green1,
    Red0,
    Red1,
    Pink,
    Beige,
    Orange0,
    Orange1,
    Yellow,
    Sand,
}

impl Palette {
    pub fn color(&self) -> Color {
        match self {
            Palette::Black => Color::hex("010103"),
            Palette::White => Color::hex("ffffff"),
            Palette::Blue0 => Color::hex("151a2d"),
            Palette::Blue1 => Color::hex("203746"),
            Palette::Blue2 => Color::hex("323f74"),
            Palette::Blue3 => Color::hex("6cc8d7"),
            Palette::Green0 => Color::hex("234425"),
            Palette::Green1 => Color::hex("469456"),
            Palette::Red0 => Color::hex("871b11"),
            Palette::Red1 => Color::hex("e94b4f"),
            Palette::Pink => Color::hex("ddc5c3"),
            Palette::Beige => Color::hex("eee4c9"),
            Palette::Orange0 => Color::hex("754b19"),
            Palette::Orange1 => Color::hex("e1891b"),
            Palette::Yellow => Color::hex("f6d42d"),
            Palette::Sand => Color::hex("e5ca9f"),
        }.unwrap()
    }
}