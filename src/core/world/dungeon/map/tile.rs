use std::fmt;
use std::slice::Iter;

use core::renderer::{Renderable, RGB};

///
/// Tiles have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
  Wall(Wall),
  Floor(Floor),
  Stair(Stair),
  TallGrass,
  Vine,
  Water,
  Unseen,
  Trap(Trap),
  Debug
}

///
/// Floors have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Floor {
  Normal,
  Crystal
}

///
/// Walls have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Wall {
  Normal,
  Crystal,
  Hard
}

///
/// Traps have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Trap {
  MemoryLoss,
  Shaft,
  PaintBomb,
  Teleport
}

///
/// Stairs have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stair {
  DownStair(DownStair),
  UpStair(UpStair)
}

///
/// Up/Down stairs have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DownStair {
  Normal
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UpStair {
  Normal
}

///
/// Properties
/// 

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Props {
  Visibility(Visibility),
  Traversability(Traversability)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Visibility {
  Opaque,
  Transparent
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Traversability {
  Walkable,
  Blocking
}

///
/// Tile type helper functions
/// 
/// The idea is that important or reusable matching patterns can be placed into these functions for a broad range
/// of other resources to utilize without needing to update all of those patterns individually.
/// 
/// They are located in this file, as when tile types are added, the developer also ideally updates these lists at the same
/// time, meaning new tile types can be introduced swiftly
/// 

// Does the tile block vision?
pub fn opaque(t: &Tile) -> bool {
  match t.tiletype {
    Type::Wall(_) | Type::TallGrass => true,
    _ => false
  }
}

// Is it okay to spawn stuff on this tile / replace it?
pub fn spawnable(t: &Tile) -> bool {
  match t.tiletype {
    Type::Floor(_) | Type::Water | Type::TallGrass | Type::Vine => true,
    _ => false
  }
}

// Is the tile able to be walked on?
pub fn walkable(t: &Tile) -> bool {
  match t.tiletype {
    Type::Floor(_) | Type::Water | Type::Stair(_) | Type::Trap(_) | Type::TallGrass | Type::Vine => true,
    _ => false
  }
}

///
/// Archetypal floor patterns
/// 

pub fn generic_floor() -> Tile {
  Tile::new(
    "Generic Floor",
    ' ',
    RGB(0, 0, 0),
    RGB(0, 0, 0),
    Type::Floor(Floor::Normal)
  )
}

pub fn generic_wall() -> Tile {
  Tile::new(
    "Generic Wall",
    ' ',
    RGB(0, 0, 0),
    RGB(0, 0, 0),
    Type::Wall(Wall::Normal)
  )
}

///
/// Tiles have biomes
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Biome {
  Dungeon,
  Crypt,
  Cave,
  Sunken,
  Crystal
}

// Implement ability to turn the enum into a string
impl fmt::Display for Biome {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Biome::Dungeon => write!(fmt, "Dungeon"),
      Biome::Crypt   => write!(fmt, "Crypt"),
      Biome::Cave    => write!(fmt, "Cave"),
      Biome::Sunken  => write!(fmt, "Sunken"),
      Biome::Crystal => write!(fmt, "Crystal")
    }
  }
}

///
/// Scents
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Scent {
  Player = 0,
  Insectoid,
  Canine,
  Feline,
  // Untracked scents, as in won't render in debug mode.
  Reptilian,
  Decay,
  Avian,
  Smoke,
  Incense,
  // C like enum construction for defining # of things inside enum
  Num
}

// Implement ability to turn the enum into a string
impl fmt::Display for Scent {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Scent::Player    => write!(fmt, "Player"),
      Scent::Insectoid => write!(fmt, "Insectoid"),
      Scent::Canine    => write!(fmt, "Canine"),
      Scent::Feline    => write!(fmt, "Feline"),
      Scent::Reptilian => write!(fmt, "Reptilian"),
      Scent::Decay     => write!(fmt, "Decay"),
      Scent::Avian     => write!(fmt, "Avian"),
      Scent::Smoke     => write!(fmt, "Smoke"),
      Scent::Incense   => write!(fmt, "Incense"),
      Scent::Num       => write!(fmt, "Num - Something wrong must have happened"),
    }
  }
}

// Implement an iterator for Scent to get the variants out in order
impl Scent {
  pub fn iterator() -> Iter<'static, Scent> {
    static SCENT_TYPES: [Scent;  Scent::Num as usize] = [
      Scent::Player, 
      Scent::Insectoid, 
      Scent::Canine, 
      Scent::Feline,
      Scent::Reptilian,
      Scent::Decay,
      Scent::Avian,
      Scent::Smoke,
      Scent::Incense
    ];
    SCENT_TYPES.into_iter()
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct _Scent {
  pub val: u8,
  pub scent_type: Scent
}

impl _Scent {

  #[inline]
  pub fn new(value: u8, scent_type: Scent) -> Self {
    _Scent {
      val: value,
      scent_type: scent_type
    }
  }

}

///
/// Tile represents an environmental entity
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tile {
  name: &'static str,
  pub glyph: char,
  pub fg: RGB,
  pub bg: RGB,
  pub biome: Biome,
  pub scents: Vec<_Scent>,
  pub sound: usize,
  pub tiletype: Type,
  pub seen: bool
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: &'static str, glyph: char, fg: RGB, bg: RGB, tiletype: Type) -> Self {
    Tile { 
      name: name,
      glyph: glyph,
      fg: fg,
      bg: bg,
      biome: Biome::Dungeon,
      // Create scents by iterating over ScentTypes
      scents: {
        let mut scent_vec = vec![];
        for scent in Scent::iterator() {
          scent_vec.push(_Scent::new(0, scent.clone()));
        }
        scent_vec
      },
      sound: 0,
      tiletype: tiletype,
      seen: false
    }
  }

}

impl Renderable for Tile {

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
  fn get_id(&self) -> &'static str {
    self.name.clone()
  }

  #[inline]
  fn set_bg(&mut self, bg: RGB) {
    self.bg = bg;
  }

  #[inline]
  fn set_fg(&mut self, fg: RGB) {
    self.fg = fg;
  }

  #[inline]
  fn set_glyph(&mut self, glyph: char) {
    self.glyph = glyph
  }

  #[inline]
  fn set_id(&mut self, name: &'static str) {
    self.name = name;
  }

}