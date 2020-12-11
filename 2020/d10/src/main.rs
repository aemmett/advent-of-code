use std::convert::TryFrom;
use std::{io::BufRead, iter};

fn main() {
  let mut jolts: Vec<u32> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| line.parse::<u32>().unwrap())
    .collect();

  jolts.sort_unstable();

  let mut diffs = (0, 0, 1);
  let mut prev = 0;
  for &j in &jolts {
    *match j - prev {
      1 => &mut diffs.0,
      2 => &mut diffs.1,
      3 => &mut diffs.2,
      _ => panic!("diff too far"),
    } += 1;
    prev = j;
  }
  println!("part 1: {}", diffs.0 * diffs.2);

  jolts.insert(0, 0);

  let mut tail = &jolts[..];
  let blocks = iter::from_fn(|| {
    if tail.is_empty() {
      return None
    }

    let start = tail[0];
    let end = tail.iter().enumerate()
      .take_while(|(i, &x)| x - start == u32::try_from(*i).unwrap())
      .last()
      .unwrap()
      .0;

    let ret = Some(&tail[0..end+1]);
    tail = &tail[end+1..];
    ret
  });

  println!("part 2: {}", blocks.map(|b| {
    match b.len() {
      1 => 1,
      2 => 1,
      3 => 2,
      4 => 4,
      5 => 7,
      _ => panic!("I'm lazy")
    }
  }).product::<u64>())
}
