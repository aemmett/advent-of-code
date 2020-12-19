use std::io::BufRead;

use regex::{CaptureMatches, Regex};

fn main() {
  let token_re = Regex::new(r"(\d+)|\(|\)|\+|\*").unwrap();
  let part2: u64 = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| {
      let mut lexer = Lexer {
        capture_iter: token_re.captures_iter(&line),
        current_token: None
      };
      lexer.advance();
      pratt(&mut lexer, 0)
    })
    .sum();

  println!("part 2: {}", part2);
}

struct Lexer<'r, 't> {
  capture_iter: CaptureMatches<'r, 't>,
  current_token: Option<&'t str>
}

impl<'t> Lexer<'_, 't> {
  fn advance(&mut self) -> Option<&'t str> {
    let prev_token = self.next_token();
    self.current_token = self.capture_iter.next().map(|c| c.get(0).unwrap().as_str());
    prev_token
  }
  fn next_token(&self) -> Option<&'t str> {
    self.current_token
  }
}

fn pratt(mut lexer: &mut Lexer, rbp: u8) -> u64 {
  let token = lexer.advance().unwrap();
  let mut left = match token {
    "(" => {
      let val = pratt(&mut lexer, 0);
      assert_eq!(lexer.advance().unwrap(), ")");
      val
    },
    _ => token.parse().unwrap()
  };

  while let Some(token) = lexer.next_token() {
    if rbp >= lbp_of(token) {
      break;
    }

    lexer.advance();
    left = do_op(left, token, lexer);
  }

  left
}

fn lbp_of(t: &str) -> u8 {
  match t {
    "*" => 1,
    "+" => 3,
    _ => 0,
  }
}

fn do_op(lhs: u64, token: &str, mut lexer: &mut Lexer) -> u64 {
  let rhs = pratt(&mut lexer, lbp_of(token));
  match token {
    "+" => lhs + rhs,
    "*" => lhs * rhs,
    _ => panic!("unknown operator")
  }
}
