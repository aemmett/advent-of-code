use std::io::BufRead;
use std::{collections::HashSet, str::FromStr};
use std::{convert::TryFrom, fmt::Debug};

enum Instr {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

impl Debug for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Instr::Nop(v) => f.write_fmt(format_args!("Nop({})", v)),
      Instr::Acc(v) => f.write_fmt(format_args!("Acc({})", v)),
      Instr::Jmp(v) => f.write_fmt(format_args!("Jmp({})", v)),
    }
  }
}

fn main() {
  let prog: Vec<Instr> = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .map(|line| match &line[0..3] {
      "nop" => Instr::Nop(match line.chars().nth(3).unwrap() { '-' => -1, _ => 1 } * i32::from_str(&line[4..]).unwrap()),
      "acc" => Instr::Acc(match line.chars().nth(3).unwrap() { '-' => -1, _ => 1 } * i32::from_str(&line[4..]).unwrap()),
      "jmp" => Instr::Jmp(match line.chars().nth(3).unwrap() { '-' => -1, _ => 1 } * i32::from_str(&line[4..]).unwrap()),
      _ => panic!("Unknown instruction: {}", line),
    })
    .collect();

  let mut acc = 0;
  let mut pc = 0usize;
  let mut seen_pcs = HashSet::<usize>::new();

  loop {
    if !seen_pcs.insert(pc) {
      break;
    }

    println!("{} {} {:?}", pc, acc, prog.get(pc));
    match prog.get(pc) {
      Some(Instr::Nop(_)) => pc += 1,
      Some(Instr::Acc(v)) => {
        pc += 1;
        acc += v;
      }
      Some(Instr::Jmp(v)) => {
        pc = usize::try_from(i32::try_from(pc).unwrap() + v).unwrap();
      }
      None => panic!("program counter outside program"),
    }
  }

  println!("part 1: {}", acc);
}
