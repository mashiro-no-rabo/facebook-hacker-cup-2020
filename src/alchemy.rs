use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn run() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let lines: Vec<&str> = input.trim().lines().skip(1).collect();

  let cases: Vec<bool> = lines
    .par_chunks_exact(2)
    .map(|v| {
      let s = v.get(1).unwrap();
      good_alchemy(s)
    })
    .collect();

  let mut output = File::create("output.txt").unwrap();
  cases.into_iter().enumerate().for_each(|(i, case)| {
    let answer = if case { "Y" } else { "N" };
    output
      .write_all(format!("Case #{}: {}\n", i + 1, answer).as_bytes())
      .unwrap();
  });
}

fn good_alchemy(input: &str) -> bool {
  let acc: i32 = input
    .as_bytes()
    .iter()
    .fold(0, |acc, c| if *c == b'A' { acc + 1 } else { acc - 1 });

  acc.abs() == 1
}
