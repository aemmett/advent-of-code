use std::convert::TryFrom;
use std::io::BufRead;

fn main() {
  let lines: Vec<String> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .collect();
/*
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
  println!("part 1: {}", part1.0 * (part1.1 - wait));*/

  let mut buses: Vec<(u64, u64)> = lines[1]
    .split(',')
    .enumerate()
    .filter_map(|(i, id)| {
      if id == "x" {
        return None;
      }

      // target_offset, id
      Some((u64::try_from(i).unwrap(), id.parse::<u64>().unwrap()))
    })
    .collect();

  buses.sort_by(|a, b| b.1.cmp(&a.1));

  let adv = buses[0].1;
  let ao_inv: Vec<u64> = buses
    .iter()
    .skip(1)
    .map(|&(_, id)| {
      let ao = id - (adv % id);
      // print!("{}, ", ao);
      for x in 1..id {
        if ao*x % id == 1 {
          return x
        }
      }

      panic!("could not compute multiplicative inverse for {} in GF({})", ao, id);
    })
    .collect();

  // println!("{:?}", ao_inv);

  // let mut ts = 1068700u64;
  let mut ts = 100000000000000u64;
  ts = ts - (ts % adv) - buses[0].0;
  ts += adv;
  loop {
    // println!("{}: {:?}", ts, buses.iter().skip(1).map(|&(to, id)| {
    //   let o = next_depart_after(id, ts);
    //   format!("{}: {}-{}={}", id, to, o, (to + (id-1)*o) % id)
    // }).collect::<Vec<String>>());

    let cycles_to_advance = buses.iter()
      .skip(1)
      .enumerate()
      // .inspect(|&(i, &(targ_offset, id))| {
      //   let offset = id - ((ts - 1) % id) - 1;
      //   println!("")
      //   ((targ_offset + (id-1)*offset) * ao_inv[i]) % id
      // })
      .map(|(i, &(targ_offset, id))| {
        let offset = next_depart_after(id, ts);
        let offset_diff = (targ_offset + (id-1)*offset) % id;
        (offset_diff * ao_inv[i]) % id
      })
      // .inspect(|&cta| {
      //   print!("{}, ", cta);
      // })
      .max()
      .unwrap();

    // println!("max: {}", cycles_to_advance);

    if cycles_to_advance == 0 {
      break;
    }

    ts += adv * cycles_to_advance;
  }

  println!("part 2: {}", ts);
}

fn next_depart_after(id: u64, ts: u64) -> u64 {
  id - ((ts - 1) % id) - 1
}
