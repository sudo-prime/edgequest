//!
//! Generate a super simple dungeon
//!

extern crate rand;
use self::rand::Rng;

use core::renderer::{Renderable, RGB};

pub mod map;
use self::map::{tile, Pos, Tile};

// Privately use filter
mod filter;
use self::filter::{Filter, Structure, Simple};

// Privately use automata
mod automata;
use self::automata::{Automaton, DrunkardsWalk};

// Privately use builders
mod builder;
use self::builder::{Buildable, Fussy};

mod dungeon_tests;

const GRASS_COLORS : [RGB; 3] = [
  RGB(76, 74, 75),
  RGB(76, 79, 75),
  RGB(80, 74, 75)
];

const VINE_GLYPHS : [char; 5] = [
  '/', '|', '\\', '-', '~'
];


///
/// `Dungeon` struct to stitch together all builders and cellular automatons
///
#[derive(Default, Clone)]
pub struct Dungeon {
  pub width: usize,
  pub height: usize,
  pub grid: map::Grid<Tile>,
}

// Make Dungeon Indexable
// The trick here is that you can do dungeon[][] since it will return a vector
// that is in of itself, indexable
impl std::ops::Index<usize> for Dungeon {
  type Output = [Tile];
  fn index(&self, idx: usize) -> &Self::Output {
    &self.grid[idx]
  }
}

impl std::ops::IndexMut<usize> for Dungeon {
  fn index_mut(&mut self, idx: usize) -> &mut [Tile] {
    &mut self.grid[idx]
  }
}

impl std::ops::Index<map::Pos> for Dungeon {
  type Output = Tile;
  fn index(&self, idx: map::Pos) -> &Self::Output {
    &self.grid[idx]
  }
}

impl std::ops::IndexMut<map::Pos> for Dungeon {
  fn index_mut(&mut self, idx: map::Pos) -> &mut Tile {
    &mut self.grid[idx]
  }
}

impl Dungeon {

  #[inline]
  pub fn width(&self) -> usize {
    self.grid.len()
  }

  #[inline]
  pub fn height(&self) -> usize {
    self.grid[0].len()
  }


  ///
  /// Add a tile to the gird and preserve the existing BG color of the spot
  /// 
  fn add_tile(&mut self, g: &mut map::Grid<Tile>, t: &mut Tile, pos: Pos) {
    // Get the background color of the tile that the new one will be going on top of
    let bg_col = g[pos].get_bg();
    t.set_bg(bg_col);
    // Replace grid tile with tile
    g[pos] = t.clone();
  }

  ///
  /// Make the dungeon
  ///
  pub fn build(mut self) -> Self {

    // The purpose of this function is to create some basic grid object, 
    // and completely fill it out into a dungeon.

    // Currently, it only builds on specific type of dungeon, and in the future,
    // there should be some construct to generate types of dungeons.

    // 'Types' here refers to dungeons with different themes, that have different terrain features -
    // think of branches in DCSS. Coupled with this, the Dungeon struct should hold onto this type and
    // the world should be able to access it to generate items and monsters in the type/biome combinations
    // that will appear

    // We start with a basic grid object. We will pass references of this object into various functions to carve out a dungeon.
    let mut grid : map::Grid<Tile>;

    // Fill the map with walls first
    grid = Dungeon::generate_grid(self.width, self.height, Tile::new(
      "Wall",
      ' ',
      RGB(40, 40, 40),
      RGB(33, 33, 33),
      tile::Type::Wall(tile::Wall::Normal))
    );

    // Apply simple builder. This creates a simple corridor/room dungeon based off the simple builder
    // Note how the grid is being consumed to replace itself here, but we don't want this later on.
    Simple::new(&grid).apply(&mut grid);

    // Create several reusable constructs for walls and floors. Since passing these directly into functions
    // will cause the references to be lost, clone them first, since Tile derives Clone.
    let wall = Tile::new(
      "Wall",
      ' ',
      RGB(40, 40, 40),
      RGB(33, 33, 33),
      tile::Type::Wall(tile::Wall::Normal)
    );

    let floor = Tile::new(
      "Floor",
      ' ',
      RGB(27, 27, 27),
      RGB(20, 20, 20),
      tile::Type::Floor(tile::Floor::Normal)
    );

    // Closure for generating drunkards walks. Basically just makes the process of adding these
    // easy. And since we don't use it anywhere else, it doesn't really make sense to turn it into a function,
    // though it may be in the future.
    
    // This is geared towards eating walls and replacing them with floors, so mainly just to flesh out the dungeon.
    let drunk = |chaos: f32, iter: u32, grid: &mut map::Grid<Tile> | {
      let d = DrunkardsWalk::new(chaos);
      d.apply(
        grid,
        None,
        Some(wall.clone()),
        floor.clone(),
        iter
      )
    };

    // Make three passes of this basic walk to carve caves.

    // Total randomness - Really centralized areas that are mostly opened since it walks over itself a lot
    drunk(1.0, 800, &mut grid);

    // Semi random - A mixture of the previous and next option
    drunk(0.5, 1000, &mut grid);

    // Mostly orderly - Long corridors that occassionally deviate
    drunk(0.25, 1000, &mut grid);

    // Add structures
    Structure::new().apply(&mut grid);

    // Biome generation

    // Mostly just a proof of concept. Biomes are generated by comparing noise maps to the grid then flipping biomes

    // Apply noise for Cave biome
    let mut f1 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.2);
    let bin_grid1 = f1.build();

