use std::convert::TryFrom;
use std::io::BufRead;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Dir {
  N,
  S,
  E,
  W,
}

fn main() {
  let rot = vec![Dir::N, Dir::E, Dir::S, Dir::W];

  let mut dir = Dir::E;
  let mut x = 0;
  let mut y = 0;
  std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .for_each(|line| {
      println!("{} {} {:?}", x, y, dir);
      match line.chars().next().unwrap() {
        'N' => y -= line[1..].parse::<i32>().unwrap(),
        'S' => y += line[1..].parse::<i32>().unwrap(),
        'E' => x += line[1..].parse::<i32>().unwrap(),
        'W' => x -= line[1..].parse::<i32>().unwrap(),
        'R' => {
          let deg = line[1..].parse::<usize>().unwrap() / 90;
          dir = rot[(rot.iter().position(|&r| r == dir).unwrap() + 1 * deg) % 4];
        }
        'L' => {
          let deg = line[1..].parse::<i32>().unwrap() / 90;
          dir =
            rot[usize::try_from(
              i32::try_from(rot.iter().position(|&r| r == dir).unwrap()).unwrap() - 1 * deg + 40).unwrap()
              % 4];
        }
        'F' => {
          let dist = line[1..].parse::<i32>().unwrap();
          match dir {
            Dir::N => y -= dist,
            Dir::S => y += dist,
            Dir::E => x += dist,
            Dir::W => x -= dist,
          }
        }
        _ => panic!("invalid command"),
      }
    });

  println!("part 1: {}", x.abs() + y.abs());
}
