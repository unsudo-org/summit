use super::*;

#[derive(Clone)]
pub struct Theme {
    pub color: Color,
    pub font: Font,
    pub cursor: Cursor
}

#[derive(Clone)]
pub struct Color {
    pub foreground: color::Hex<2, u128>,
    pub background: color::Hex<2, u128>,
    pub success: color::Hex<2, u128>,
    pub failure: color::Hex<2, u128>,
    pub warning: color::Hex<2, u128>,
    pub highlight: color::Hex<2, u128>
}

#[derive(Clone)]
pub struct Font {
    pub regular: &'static str,
    pub body: &'static str,
    pub display: &'static str,
    pub monospace: &'static str
}

#[derive(Clone)]
pub struct Cursor {
    pub ban: Asset,
    pub click: Asset,
    pub default: Asset,
    pub disabled: Asset,
    pub finger: Asset,
    pub finger_disabled: Asset,
    pub grip: Asset,
    pub grip_horizontal: Asset,
    pub grip_vertical: Asset,
    pub hand: Asset,
    pub loading: Asset,
    pub move_diagonal_from_bottom_left: Asset,
    pub move_diagonal_from_top_left: Asset,
    pub move_horizontal: Asset,
    pub move_vertical: Asset,
    pub square: Asset,
    pub square_dashed: Asset,
    pub text_input: Asset,
    pub zoom_in: Asset,
    pub zoom_out: Asset
}