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

#[derive(EnumIter, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Palette {
    Transparent,
    Black,
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
}

impl Palette {
    pub fn color(&self) -> Color {
        match self {
            Palette::Transparent => Color::hex("00000000"),
            Palette::Black => Color::hex("000000"),

            Palette::A => Color::hex("F7913D"),
            Palette::B => Color::hex("989281"),
            Palette::C => Color::hex("5B524C"),
            Palette::D => Color::hex("3D2F2C"),
            Palette::E => Color::hex("627EAF"),
            Palette::F => Color::hex("B3CBDF"),
            Palette::G => Color::hex("FBCC0A"),
            Palette::H => Color::hex("FBEAA6"),
            Palette::I => Color::hex("88519B"),
            Palette::J => Color::hex("CBAAD1"),
            Palette::K => Color::hex("70824D"),
            Palette::L => Color::hex("B8BF9D"),
            Palette::M => Color::hex("C17329"),
            Palette::N => Color::hex("F7C8A5"),
            Palette::O => Color::hex("9E3636"),
            Palette::P => Color::hex("F4898B"),
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