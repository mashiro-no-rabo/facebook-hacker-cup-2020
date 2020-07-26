use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::iter::from_fn;

#[derive(Debug)]
struct Restrictions {
  incoming: Vec<bool>,
  outgoing: Vec<bool>,
}

pub fn run() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let mut lines: Vec<&str> = input.trim().lines().skip(1).collect();

  let cases: Vec<String> = lines
    .par_chunks_exact(3)
    .map(|v| {
      let n = v.get(0).unwrap().parse::<u8>().unwrap();
      let inc = v.get(1).unwrap();
      let out = v.get(2).unwrap();

      trips(n, inc, out)
    })
    .collect();

  let mut output = File::create("output.txt").unwrap();
  cases.iter().enumerate().for_each(|(i, case)| {
    output.write_all(format!("Case #{}:\n", i + 1).as_bytes()).unwrap();
    output.write_all(case.as_bytes()).unwrap();
    output.write_all("\n".as_bytes()).unwrap();
  });
}

fn trips(n: u8, inc: &str, out: &str) -> String {
  let mut restrictions = Restrictions {
    incoming: vec![true],
    outgoing: vec![true],
  };

  restrictions.incoming.extend(inc.bytes().map(|b| b == b'Y'));
  restrictions.outgoing.extend(out.bytes().map(|b| b == b'Y'));

  let lines: Vec<String> = (1..=n)
    .into_par_iter()
    .map(|source| {
      let mut line = String::new();

      // build 1..(source-1), backwards
      let mut x = source;
      from_fn(move || {
        x -= 1; // First item is i - 1

        if x > 0 {
          Some(x)
        } else {
          None
        }
      })
      .fold(true, |prev, target| {
        if prev {
          let this = restrictions.incoming[target as usize] && restrictions.outgoing[(target + 1) as usize];
          if this {
            line.insert(0, 'Y');
          } else {
            line.insert(0, 'N');
          }
          this
        } else {
          line.insert(0, 'N');
          prev
        }
      });

      // source to source is always true
      line.push('Y');

      // add (source+1)..n
      (source + 1..=n).fold(true, |prev, target| {
        if prev {
          let this = restrictions.incoming[target as usize] && restrictions.outgoing[(target - 1) as usize];
          if this {
            line.push('Y');
          } else {
            line.push('N');
          }
          this
        } else {
          line.push('N');
          prev
        }
      });

      line
    })
    .collect();

  lines.join("\n")
}

#[cfg(test)]
mod tests {
  use super::trips;

  #[test]
  fn test1() {
    assert_eq!(trips(2, "YY", "YY"), "YY\nYY");
  }

  #[test]
  fn test2() {
    assert_eq!(trips(2, "NY", "YY"), "YY\nNY");
  }

  #[test]
  fn test3() {
    assert_eq!(trips(2, "NN", "YY"), "YN\nNY");
  }

  #[test]
  fn test4() {
    assert_eq!(
      trips(5, "YNNYY", "YYYNY"),
      "YNNNN
YYNNN
NNYYN
NNNYN
NNNYY"
    );
  }

  #[test]
  fn test5() {
    assert_eq!(
      trips(10, "NYYYNNYYYY", "YYNYYNYYNY"),
      "YYYNNNNNNN
NYYNNNNNNN
NNYNNNNNNN
NNYYNNNNNN
NNYYYNNNNN
NNNNNYNNNN
NNNNNNYYYN
NNNNNNYYYN
NNNNNNNNYN
NNNNNNNNYY"
    );
  }
}
