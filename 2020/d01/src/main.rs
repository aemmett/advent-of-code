use std::env;
use std::io::{self};
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
  let sum_target = 2020;

  let tuple_size = env::args().nth(1).map(|s| usize::from_str(&s).unwrap()).unwrap_or(2);

  let mut input: Vec<u32> = io::stdin().lock().lines()
    // parse each line into a u32, ignore inputs greater than 2020
    .filter_map(|line| u32::from_str(&line.unwrap()).ok().filter(|&x| x <= sum_target))
    .collect();
  input.sort();
  
  let mut result = vec![0u32; tuple_size];
  
  find_sum_tuple(&mut result, tuple_size - 1, sum_target, &input);
  println!("{}", result.iter().product::<u32>());
}

fn find_sum_tuple(tuple: &mut[u32], tuple_pos: usize, remainder: u32, nums: &[u32]) -> bool {
  if tuple_pos == 0 {
    if let Ok(elem) = nums.binary_search(&remainder) {
      tuple[0] = nums[elem];
      return true;
    }
    
    return false;
  }

  let mut sub_nums = nums;
  for i in 0..nums.len() {
    let elem = nums[i];
    let new_remainder = remainder - elem;
    // trim subslice to exclude elements greater than the new remainder
    let end = sub_nums.binary_search(&new_remainder).unwrap_or_else(|x| x - 1);
    sub_nums = &sub_nums[1..end];
    if find_sum_tuple(tuple, tuple_pos - 1, new_remainder, nums) {
      tuple[tuple_pos] = elem;
      return true;
    }
  }

  false
}
