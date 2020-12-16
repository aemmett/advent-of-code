use std::{io::BufRead, ops::{RangeInclusive}};

use regex::Regex;

struct Field {
  label: String,
  range1: RangeInclusive<u32>,
  range2: RangeInclusive<u32>,
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines_iter = stdin
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap());

  let mut fields = Vec::<Field>::new();
  let field_re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
  while let Some(line) = lines_iter.next() {
    let caps = field_re.captures(&line);
    if caps.is_none() {
      break;
    }

    let caps = caps.unwrap();
    fields.push(Field {
      label: caps[1].to_string(),
      range1: caps[2].parse().unwrap()..=caps[3].parse().unwrap(),
      range2: caps[4].parse().unwrap()..=caps[5].parse().unwrap(),
    });
  }

  assert_eq!(lines_iter.next().unwrap(), "your ticket:");

  let my_ticket: Vec<u32> = lines_iter
    .next()
    .unwrap()
    .split(',')
    .map(|n| n.parse().unwrap())
    .collect();
  
  lines_iter.next();
  assert_eq!(lines_iter.next().unwrap(), "nearby tickets:");

  let other_tickets: Vec<Vec<u32>> = lines_iter
    .map(|line| {
      line.split(',').map(|n| n.parse().unwrap()).collect()
    })
    .collect();

  let part1: u32 = other_tickets.iter()
    .flat_map(|t| t)
    .filter(|n| {
      fields.iter().all(|f| !f.range1.contains(n) && !f.range2.contains(n))
    })
    .sum();

  println!("part1: {}", part1);
}
