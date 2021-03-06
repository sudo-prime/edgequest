use core::world::dungeon::map::{self, tile, Tile};

use super::AI;
use core::creature::{Actions, Creature, Actor, Stats};

///
/// AI that tracks player
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrackerAI;

impl TrackerAI {
  #[inline]
  pub fn new() -> Self {
    TrackerAI {}
  }
}

impl AI for TrackerAI {
  
  ///
  /// Track player and follow if near
  ///
  fn take_turn(&mut self, map: &map::Grid<Tile>, player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut state = Actions::Wait;

    // ^ is overridden to be the distance formula, this isn't xor
    let distance = me.pos ^ player.actor.pos;
    let mut x = me.pos.x;
    let mut y = me.pos.y;

    if distance < 20.0 && distance > 2.00 {

      // Move x
      if x < player.actor.pos.x {
        x += 1;
        state = Actions::Move;
      } else if x > player.actor.pos.x {
        x -= 1;
        state = Actions::Move;
      }

      // Check
      if !tile::walkable(&map[x as usize][y as usize]) {
        x = me.pos.x;
      }

      // Move y
      if y < player.actor.pos.y {
        y += 1;
        state = Actions::Move;
      } else if y > player.actor.pos.y {
        y -= 1;
        state = Actions::Move;
      }

      // Check
      if !tile::walkable(&map[x as usize][y as usize]) {
        y = me.pos.y
      }

    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<AI> {
    Box::new((*self).clone())
  }

}