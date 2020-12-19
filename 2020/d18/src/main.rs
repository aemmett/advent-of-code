use std::io::BufRead;

use regex::{CaptureMatches, Regex};

fn main() {
  let token_re = Regex::new(r"(\d+)|\(|\)|\+|\*").unwrap();
  let part1: u64 = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| {
      let mut token_iter = token_re.captures_iter(&line);
      eval_expr(&mut token_iter)
    })
    .sum();

  println!("part 1: {}", part1);
}

#[derive(Debug)]
enum Op {
  Add,
  Mul,
}

enum Item {
  Op(Op),
  Val(u64),
}

fn eval_expr(mut tokens: &mut CaptureMatches) -> u64 {
  let mut stack: Vec<Item> = vec![];
  let mut op: Option<Op> = None;
  while let Some(token) = tokens.next() {
    let token = token.get(0).unwrap().as_str();
    // println!("{}", token);
    if token == ")" {
      break;
    }

    match token {
      "(" => {
        stack.push(Item::Val(eval_expr(&mut tokens)));
        if let Some(o) = op.take() {
          stack.push(Item::Op(o));
        }
      },
      "+" => { assert!(op.replace(Op::Add).is_none()); },
      "*" => { assert!(op.replace(Op::Mul).is_none()); },
      _ => {
        stack.push(Item::Val(token.parse().expect("expected a number")));
        if let Some(o) = op.take() {
          stack.push(Item::Op(o));
        }
      }
    };
  }

  calc_val(&mut stack)
}

fn calc_val(mut stack: &mut Vec<Item>) -> u64 {
  match stack.pop().unwrap() {
    Item::Val(val) => val,
    Item::Op(Op::Add) => calc_val(&mut stack) + calc_val(&mut stack),
    Item::Op(Op::Mul) => calc_val(&mut stack) * calc_val(&mut stack),
  }
}
