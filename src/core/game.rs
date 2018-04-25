
use core::world::World;

use core::tcod::input;

// use core::renderer::Screen;

///
/// Enum representing possible actions the player can take
///
pub enum Actions {
  // Player moved
  Move,
  // Player waited
  Wait,
  // Player went down
  DownStair,
  // Player went up
  UpStair,
  // Unknown action
  Unknown
}

///
/// Enum representing the state of the game
///
pub enum State {
  // Game just created
  New,
  // Player acted
  Act(Actions),
}

///
/// Game struct. Holds a player and a floor
/// 
/// * `world` - `World` to represent the game world
/// * `state` - represents the game state
/// 
pub struct Game {
  pub world: World,
  pub state: State
}

impl Game {

  ///
  /// Capture keyboard input from tcod
  /// 
  pub fn process_keypress(&mut self, keypress: input::Key) {

    match keypress.code {

      input::KeyCode::Escape => panic!("Bye"),
      _ => { 
  
        if keypress.printable != ' ' {

          let oldpos = self.world.player.pos.clone();
        
          match keypress.printable {

            'h' => { 
              self.world.player.move_cart(-1, 0);
              self.state = State::Act(Actions::Move);
            },
            'j' => { 
              self.world.player.move_cart(0, 1);
              self.state = State::Act(Actions::Move);
            },
            'k' => {
              self.world.player.move_cart(0, -1);
              self.state = State::Act(Actions::Move);
            },
            'l' => {
              self.world.player.move_cart(1, 0);
              self.state = State::Act(Actions::Move);  
            },
            'y' => {
              self.world.player.move_cart(-1, -1);
              self.state = State::Act(Actions::Move);
            },
            'u' => {
              self.world.player.move_cart(1, -1);
              self.state = State::Act(Actions::Move);
            },
            'b' => {
              self.world.player.move_cart(-1, 1);
              self.state = State::Act(Actions::Move);
            },
            'n' => { 
              self.world.player.move_cart(1, 1);
              self.state = State::Act(Actions::Move);
            },
            '.' => { self.state = State::Act(Actions::Wait) },
            '>' => { self.state = State::Act(Actions::DownStair) },
            '<' => { self.state = State::Act(Actions::UpStair) },
            _ => { self.state = State::Act(Actions::Unknown) }
            
          }

          if !self.world.cur_dungeon.is_valid(self.world.player.pos.x as usize, self.world.player.pos.y as usize) {
            self.world.player.pos = oldpos;
            self.state = State::Act(Actions::Unknown);
          }

        } 
        
        /* 
        else {
          println!("{:?}", keypress.code);
        }
        */

      }

    }
    
  }

  ///
  /// Return a new `Game`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new(map_dim: (isize, isize)) -> Game {

    Game {
      world: World::new(map_dim),
      state: State::New
    }
    
  }

  ///
  /// Update the game depending on the state
  ///
  pub fn update(&mut self) {
    match &self.state {
      &State::Act(Actions::Move) | &State::Act(Actions::Wait) => self.world.update(),
      &State::Act(Actions::DownStair) => {
        if self.world.can_go_down() {
          self.world.go_down();
        }
      },
      &State::Act(Actions::UpStair) => {}
      _ => ()
    }
  }

}