    // Iterate over the grid
    for x in 0..self.width {
      for y in 0..self.height {
        // Positions of 1 mean that the noise passes the threshold
        if bin_grid1[x][y] == 1 {
          // First flip the biome
          grid[x][y].biome = tile::Biome::Cave;
          // Then recolor based on tile type
          match grid[x][y].tiletype {
            tile::Type::Wall(_) => {
               grid[x][y].set_fg(RGB(67, 57, 57));
               grid[x][y].set_bg(RGB(60, 50, 50));
            },
            tile::Type::Stair(_) | tile::Type::TallGrass => {
              grid[x][y].set_bg(RGB(25, 20, 20));
            },
            _ => {
              grid[x][y].set_fg(RGB(32, 27, 27));
              grid[x][y].set_bg(RGB(25, 20, 20));
            }
          }
        }
      }
    }

    // Apply noise for Crypt biome
    let mut f2 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.2);
    let bin_grid2 = f2.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid2[x][y] == 1 {
          grid[x][y].biome = tile::Biome::Crypt;
          match grid[x][y].tiletype {
            tile::Type::Wall(_) => {
               grid[x][y].set_fg(RGB(57, 57, 57));
               grid[x][y].set_bg(RGB(50, 50, 50));
            },
            tile::Type::Stair(_) | tile::Type::TallGrass => {
              grid[x][y].set_bg(RGB(20, 20, 20));
            }
            _ => {
              grid[x][y].set_fg(RGB(27, 27, 27));
              grid[x][y].set_bg(RGB(20, 20, 20));
            }
          }
        }
      }
    }

    // Apply noise for Sunken biome
    let mut f3 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.4);
    let bin_grid3 = f3.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid3[x][y] == 1 {
          grid[x][y].biome = tile::Biome::Sunken;
          match grid[x][y].tiletype {
            tile::Type::Wall(_) => {
              grid[x][y].set_fg(RGB(57, 57, 67));
              grid[x][y].set_bg(RGB(50, 50, 60));
            },
            tile::Type::Stair(_) | tile::Type::TallGrass => {
              grid[x][y].set_bg(RGB(20, 20, 25));
            }
            _ => {
              grid[x][y].set_fg(RGB(27, 27, 32));
              grid[x][y].set_bg(RGB(20, 20, 25));
            }
          }
        }
      }
    }

    // Apply noise for water
    let mut f4 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.4);
    let bin_grid4 = f4.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid4[x][y] == 1 {
          match grid[x][y].tiletype {
            tile::Type::Wall(_) | tile::Type::Stair(_) => {},
            _ => {
              grid[x][y].set_bg(RGB(57, 144, 255));
              grid[x][y].tiletype = tile::Type::Water;
            }
          }
        }
      }
    }

    // Apply noise for crystal biome
    let mut f5 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.67);
    let bin_grid5 = f5.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid5[x][y] == 1 {
          match grid[x][y].tiletype {
            tile::Type::Floor(_) => {
              grid[x][y] = Tile::new(
                "Crystaline Floor", 
                ' ', 
                RGB(0, 0, 0), 
                RGB(183, 141, 212), 
                tile::Type::Floor(tile::Floor::Crystal)
              );
            },
            tile::Type::Wall(_) => {
              grid[x][y] = Tile::new(
                "Crystaline Wall", 
                ' ', 
                RGB(0, 0, 0), 
                RGB(216, 197, 244), 
                tile::Type::Wall(tile::Wall::Crystal)
              );
            }
            _ => {}
          }
        }
      }
    }

    // Apply noise for tall grass
    let mut f6 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.5);
    let bin_grid6 = f6.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid6[x][y] == 1 {
          match grid[x][y].tiletype {
            tile::Type::Floor(_) => {
              grid[x][y] = Tile::new(
                "Tall Grass", 
                '"', 
                *rand::thread_rng().choose(&GRASS_COLORS).unwrap(), 
                grid[x][y].get_bg(), 
                tile::Type::TallGrass
              );
            },
            _ => {}
          }
        }
      }
    }

    // Apply noise for vines
    let mut f7 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.55);
    let bin_grid7 = f7.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid7[x][y] == 1 {
          match grid[x][y].tiletype {
            tile::Type::Floor(_) => {
              grid[x][y] = Tile::new(
                "Vine", 
                *rand::thread_rng().choose(&VINE_GLYPHS).unwrap(), 
                *rand::thread_rng().choose(&GRASS_COLORS).unwrap(), 
                grid[x][y].get_bg(), 
                tile::Type::Vine
              );
            },
            _ => {}
          }
        }
      }
    }

    // Add pretty details last.

    // Add some hard walls
    for x in 0..self.width {
      for y in 0..self.height {
        // Basically just select 30% of walls to be 'hard walls' which are no different from normal walls
        if grid[x][y].tiletype == tile::Type::Wall(tile::Wall::Normal) {
          let mut rng = rand::thread_rng();
          let chance = rng.gen_range(1, 100);
          if chance > 70 {
            grid[x][y].glyph = '#';
            grid[x][y].tiletype = tile::Type::Wall(tile::Wall::Hard);
          }
        }
      }
    }

    // Add floor features
    for x in 0..self.width {
      for y in 0..self.height {
        if grid[x][y].tiletype == tile::Type::Floor(tile::Floor::Normal) {
          let mut rng = rand::thread_rng();
          let feature_chance = rng.gen_range(1, 100);

          // Create basic rock features
          if feature_chance > 90 {
            let feature = rng.gen_range(1, 5);
            match feature {
              1 => grid[x][y].glyph = ',',
              2 => grid[x][y].glyph = '.',
              3 => grid[x][y].glyph = '%',
              4 => grid[x][y].glyph = '*',
              _ => {}
            };

            // Add foliage in specific biomes
            match grid[x][y].biome {
              tile::Biome::Cave => {
                let foliage_chance = rng.gen_range(1, 100);
                match foliage_chance {
                  1...5 => grid[x][y].set_fg(RGB(76, 74, 45)),
                  6...10 => grid[x][y].set_fg(RGB(35, 30, 30)),
                  11...15 => grid[x][y].set_fg(RGB(76, 74, 45)),
                  16...20 => grid[x][y].set_fg(RGB(76, 74, 45)),
                  _ => {}
                };
              }
              _ => {}
            };
            
          }
        }
      }
    }

    // Add Stairs

    // So I know that get_valid_location() should be deprecated since we started adding stairs
    // but we're just gonna have to live with it for now
    
    // Downstair location
    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Down Stair",
        '>',
        RGB(255, 255, 255),
        RGB(0, 0, 0),
        tile::Type::Stair(tile::Stair::DownStair(tile::DownStair::Normal))
      ),
      loc
    );

    // Stair location
    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Up Stair",
        '<',
        RGB(255, 255, 255),
        RGB(0, 0, 0),
        tile::Type::Stair(tile::Stair::UpStair(tile::UpStair::Normal))
      ),
      loc
    );

    // Add a trap
    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Memory Loss Trap", 
        '^', 
        RGB(255, 255, 0), 
        RGB(0, 0, 0), 
        tile::Type::Trap(tile::Trap::MemoryLoss)
      ),
      loc
    );

    // Or two
    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Shaft", 
        '^', 
        RGB(200, 50, 20), 
        RGB(0, 0, 0), 
        tile::Type::Trap(tile::Trap::Shaft)
      ),
      loc
    );

    // Anotha one
    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Paint bomb", 
        '^', 
        RGB(50, 200, 20), 
        RGB(0, 0, 0), 
        tile::Type::Trap(tile::Trap::PaintBomb)
      ),
      loc
    );

    let loc = Dungeon::get_valid_location(&grid);
    self.add_tile(
      &mut grid,
      &mut Tile::new(
        "Teleport Trap", 
        '^', 
        RGB(50, 127, 200), 
        RGB(0, 0, 0), 
        tile::Type::Trap(tile::Trap::Teleport)
      ),
      loc
    );

    // Spent 300 million years wondering why the map was all walls until I realized this CRUCIAL piece of code
    // suddenly vanished.
    // Don't delete.
    self.grid = grid;

    // Return self
    return self;

  }

  fn generate_grid<T : Clone>(w: usize, h: usize, init: T) -> map::Grid<T> {
    // Make grid
    let mut grid = map::Grid::<T>::new();

    // Fill it with Vecs
    for _x in 0..w {

      // Fill new vecs with init
      let mut vec = Vec::<T>::new();

      for _y in 0..h {
        vec.push(init.clone());
      }

      grid.push(vec);

    }

    return grid;

  }

  pub fn get_bounds_pos(&self) -> Pos {
    Pos::from_usize(self.width, self.height)
  }

  ///
  /// Get the player's starting location as a tuple
  ///
  /// NOTE: Should be deprecated and removed once stairs show up
  ///
  pub fn get_valid_location(grid: &map::Grid<Tile>) -> Pos {
    loop {
      let mut rng = rand::thread_rng();
      let x : usize = rng.gen_range(1, grid.len() - 2);
      let y : usize = rng.gen_range(1, grid[0].len() - 2);

      if tile::spawnable(&grid[x][y]) {
        return Pos::from_usize(x, y);
      }

    }
  }

  ///
  /// Return a new `Dungeon` that consists of pure walls
  ///
  pub fn new(map_dim: Pos) -> Self {

    return Dungeon {
      width: map_dim.x as usize,
      height: map_dim.y as usize,
      grid: map::Grid::new()
    };

  }

}


