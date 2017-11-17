use game::tcod::colors::Color;

pub use game::object::entity::Entity;
pub use game::object::pos::Pos;

///
/// Tile represents an environmental entity
/// 
pub struct Tile {
  pub entity: Entity,
  pub blocks: bool,
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  pub fn new(pos: Pos, glyph: char, fg: Color, bg: Color, blocks: bool) -> Tile {
    return Tile { 
      entity: Entity::new(pos, glyph, fg, bg), 
      blocks: blocks
    };
  }

}