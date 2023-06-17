use bevy::prelude::{Color, KeyCode};
use strum_macros::EnumIter;

pub mod tool_priority {
    pub const PENCIL: u16 = 0;
    pub const FILL: u16 = 10;
    pub const ERASER: u16 = 20;
    pub const SELECT: u16 = 30;
    pub const PICK: u16 = 40;
    pub const TEXT: u16 = 50;
    pub const RESIZE: u16 = 60;
    pub const EXPORT: u16 = 70;
    pub const IMPORT: u16 = 80;
}

pub mod size {
    pub const ICON: f32 = 16.;
    pub const GRID_X: usize = 6;
    pub const GRID_Y: usize = 6;
    pub const LEFT_MARGIN: f32 = 32.;
    pub const BOTTOM_MARGIN: f32 = 40.;
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

#[derive(EnumIter, Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Palette {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Transparent,
    Black,
}

impl Palette {
    pub fn color(&self) -> Color {
        match self {
            Palette::Transparent => Color::hex("00000000"),
            Palette::Black => Color::hex("00000000"),
            Palette::A => Color::hex("#ffffff"),
            Palette::B => Color::hex("#6df7c1"),
            Palette::C => Color::hex("#11adc1"),
            Palette::D => Color::hex("#606c81"),
            Palette::E => Color::hex("#393457"),
            Palette::F => Color::hex("#1e8875"),
            Palette::G => Color::hex("#5bb361"),
            Palette::H => Color::hex("#a1e55a"),
            Palette::I => Color::hex("#f7e476"),
            Palette::J => Color::hex("#f99252"),
            Palette::K => Color::hex("#cb4d68"),
            Palette::L => Color::hex("#6a3771"),
            Palette::M => Color::hex("#c92464"),
            Palette::N => Color::hex("#f48cb6"),
            Palette::O => Color::hex("#f7b69e"),
            Palette::P => Color::hex("#9b9c82"),
        }.unwrap()
    }
}

pub fn get_char(code: &KeyCode) -> Option<char> {
    match code {
        KeyCode::A => Some('a'),
        KeyCode::B => Some('b'),
        KeyCode::C => Some('c'),
        KeyCode::D => Some('d'),
        KeyCode::E => Some('e'),
        KeyCode::F => Some('f'),
        KeyCode::G => Some('g'),
        KeyCode::H => Some('h'),
        KeyCode::I => Some('i'),
        KeyCode::J => Some('j'),
        KeyCode::K => Some('k'),
        KeyCode::L => Some('l'),
        KeyCode::M => Some('m'),
        KeyCode::N => Some('n'),
        KeyCode::O => Some('o'),
        KeyCode::P => Some('p'),
        KeyCode::Q => Some('q'),
        KeyCode::R => Some('r'),
        KeyCode::S => Some('s'),
        KeyCode::T => Some('t'),
        KeyCode::U => Some('u'),
        KeyCode::V => Some('v'),
        KeyCode::W => Some('w'),
        KeyCode::X => Some('x'),
        KeyCode::Y => Some('y'),
        KeyCode::Z => Some('z'),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}

pub fn char_to_tile(c: char) -> Option<usize> {
    let start = b'!' as usize;
    let start_index: usize = 865;

    let input = c.to_ascii_uppercase() as usize;
    if input < start { return None }
    let result = input - start + start_index;

    return Some(result)
}

pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = usize;
pub type FG = usize;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);