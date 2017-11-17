//!
//! Generate a super simple dungeon
//! 

extern crate rand;
use self::rand::{thread_rng, Rng};

mod rect;
use self::rect::Rect;

mod corr;
use self::corr::Corr;

mod cellular_automata;

mod drunkards_walk;

mod dungeon_tests;

mod simplex;

/// 
/// `Dungeon` struct to generate a bitmap dungeon (1s and 0s)
/// 
/// Q: Why not use booleans?
/// A: Eventually this is going to have more than just 1s and 0s.
/// 
/// * `grid` - 2D Vec of `u8`s
/// * `w` - Width of desired map
/// * `h` - Height of desired map
/// 
/// * `rooms` - Vec of `Rect`s but since `Rect` is private you should be
/// getting `Dungeon`s via `new()`
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Dungeon {

  pub grid: Vec<Vec<u8>>,
  // i32 because of tcod
  pub w: i32,
  pub h: i32,

  // Privatre vector to hold rooms
  rooms: Vec<Rect>,

}

impl Dungeon {

  ///
  /// Add rooms to the `rooms` vec and build them on the `grid`
  /// 
  fn add_rooms(&mut self, n: i32) {

    self.rooms = Vec::<Rect>::new();

    let mut rng = thread_rng();

    for _ in 0..n {

      let x: i32 = rng.gen_range(1, self.w - 2);
      let y: i32 = rng.gen_range(1, self.h - 2);
      let l: i32 = rng.gen_range(5, 20);
      let w: i32 = rng.gen_range(5, 20);

      // Check bounds
      if w + x >= self.w || l + y >= self.h {
        continue;
      } else {
        let r = Rect::new(x, y, l, w);
        self.build_rect(&r);
        self.rooms.push(r);
      }
      
    }
    
  }

  /// 
  /// Build a corridor to the grid
  /// 
  /// Start by moving along the x-axis, then the y-axis
  /// 
  fn build_corr(&mut self, c: &Corr) {

    let mut mover = (c.start.0, c.start.1);

    while mover.0 != c.end.0 {

      if mover.0 < c.end.0 {
        mover.0 += 1;
      } else if mover.0 > c.end.0 {
        mover.0 -= 1;
      } 

      self.grid[mover.0 as usize][mover.1 as usize] = 1;

    }

    while mover.1 != c.end.1 {

      if mover.1 < c.end.1 {
        mover.1 += 1;
      } else if mover.1 > c.end.1 {
        mover.1 -= 1;
      } 

      self.grid[mover.0 as usize][mover.1 as usize] = 1;

    }

  }

  /// 
  /// Build a rectangle to the grid
  /// 
  fn build_rect(&mut self, r: &Rect) {
    for w in 0..r.w {
      for l in 0..r.l {
        self.grid[(w + r.x) as usize][(l + r.y) as usize] = 1;
      }
    }
  }

  /// 
  /// Connect rooms by making `Corr`s then build them.
  /// 
  /// Essentially we connect each room in `rooms` to the next room in the
  /// vector, once we run out we wrap it back around, this in theory
  /// creates a cyclical dungeon with no deadends, but it doesn't happen
  /// consistently due to the way corridors are built (which is a good thing).
  /// 
  fn connect_rooms(&mut self) {

    for r in 0..self.rooms.len() - 1 {

      let c1 : (i32, i32);
      let c2 : (i32, i32);

      // Wrap around
      if r == self.rooms.len() - 1 {

        c1 = self.rooms[r].center().clone();
        c2 = self.rooms[0].center().clone();

      } else {  

        c1 = self.rooms[r].center().clone();
        c2 = self.rooms[r + 1].center().clone();

      }

      self.build_corr(&Corr::new(c1, c2));

    }

  }

  ///
  /// Get the player's starting location as a `Pos`
  /// 
  pub fn get_starting_location(&self) -> (i32, i32) {
    return self.rooms[0].center();
  }

  /// 
  /// Return a new `Dungeon`
  /// 
  pub fn new(w: i32, h: i32, rooms: i32) -> Dungeon {

    // Make a grid
    let mut grid = Vec::<Vec<u8>>::new();

    // Fill it with Vecs
    for _x in 0..w {

      // Fill new vecs with 0s
      let mut vec = Vec::<u8>::new();

      for _y in 0..h {
        vec.push(0);
      }

      grid.push(vec);

    }

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let mut d = Dungeon { 
      grid: grid, 
      rooms: Vec::<Rect>::new(), 
      w: w, 
      h: h 
    };

    // Generate the dungeon
    d.add_rooms(rooms);
    d.connect_rooms();

    for _ in 0..3 {
      d.grid = drunkards_walk::generate(&mut d.grid, 0, 1, 1500);
    }

    let mut sn = simplex::Simplex::new();
    sn.seed();

    // for i in 0..w {
    //   for j in 0..h {
    //     print!("{}", sn.noise_2d(i as f32, j as f32));
    //   }
    //   print!("\n");
    // }
    
    return d;

  }

  /// 
  /// Regenerate the dungeon map
  /// 
  pub fn regen(&mut self) {

    let d = Dungeon::new(self.w, self.h, self.rooms.len() as i32);
    self.grid = d.grid;

  }

}