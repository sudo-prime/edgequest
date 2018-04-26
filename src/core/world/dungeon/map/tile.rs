use core::object::{Entity, RGB};

pub const DARKEN_FAC : RGB = RGB(10, 10, 10);
pub const YELLOW_FAC : RGB = RGB(25, 20, 10);

///
/// Tiles have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileType {
  Wall,
  Floor,
  DownStair,
  UpStair,
  Water,
  Unseen,
  Debug
}

///
/// Tiles have biomes
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Biome {
  Dungeon
}

///
/// Tile represents an environmental entity
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tile {
  name: &'static str,
  pub glyph: char,
  fg: RGB,
  bg: RGB,
  pub biome: Biome,
  pub scent: u8,
  pub sound: u8, // Not in use (yet)
  pub tiletype: TileType,
  pub seen: bool
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: &'static str, glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8), tiletype: TileType) -> Tile {
    Tile { 
      name: name,
      glyph: glyph,
      fg: RGB::from_tup(fg),
      bg: RGB::from_tup(bg),
      biome: Biome::Dungeon,
      scent: 0,
      sound: 0,
      tiletype: tiletype,
      seen: false
    }
  }

  ///
  /// Darken a tile's fg and bg color
  ///
  pub fn darken(&mut self) -> Tile {
    let mut t = self.clone();
    t.fg = self.fg - DARKEN_FAC;
    t.bg = self.bg - DARKEN_FAC;
    return t;
  }

  ///
  /// Make a tile's fg and bg color more yellowish
  ///
  pub fn yellowish(&mut self) -> Tile {
    let mut t = self.clone();
    t.fg = self.fg + YELLOW_FAC;
    t.bg = self.bg + YELLOW_FAC;
    return t;
  }

}

impl Entity for Tile {

  #[inline]
  fn get_bg(&self) -> RGB {
    self.bg
  }

  #[inline]
  fn get_fg(&self) -> RGB {
    self.fg
  }

  #[inline]
  fn get_glyph(&self) -> char {
    self.glyph
  }

  #[inline]
  fn get_name(&self) -> &'static str {
    self.name.clone()
  }

  #[inline]
  fn set_bg(&mut self, bg: (u8, u8, u8)) {
    self.bg = RGB::from_tup(bg);
  }

  #[inline]
  fn set_fg(&mut self, fg: (u8, u8, u8)) {
    self.fg = RGB::from_tup(fg);
  }

  #[inline]
  fn set_glyph(&mut self, glyph: char) {
    self.glyph = glyph
  }

  #[inline]
  fn set_name(&mut self, name: &'static str) {
    self.name = name;
  }

}