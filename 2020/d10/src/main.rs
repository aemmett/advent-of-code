use std::{io::BufRead};

fn main() {
  let mut jolts: Vec<i32> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| line.parse::<i32>().unwrap())
    .collect();

  jolts.sort();

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
  println!("{:?}", diffs);
  println!("part 1: {}", diffs.0 * diffs.2);

  let mut blocks = Vec::<Vec::<i32>>::new();

  let mut prev = 0;
  let mut v = vec![0];
  for &j in &jolts {
    if j == prev + 1 {
      v.push(j);
    } else {
      blocks.push(v);
      v = vec![j];
    }

    prev = j;
  }
  blocks.push(v);

  // println!("{:?}", blocks);
  // println!("{:?}", &blocks.map(|b| b.len()).max().unwrap());
  // 1 2 3 4 5
  // 1 2 3  5
  // 1 2  4 5
  // 1  3 4 5
  // 1   4 5
  // 1 2   5
  // 1  3  5
  println!("part 2: {}", blocks.iter().map(|b| {
    match b.len() {
      1 => 1,
      2 => 1,
      3 => 2,
      4 => 4,
      5 => 7,
      _ => panic!("I'm lazy")
    }
  }).product::<u64>())

  // println!("part 2: {}", count_arr(0, &jolts));
}
/*
fn valid_chains(block: &[i32]) -> u64 {
  if block.len() == 1 {
    return 1;
  }

  for &i in &block[1..] {}

  0
}

fn count_arr(start: i32, tail: &[i32]) -> u64 {
  if tail.is_empty() {
    return 1;
  }

  let mut prev = start;
  let mut block = 1;
  for (i, &t) in tail.iter().enumerate() {
    if block <= 3 && t == prev + 1 {
      block += 1
    } else {
      return block * count_arr(t, &tail[i + 1..]);
    }
  }

  return block;

  // tail.iter().enumerate()
  //   .take_while(|(_, &t)| t == start + 1)
  //   .map(|(i, &t)| count_arr(t, &tail[i+1..]))
  //   .sum()
}
*/