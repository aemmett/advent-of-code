use std::{collections::VecDeque, env, io::BufRead};

fn main() {
  let window_size = env::args()
    .nth(1)
    .and_then(|s| s.parse::<usize>().ok())
    .unwrap_or(2usize);

  let nums: Vec<i64> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| line.parse::<i64>().unwrap())
    .collect();

  let mut window: VecDeque<i64> = nums[0..window_size].iter().copied().collect();
  let mut target_num: Option<i64> = None;
  'iter_loop: for &n in nums.iter().skip(window_size) {
    for j in 0..window_size {
      for k in (j + 1)..window_size {
        if window[j] + window[k] == n {
          window.pop_front();
          window.push_back(n);
          continue 'iter_loop;
        }
      }
    }

    target_num = Some(n);
    println!("part 1: {}", n);
    break;
  }

  if let Some(target) = target_num {
    for i in 0..nums.len() {
      let mut remainder = target;
      for (len, &n) in nums[i..].iter().enumerate() {
        remainder -= n;
        if len > 1 && remainder == 0 {
          let set = &nums[i..i+len];
          let min = set.iter().min().unwrap();
          let max = set.iter().max().unwrap();
          println!("{:?}", set);
          println!("part 2: {}", min + max);
          return;
        }

        if remainder < 0 {
          break;
        }
      }
    }
  }
}
