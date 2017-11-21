//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::Console;

// core::dungeon
pub mod dungeon;

// core::init
pub mod init;

// core::object
pub mod object;

// core::renderer
pub mod renderer;

// core::game
pub mod game;

use self::game::Game;

use self::object::Pos;

use self::renderer::Renderer;

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  // Get map height
  let map_dim = init::map_dimensions();

  // Get a new renderer
  let mut ren = Renderer::new(map_dim, Pos::new(root.width() as isize, root.height() as isize));

  // Get a new game
  let mut game = Game::new(map_dim);

  // Draw all and capture keypresses
  while !(root.window_closed()) {

    // AI actions go here \\

    // Draw what the camera sees
    ren.draw_all(&mut root, &game);

    // Flush all draws to root
    root.flush();

    // Capture keypresses
    game.capture_keypress(&mut root);

    for nx in -1..2 {
      for ny in -1..2 {
        if game.player.pos.x - nx > 0 && game.player.pos.x - nx < game.dungeon.width as isize && game.player.pos.y - ny > 0 && game.player.pos.y - ny < game.dungeon.height as isize && !game.dungeon.grid.0[(game.player.pos.x - nx) as usize][(game.player.pos.y - ny) as usize].blocks{ 
          game.dungeon.scent_map.0[(game.player.pos.x - nx) as usize][(game.player.pos.y - ny) as usize].inc(150);
        }
      }
    }
    game.dungeon.update_scentmap();

  }

}