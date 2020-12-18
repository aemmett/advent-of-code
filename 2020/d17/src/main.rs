use std::{cmp::{max, min}, collections::BTreeSet, convert::TryInto, io::BufRead, mem, ops::Add};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Coord(i32, i32, i32);

impl std::ops::AddAssign for Coord {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Coord {
  fn zero() -> Coord {
    Coord(0, 0, 0)
  }
}

impl Add for Coord {
  type Output = Self;

  fn add(self, rhs: Coord) -> Self::Output {
    Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

fn main() {
  let mut active_cubes = BTreeSet::<Coord>::new();

  std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .enumerate()
    .for_each(|(y, line)| {
      line.chars().enumerate().for_each(|(x, c)| {
        if c == '#' {
          active_cubes.insert(Coord(x.try_into().unwrap(), y.try_into().unwrap(), 0));
        }
      })
    });

  // println!("{:?}", &active_cubes);
  // print_state(&active_cubes);

  let mut next_ac = BTreeSet::<Coord>::new();
  for cycle in 1..=6 {
    let (mut min_pos, mut max_pos) = &active_cubes
      .iter()
      .fold((Coord::zero(), Coord::zero()), |(min_c, max_c), c| {
        (Coord(
          min(min_c.0, c.0),
          min(min_c.1, c.1),
          min(min_c.2, c.2),
        ), Coord(
          max(max_c.0, c.0),
          max(max_c.1, c.1),
          max(max_c.2, c.2),
        ))
      });
    min_pos += Coord(-1, -1, -1);
    max_pos += Coord(1, 1, 1);
    
    for x in min_pos.0..=max_pos.0 {
      for y in min_pos.1..=max_pos.1 {
        for z in min_pos.2..=max_pos.2 {
          let pos = Coord(x, y, z);
          match (active_cubes.contains(&pos), count_active_neighbors(pos, &active_cubes)) {
            (true, 2..=3) => next_ac.insert(pos),
            (false, 3) => next_ac.insert(pos),
            _ => false
          };
        }
      }
    }

    // println!("after cycle {}", cycle);
    // print_state(&next_ac);

    mem::swap(&mut active_cubes, &mut next_ac);
    next_ac.clear();
  }

  println!("part 1: {}", active_cubes.iter().count());
}

fn print_state(active_cubes: &BTreeSet<Coord>) {
  let (min_pos, max_pos) = &active_cubes
    .iter()
    .fold((Coord::zero(), Coord::zero()), |(min_c, max_c), c| {
      (Coord(
        min(min_c.0, c.0),
        min(min_c.1, c.1),
        min(min_c.2, c.2),
      ), Coord(
        max(max_c.0, c.0),
        max(max_c.1, c.1),
        max(max_c.2, c.2),
      ))
    });
  for z in min_pos.2..=max_pos.2 {
    println!("z={}", z);
    for y in min_pos.1..=max_pos.1 {
      for x in min_pos.0..=max_pos.0 {
        print!("{}", if active_cubes.contains(&Coord(x, y, z)) {"#"} else {"."});
      }
      println!()
    }
    println!()
  }
}

static NEIGHBOR_DELTAS: [Coord; 26] = [
  Coord(-1, -1, -1),
  Coord(-1, -1,  0),
  Coord(-1, -1,  1),
  Coord(-1,  0, -1),
  Coord(-1,  0,  0),
  Coord(-1,  0,  1),
  Coord(-1,  1, -1),
  Coord(-1,  1,  0),
  Coord(-1,  1,  1),
  Coord( 0, -1, -1),
  Coord( 0, -1,  0),
  Coord( 0, -1,  1),
  Coord( 0,  0, -1),
  
  Coord( 0,  0,  1),
  Coord( 0,  1, -1),
  Coord( 0,  1,  0),
  Coord( 0,  1,  1),
  Coord( 1, -1, -1),
  Coord( 1, -1,  0),
  Coord( 1, -1,  1),
  Coord( 1,  0, -1),
  Coord( 1,  0,  0),
  Coord( 1,  0,  1),
  Coord( 1,  1, -1),
  Coord( 1,  1,  0),
  Coord( 1,  1,  1),
];

fn count_active_neighbors(pos: Coord, active_cubes: &BTreeSet<Coord>) -> usize {
  NEIGHBOR_DELTAS.iter().filter(|&&d| active_cubes.contains(&(pos + d))).count()
}