use regex::Regex;
use std::env;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
  let use_new_polilcy = env::args()
    .nth(1)
    .map(|s| bool::from_str(&s).expect("argument must be true or false"))
    .unwrap_or(false);

  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

  let result = std::io::stdin()
    .lock()
    .lines()
    .filter_map(|line_res| line_res.ok())
    .filter_map(|line| {
      re.captures(&line)
        .filter(|c| {
          let n1 = usize::from_str(&c[1]).unwrap();
          let n2 = usize::from_str(&c[2]).unwrap();
          let ch = c[3].chars().next().unwrap();
          let pwd = &c[4];

          if use_new_polilcy {
            compliant_new_policy(n1, n2, ch, pwd)
          } else {
            compliant_old_policy(n1, n2, ch, pwd)
          }
        })
        .and(Some(()))
    })
    .count();
  println!("{}", result);
}

fn compliant_old_policy(n1: usize, n2: usize, ch: char, pwd: &str) -> bool {
  (n1..=n2).contains(&pwd.chars().filter(|&c| c == ch).count())
}

fn compliant_new_policy(n1: usize, n2: usize, ch: char, pwd: &str) -> bool {
  let c1 = pwd.chars().nth(n1 - 1).unwrap();
  let c2 = pwd.chars().nth(n2 - 1).unwrap();
  (c1 == ch) ^ (c2 == ch)
}
