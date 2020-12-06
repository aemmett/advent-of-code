use std::io::Read;
use std::{collections::HashMap};

// enum CredKey {
//   Byr, // Birth Year
//   Iyr, // Issue Year
//   Eyr, // Expiration Year
//   Hgt, // Height
//   Hcl, // Hair Color
//   Ecl, // Eye Color
//   Pid, // Passport ID
//   Cid, // Country ID
// }

fn main() {
  let required_fields = vec![
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
  ];

  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();
  let creds = input.lines()
    .scan(None, |cred, line| {
      if line.is_empty() {
        return Some(cred.take());
      }

      if cred.is_none() {
        *cred = Some(HashMap::<&str, &str>::new());
      }

      let map = cred.as_mut().unwrap();
      line.split(' ').for_each(|pair| {
        let mut iter = pair.split(':');
        map.insert(iter.next().unwrap(), iter.next().unwrap());
      });

      Some(None)
    })
    .flatten()
    .filter(|cred| {
      required_fields.iter().all(|&key| cred.contains_key(key))
    })
    .count();

  println!("{}", creds);
}
