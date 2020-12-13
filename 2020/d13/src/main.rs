use std::convert::TryFrom;
use std::io::BufRead;

fn main() {
  let lines: Vec<String> = std::io::stdin()
    .lock()
    .lines()
    .take(2)
    .map(|line_res| line_res.unwrap())
    .collect();

  let wait = lines[0].parse::<u32>().unwrap();
  let buses: Vec<u32> = lines[1]
    .split(',')
    .filter_map(|id| {
      if id == "x" {
        return None;
      }

      Some(id.parse::<u32>().unwrap())
    })
    .collect();

  let part1 = buses
    .iter()
    .map(|id| (id, wait + id - ((wait - 1) % id) - 1))
    // .inspect(|x| {
    //   println!("{:?}", x);
    // })
    .min_by(|&a, &b| (a.1).cmp(&b.1))
    .unwrap();
  // println!("{:?}", part1);
  println!("part 1: {}", part1.0 * (part1.1 - wait));

  let mut buses: Vec<(u64, u64)> = lines[1]
    .split(',')
    .enumerate()
    .filter_map(|(i, id)| {
      if id == "x" {
        return None;
      }

      let id = id.parse::<u64>().unwrap();
      Some((u64::try_from(i).unwrap() % id, id))
    })
    .collect();

  buses.sort_by(|&a, &b| b.1.cmp(&a.1));

  let mut ts = 0u64;
  let mut adv = 1;
  let mut bus_i = 0usize;
  while bus_i < buses.len() {
    ts += adv;
    let (target_offset, id) = buses[bus_i];
    if next_depart_after(id, ts) == target_offset {
      adv *= id;
      bus_i += 1;
    }
  }

  println!("part 2: {}", ts);
}

fn next_depart_after(id: u64, ts: u64) -> u64 {
  id - ((ts - 1) % id) - 1
}
