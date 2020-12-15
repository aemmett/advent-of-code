use std::collections::HashMap;

fn main() {
  let input = vec![0,13,16,17,1,10,6];
  println!("part 1: {}", mem_game(&input, 2020));
}

fn mem_game(mut starting_nums: &[u32], run_for_turns: u32) -> u32 {  
  let mut num_spoken_on = HashMap::<u32, u32>::new();
  
  let mut turn = 1;
  let mut next_num = 0;
  while turn < run_for_turns {
    let num_to_speak = if starting_nums.is_empty() {
      next_num
    } else {
      let next = starting_nums[0];
      starting_nums = &starting_nums[1..];
      next
    };
    
    next_num = num_spoken_on.get(&num_to_speak).and_then(|t| Some(turn - t)).unwrap_or_default();
    num_spoken_on.insert(num_to_speak, turn);
    turn += 1;
  }

  next_num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example0() { assert_eq!(mem_game(&vec![0,3,6], 10), 0); }
  #[test]
  fn example1() { assert_eq!(mem_game(&vec![1,3,2], 2020), 1); }
  #[test]
  fn example2() { assert_eq!(mem_game(&vec![2,1,3], 2020), 10); }
  #[test]
  fn example3() { assert_eq!(mem_game(&vec![1,2,3], 2020), 27); }
  #[test]
  fn example4() { assert_eq!(mem_game(&vec![2,3,1], 2020), 78); }
  #[test]
  fn example5() { assert_eq!(mem_game(&vec![3,2,1], 2020), 438); }
  #[test]
  fn example6() { assert_eq!(mem_game(&vec![3,1,2], 2020), 1836); }
}