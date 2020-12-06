use std::io::BufRead;

fn main() {
  let result = std::io::stdin().lock().lines()
    .map(|line_res| line_res.unwrap())
    .filter(|line| !line.is_empty())
    .map(|line| decode(&line))
    .max()
    .unwrap();
  println!("{}", result);
}

fn decode(bsp: &str) -> u16 {
  bsp.chars().fold(0, |acc, c| {
    acc << 1
      | match c {
        'B' | 'R' => 1,
        _ => 0,
      }
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_add() {
    assert_eq!(decode("FBFBBFFRLR"), (44 << 3) + 5);
  }
}
