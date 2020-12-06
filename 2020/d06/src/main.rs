use std::io::BufRead;

fn main() {
  let result: u32 = std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .scan(0u32, |mask, line| {
      if line.is_empty() {
        let ret = Some(Some(*mask));
        *mask = 0;
        return ret;
      }

      let a_ord: u32 = 'a'.into();
      line.chars().for_each(|c| {
        *mask |= 1 << (u32::from(c) - a_ord);
      });

      Some(None)
    })
    .flatten()
    .map(|group_mask| group_mask.count_ones())
    .sum();

  println!("{}", result);
}
