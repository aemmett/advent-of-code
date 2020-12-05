use regex::Regex;
use std::convert::TryInto;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

  let result: i32 = std::io::stdin()
    .lock()
    .lines()
    .filter_map(|line_res| line_res.ok())
    .filter_map(|line| {
      re.captures(&line).map(|c| {
        let min_occurs = i32::from_str(&c[1]).unwrap();
        let max_occurs = i32::from_str(&c[2]).unwrap();
        let ch = c[3].chars().next().unwrap();
        let pwd = &c[4];

        if (min_occurs..=max_occurs)
          .contains(&pwd.chars().filter(|&c| c == ch).count().try_into().unwrap())
        {
          1
        } else {
          0
        }
      })
    })
    .sum();
  println!("{}", result);
}

/*
For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/
