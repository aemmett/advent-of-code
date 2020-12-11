use std::convert::TryInto;
use std::convert::TryFrom;
use std::{io::BufRead, mem};

#[derive(PartialEq, Copy, Clone)]
enum Cell {
  Floor,
  Seat,
  Person,
}

fn main() {
  let mut map: Vec<Vec<Cell>> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '.' => Cell::Floor,
          '#' => Cell::Seat,
          'L' => Cell::Person,
          _ => panic!("Invalid input"),
        })
        .collect::<Vec<Cell>>()
    })
    .collect();

  let rows = map.len();
  let cols = map[0].len();

  // print_map(&map);
  // println!("{}", adjacent_occupancy(&mut map, 2, 0));

  let initial_map: Vec<Vec<Cell>> = map.iter().map(|row| row.to_vec()).collect();
  let mut next_map: Vec<Vec<Cell>> = map.iter().map(|row| row.to_vec()).collect();
  loop {
    // print_map(&map);
    // println!("");

    for y in 0..rows {
      for x in 0..cols {
        let cell = map[y][x];
        if cell == Cell::Floor {
          continue;
        }

        next_map[y][x] = match (cell, adjacent_occupancy(&mut map, x, y)) {
          (Cell::Seat, 0) => Cell::Person,
          (Cell::Person, 4..=9) => Cell::Seat,
          _ => cell,
        }
      }
    }

    if map.iter().zip(&next_map).all(|(a, b)| a.iter().eq(b)) {
      break
    }

    mem::swap(&mut map, &mut next_map);
  }

  let part1 = map.iter().map(|row| {
    row.iter().map(|c| match c {
      Cell::Person => 1,
      _ => 0
    }).sum::<i32>()
  }).sum::<i32>();
  println!("part 1: {}", part1);

  let rays: Vec<(i32, i32)> = vec![
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
  ];

  map = initial_map;
  loop {
    // print_map(&map);
    // println!("");

    for y in 0..rows {
      for x in 0..cols {
        let cell = map[y][x];
        if cell == Cell::Floor {
          continue;
        }

        next_map[y][x] = match (cell, cast_occupancy(&rays, &mut map, x, y)) {
          (Cell::Seat, 0) => Cell::Person,
          (Cell::Person, 5..=9) => Cell::Seat,
          _ => cell,
        }
      }
    }

    if map.iter().zip(&next_map).all(|(a, b)| a.iter().eq(b)) {
      break
    }

    mem::swap(&mut map, &mut next_map);
  }

  let part2 = map.iter().map(|row| {
    row.iter().map(|c| match c {
      Cell::Person => 1,
      _ => 0
    }).sum::<i32>()
  }).sum::<i32>();
  println!("part 2: {}", part2);
}

// fn print_map(map: &[Vec<Cell>]) {
//   map.iter().for_each(|row| {
//     let mut s = String::new();
//     row.iter().for_each(|c| s.push(match c {
//       Cell::Floor => '.',
//       Cell::Seat => 'L',
//       Cell::Person => '#'
//     }));
//     println!("{}", s);
//   });
// }

fn cast_occupancy(rays: &[(i32, i32)], map: &mut Vec<Vec<Cell>>, ix: usize, iy: usize) -> i32 {
  let rows: i32 = map.len().try_into().unwrap();
  let cols: i32 = map[0].len().try_into().unwrap();

  rays.iter().map(|(dx, dy)| {
    let mut x = i32::try_from(ix).unwrap() + dx;
    let mut y = i32::try_from(iy).unwrap() + dy;
    while (0..cols).contains(&x) && (0..rows).contains(&y) {
      match map[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] {
        Cell::Floor => {
          x += dx;
          y += dy;
          continue;
        },
        Cell::Seat => {
          return 0;
        },
        Cell::Person => {
          return 1;
        }
      }
    };

    0
  }).sum()
}

fn adjacent_occupancy(map: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> i32 {
  let mut old_cell = Cell::Floor;
  mem::swap(&mut map[y][x], &mut old_cell);

  let row_r = if y == 0 {
    y..=y + 1
  } else if y == map.len() - 1 {
    y - 1..=y
  } else {
    y - 1..=y + 1
  };

  let ret = map[row_r]
    .iter()
    .flat_map(|row| {
      let col_r = if x == 0 {
        x..=x + 1
      } else if x == map[0].len() - 1 {
        x - 1..=x
      } else {
        x - 1..=x + 1
      };

      row[col_r].iter().map(|c| match c {
        Cell::Person => 1,
        _ => 0,
      })
    })
    .sum::<i32>();

  map[y][x] = old_cell;
  ret
}
