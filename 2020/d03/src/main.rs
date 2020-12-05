use std::io::BufRead;

fn main() {
  let mut x = 0usize;
  let mut width: Option<usize> = None;
  let result: usize = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .filter_map(|line| {
      if line.is_empty() {
        return None;
      }

      if width.is_none() {
        width = Some(line.chars().count());
      }

      let width = width.unwrap();

      let c = line.chars().nth(x % width).unwrap();
      
      x += 3;
      Some(if c == '#' { 1 } else { 0 })
    })
    .sum();

  println!("{}", result);
}
