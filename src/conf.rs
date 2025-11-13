use super::*;

#[derive(Clone)]
pub struct Conf {
    pub color: ColorPalette,
    pub font: FontPalette,
    pub cursor: CursorAssetSheet
}

pub type Hex = ::kore::color::Hex<2, u128>;

#[derive(Clone)]
pub struct ColorPalette {
    pub raisin_black: Hex,
    pub medium_slate_blue: Hex,
    pub rose_pompadour: Hex,
    pub aquamarine: Hex,
    pub mindaro: Hex,
    pub timberwolf: Hex,
    pub silver: Hex
}

#[derive(Clone)]
pub struct FontPalette {
    pub br_cobane: &'static str,
    pub stray: &'static str,
    pub alien_skyline: &'static str
}

#[derive(Clone)]
pub struct CursorAssetSheet {
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