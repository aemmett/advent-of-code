use std::io::BufRead;

fn main() {
  let rot_matrix = vec![
    (1, 0, 1, 0),
    (0, -1, 1, 0),
    (-1, 0, 0, -1),
    (0, 1, -1, 0),
  ];

  let mut x = 0;
  let mut y = 0;
  let mut wx = 10;
  let mut wy = -1;
  std::io::stdin()
    .lock()
    .lines()
    .map(|line_res| line_res.unwrap())
    .for_each(|line| {
      // println!("{} {} {} {} {}", line, x, y, wx, wy);
      match line.chars().next().unwrap() {
        'N' => wy -= line[1..].parse::<i32>().unwrap(),
        'S' => wy += line[1..].parse::<i32>().unwrap(),
        'E' => wx += line[1..].parse::<i32>().unwrap(),
        'W' => wx -= line[1..].parse::<i32>().unwrap(),
        'R' => {
          let ix = wx;
          let iy = wy;
          let rot = (line[1..].parse::<usize>().unwrap() / 90) % 4;
          let rot_mat = rot_matrix[rot];
          wx = ix*rot_mat.0 + iy*rot_mat.1;
          wy = ix*rot_mat.2 + iy*rot_mat.3;
        }
        'L' => {
          let ix = wx;
          let iy = wy;
          let rot = (40 - (line[1..].parse::<usize>().unwrap() / 90)) % 4;
          let rot_mat = rot_matrix[rot];
          // println!("{} {:?}", rot, rot_mat);
          wx = ix*rot_mat.0 + iy*rot_mat.1;
          wy = ix*rot_mat.2 + iy*rot_mat.3;
        }
        'F' => {
          let dist = line[1..].parse::<i32>().unwrap();
          x += dist * wx;
          y += dist * wy;
        }
        _ => panic!("invalid command"),
      }
    });
  // println!("{} {} {} {}", x, y, wx, wy);

  println!("part 2: {}", x.abs() + y.abs());
}
