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
    pub const GRID_X: usize = 32;
    pub const GRID_Y: usize = 16;
    pub const LEFT_MARGIN: f32 = 32.;
    pub const BOTTOM_MARGIN: f32 = 48.;
}

pub mod z {
    pub const GRID: f32 = 0.;
    pub const GRID_HOVER: f32 = 0.5;
    pub const TOOLBAR: f32 = 1.;
    pub const TOOLBAR_ICONS_BG: f32 = 1.5;
    pub const TOOLBAR_ICONS: f32 = 2.;
}

pub mod misc {
    pub const DEFAULT_TILE: usize = 1;
    pub const TILESET_COUNT: usize = 1024;
    pub const QUICK_TILES_ROWS: usize = 4;
    pub const QUICK_TILES_PER_ROW: usize = 32;
}

#[derive(EnumIter, Copy, Clone)]
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
    Orange0,
    Orange1,
    Pink,
    Beige,
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
            Palette::Orange0 => Color::hex("754b19"),
            Palette::Orange1 => Color::hex("e1891b"),
            Palette::Pink => Color::hex("ddc5c3"),
            Palette::Beige => Color::hex("eee4c9"),
            Palette::Yellow => Color::hex("f6d42d"),
            Palette::Sand => Color::hex("e5ca9f"),
        }.unwrap()
    }

    pub fn from_usize(n: usize) -> Palette {
        match n {
            1 => Palette::White,
            2 => Palette::Blue0,
            3 => Palette::Blue1,
            4 => Palette::Blue2,
            5 => Palette::Blue3,
            6 => Palette::Green0,
            7 => Palette::Green1,
            8 => Palette::Red0,
            9 => Palette::Red1,
            10 => Palette::Orange0,
            11 => Palette::Orange1,
            12 => Palette::Pink,
            13 => Palette::Beige,
            14 => Palette::Yellow,
            15 => Palette::Sand,
            _ => Palette::Black,
        }
    }
}