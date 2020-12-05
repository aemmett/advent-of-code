use std::io::BufRead;

struct SlopeChecker {
  x: usize,
  y: usize,
  dx: usize,
  dy: usize,
  trees: usize,
}

impl SlopeChecker {
  fn new(dx: usize, dy: usize) -> SlopeChecker {
    SlopeChecker {
      x: 0,
      y: 0,
      dx,
      dy,
      trees: 0,
    }
  }

  fn update(&mut self, line: &str, width: usize) {
    if self.y % self.dy != 0 {
      self.y += 1;
      return;
    }
    let c = line.chars().nth(self.x % width).unwrap();
    self.trees += if c == '#' { 1 } else { 0 };

    self.x += self.dx;
    self.y += 1;
  }
}

fn main() {
  let mut slopes = vec![
    SlopeChecker::new(1, 1),
    SlopeChecker::new(3, 1),
    SlopeChecker::new(5, 1),
    SlopeChecker::new(7, 1),
    SlopeChecker::new(1, 2),
  ];

  let mut width: Option<usize> = None;
  std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .for_each(|line| {
      if line.is_empty() {
        return;
      }

      if width.is_none() {
        width = Some(line.chars().count());
      }

      let width = width.unwrap();

      slopes.iter_mut().for_each(|s| s.update(&line, width));
    });

  slopes.iter().for_each(|s| println!("{}", s.trees));
  println!("{}", slopes.iter().map(|s| s.trees).product::<usize>());
}
