use std::convert::TryFrom;
use std::io::BufRead;

fn main() {
  let mut rows = [0u8; 128];

  std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .filter(|line| !line.is_empty())
    .map(|line| decode(&line))
    .for_each(|id| rows[usize::from(id >> 3)] |= 1 << (id & 7));

  let start = rows.iter().position(|&mask| mask > 0).unwrap() + 1;
  let partial_row_index = start
    + rows
      .iter()
      .skip(start)
      .position(|&mask| mask < 255)
      .unwrap();
  let missing_col = usize::try_from(rows[partial_row_index].trailing_ones()).unwrap();
  println!("{:?}", rows[partial_row_index]);
  println!("{:?}", missing_col);

  println!("{}", partial_row_index << 3 | missing_col);
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
