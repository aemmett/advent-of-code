use std::{io::BufRead};
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
  let mut prog: Vec<Instr> = std::io::stdin()
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

  println!("part 1: {}", try_run(&prog).0);

  for i in 0..prog.len() {
    match &prog[i] {
      Instr::Acc(_) => continue,
      _ => {
        swap_instr(&mut prog[i]);
        if let (acc, true) = try_run(&prog) {
          println!("part 2: {}", acc);
          return;
        }
        swap_instr(&mut prog[i]);
      }
    }
  }
}

fn swap_instr(ins: &mut Instr) {
  *ins = match *ins {
    Instr::Nop(v) => Instr::Jmp(v),
    Instr::Jmp(v) => Instr::Nop(v),
    _ => panic!("shouldn't happen")
  }
}

fn try_run(prog: &[Instr]) -> (i32, bool) {
  let mut acc = 0;
  let mut pc = 0usize;
  let mut seen_pcs = HashSet::<usize>::new();

  loop {
    if pc == prog.len() {
      return (acc, true)
    }

    if !seen_pcs.insert(pc) {
      return (acc, false)
    }

    match &prog[pc] {
      Instr::Nop(_) => pc += 1,
      Instr::Acc(v) => {
        pc += 1;
        acc += v;
      }
      Instr::Jmp(v) => {
        pc = usize::try_from(i32::try_from(pc).unwrap() + v).unwrap();
      }
    }
  }
}
