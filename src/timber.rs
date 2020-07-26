use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

type Position = i64;
type Height = i64;

type Best = HashMap<Position, i64>;

pub fn run() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let mut lines = input.trim().lines();

  let mut output = File::create("output.txt").unwrap();

  let num_cases = lines.next().unwrap().parse::<u8>().unwrap();
  for i_case in 1..=num_cases {
    let n = lines.next().unwrap().parse::<u32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
      let mut line = lines.next().unwrap().split(" ");
      let pos = line.next().unwrap().parse::<i64>().unwrap();
      let height = line.next().unwrap().parse::<i64>().unwrap();
      input.push((pos, height));
    }

    output
      .write_all(format!("Case #{}: {}\n", i_case, combine(input)).as_bytes())
      .unwrap();
  }
}

fn combine(mut input: Vec<(Position, Height)>) -> i64 {
  //
  let x = input.as_mut_slice();
  x.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

  let mut ret = 0;
  let mut best = Best::new();
  x.as_ref().into_iter().for_each(|(pos, height)| {
    let to_right = *best.get(pos).unwrap_or(&0) + height;
    let exist = *best.get(&(*pos + *height)).unwrap_or(&0);
    let r = exist.max(to_right);
    best.insert(pos + height, r);
    ret = ret.max(r);

    let to_left = *best.get(&(*pos - *height)).unwrap_or(&0) + height;
    let exist = *best.get(pos).unwrap_or(&0);
    let l = exist.max(to_left);
    best.insert(*pos, l);
    ret = ret.max(l);
  });

  ret
}
