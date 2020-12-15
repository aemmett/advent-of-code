use std::convert::TryFrom;
use std::{collections::HashMap, io::BufRead};
use regex::Regex;

enum Op {
  Mask(u64, u64, u64), // 0, 1, X
  Mem(usize, u64)
}

fn main() {
  let line_re = Regex::new(r"^(?:mask = ([X01]+)|mem\[(\d+)\] = (\d+))$").unwrap();

  let ops: Vec<Op> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .filter_map(|line| {
      let caps = line_re.captures(&line)?;
      if let Some(mask_match) = caps.get(1) {
        let mut zero_mask = 0u64;
        let mut one_mask = 0u64;
        let mut x_mask = 0u64;
        mask_match.as_str().chars()
          .rev()
          .enumerate()
          .for_each(|(i, c)| match c {
            '0' => zero_mask |= 1 << i,
            '1' => one_mask |= 1 << i,
            'X' => x_mask |= 1 << i,
            _ => panic!("invalid mask: {}", mask_match.as_str())
          });
        Some(Op::Mask(zero_mask, one_mask, x_mask))
      } else {
        Some(Op::Mem(caps[2].parse().unwrap(), caps[3].parse().unwrap()))
      }
    })
    .collect();

  let mut mask = 0u64;
  let mut base_val = 0u64;
  let mut mem = HashMap::<usize, u64>::new();

  for op in &ops {
    match op {
      Op::Mask(_, om, xm) => {
        mask = *xm;
        base_val = *om;
      },
      Op::Mem(addr, val) => {
        mem.insert(*addr, val & mask | base_val);
      }
    }
  }

  let part1: u64 = mem.iter().map(|(_, &v)| v).sum();
  println!("part 1: {}", part1);

  mem.clear();
  let mut float_mask = 0u64;
  let mut mem_or_mask = 0usize;

  for op in &ops {
    match op {
      Op::Mask(_, om, xm) => {
        float_mask = *xm;
        mem_or_mask = usize::try_from(*om).unwrap();
      },
      Op::Mem(addr, val) => {
        for mut variant in 0..2usize.pow(float_mask.count_ones()) {
          let mut effective_addr = *addr | mem_or_mask;
          let mut fm = float_mask;
          let mut one_mask = 0usize;
          let mut zero_mask = 0usize;
          let mut i = 0;
          while fm > 0 {
            if fm & 1 == 1 {
              if variant & 1 == 1 {
                one_mask |= 1 << i;
              } else {
                zero_mask |= 1 << i;
              }
              variant >>= 1;
            }

            fm >>= 1;
            i += 1;
          }

          effective_addr &= !zero_mask;
          effective_addr |= one_mask;
          mem.insert(effective_addr, *val);
        }
      }
    }
  }

  println!("part 2: {}", mem.iter().map(|(_, &v)| v).sum::<u64>());
}
