#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::str::FromStr;
use std::{io::Read, env};

struct Credential<'a> {
  byr: Option<&'a str>,
  iyr: Option<&'a str>,
  eyr: Option<&'a str>,
  hgt: Option<&'a str>,
  hcl: Option<&'a str>,
  ecl: Option<&'a str>,
  pid: Option<&'a str>,
  cid: Option<&'a str>,
}

impl Credential<'_> {
  fn new<'a>() -> Credential<'a> {
    Credential {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None,
    }
  }

  fn has_required_fields(&self) -> bool {
    self.byr
      .and(self.iyr)
      .and(self.eyr)
      .and(self.hgt)
      .and(self.hcl)
      .and(self.ecl)
      .and(self.pid)
      .is_some()
  }

  fn has_valid_values(&self) -> bool {
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.*/
    lazy_static! {
      static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
      static ref HCL_RE: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
      static ref ECL_RE: Regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
      static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

       self.byr.and_then(|byr| i32::from_str(byr).ok()).map_or(false, |byr| (1920..=2002).contains(&byr))
    && self.iyr.and_then(|iyr| i32::from_str(iyr).ok()).map_or(false, |iyr| (2010..=2020).contains(&iyr))
    && self.eyr.and_then(|eyr| i32::from_str(eyr).ok()).map_or(false, |eyr| (2020..=2030).contains(&eyr))
    && self.hgt.and_then(|hgt| HGT_RE.captures(hgt)).and_then(|c|
        match (i32::from_str(&c[1]).ok()?, &c[2]) {
          (150..=193, "cm") => Some(()),
          (59..=76, "in") => Some(()),
          _ => None
        }
      ).is_some()
    && self.hcl.map_or(false, |hcl| HCL_RE.is_match(hcl))
    && self.ecl.map_or(false, |ecl| ECL_RE.is_match(ecl))
    && self.pid.map_or(false, |pid| PID_RE.is_match(pid))
  }
}

fn main() {
  let validate_values = env::args()
    .nth(1)
    .map(|s| bool::from_str(&s).expect("argument must be true or false"))
    .unwrap_or(false);

  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();
  let creds = input.lines()
    .scan(None, |cred, line| {
      if line.is_empty() {
        return Some(cred.take());
      }

      if cred.is_none() {
        *cred = Some(Credential::new());
      }

      let cred = cred.as_mut().unwrap();
      line.split(' ').for_each(|pair| {
        let mut iter = pair.split(':');
        let key = iter.next().unwrap();
        let val = iter.next();
        *match key {
          "byr" => &mut cred.byr,
          "iyr" => &mut cred.iyr,
          "eyr" => &mut cred.eyr,
          "hgt" => &mut cred.hgt,
          "hcl" => &mut cred.hcl,
          "ecl" => &mut cred.ecl,
          "pid" => &mut cred.pid,
          "cid" => &mut cred.cid,
          k => panic!("unrecognized key: {}: {}", k, val.unwrap_or("(none)")),
        } = val;
      });

      Some(None)
    })
    .flatten()
    .filter(|cred| {
      cred.has_required_fields() && (!validate_values || cred.has_valid_values())
    })
    .count();

  println!("{}", creds);
}
