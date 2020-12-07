use std::io::BufRead;

fn main() {
  let result: u32 = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .scan(u32::MAX, |mask, line| {
      if line.is_empty() {
        let ret = Some(Some(*mask));
        *mask = u32::MAX;
        return ret;
      }

      let a_ord: u32 = 'a'.into();
      let mut answers = 0u32;
      line.chars().for_each(|c| {
        answers |= 1 << (u32::from(c) - a_ord);
      });
      *mask &= answers;

      Some(None)
    })
    .flatten()
    .map(|group_mask| group_mask.count_ones())
    .sum();

  println!("{}", result);
}
