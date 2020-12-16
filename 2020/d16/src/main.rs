use std::{io::BufRead, ops::{RangeInclusive}};

use regex::Regex;

struct Field {
  label: String,
  range1: RangeInclusive<u32>,
  range2: RangeInclusive<u32>,
  pos: Option<usize>
}

impl Field {
  fn new(label: String, a: u32, b: u32, c: u32, d: u32) -> Field {
    Field {
      label,
      range1: a..=b,
      range2: c..=d,
      pos: None,
    }
  }

  fn accepts(&self, n: &u32) -> bool {
    self.range1.contains(n) || self.range2.contains(n)
  }
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
    fields.push(Field::new(
      caps[1].to_string(),
      caps[2].parse().unwrap(),
      caps[3].parse().unwrap(),
      caps[4].parse().unwrap(),
      caps[5].parse().unwrap()));
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

  let (valid_tickets, invalid_tickets): (Vec<_>, Vec<_>) = lines_iter
    .map(|line| {
      line.split(',').map(|n| n.parse().unwrap()).collect::<Vec<u32>>()
    })
    .partition(|ticket| {
      ticket.iter().all(|n| fields.iter().any(|f| f.accepts(n)))
    });

  let part1: u32 = invalid_tickets.iter()
    .flatten()
    .filter(|&n| {
      fields.iter().all(|f| !f.accepts(n))
    })
    .sum();

  println!("part1: {}", part1);

  let n_cols: Vec<Vec<u32>> = (0..my_ticket.len())
    .map(|pos| {
      valid_tickets.iter().map(|t| t[pos]).collect()
    })
    .collect();

  while fields.iter().any(|f| f.pos.is_none()) {
    let pos_fi: Vec<(usize, usize)> = n_cols.iter()
      .enumerate()
      .filter(|(pos, _)| {
        !fields.iter().any(|f| f.pos.map_or(false, |fp| fp == *pos))
      })
      .filter_map(|(pos, col)| {
        // println!("considering col {:?}", col);
        let mut cfi: Option<usize> = None;
        for (fi, f) in fields.iter().enumerate().filter(|(_, f)| f.pos.is_none()) {
          if col.iter().all(|n| f.accepts(n)) {
            // println!("all values accepted by {}", f.label);
            if cfi.is_some() {
              return None; // at least two valid fields
            }

            cfi = Some(fi);
          }
        }
        
        assert!(cfi.is_some());

        Some((pos, cfi.unwrap()))
      })
      .collect();

    assert!(!pos_fi.is_empty());
    
    for &(pos, fi) in &pos_fi {
      // println!("setting {} to pos {}", fields[fi].label, pos);
      fields[fi].pos = Some(pos);
    }
  }

  let part2: u64 = fields.iter()
    .filter(|f| f.label.starts_with("departure"))
    .map(|f| u64::from(my_ticket[f.pos.unwrap()]))
    .product();

  println!("part 2: {}", part2);
}
