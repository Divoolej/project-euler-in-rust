const LIMIT: u32 = 4_000_000;

pub fn solve() -> String {
  let mut sum = 0;
  let mut a = 1;
  let mut b = 1;
  let mut c;
  loop {
    c = a + b; a = b; b = c;
    if c < LIMIT { sum += c; } else { break; }
    c = a + b; a = b; b = c;
    c = a + b; a = b; b = c;
  }
  sum.to_string()
}
