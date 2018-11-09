const LIMIT: u32 = 999;

fn sum_multiples_of(n: u32) -> u32 {
  (LIMIT / n) * ((LIMIT / n) * n + n) / 2
}

pub fn solve() -> String {
  (sum_multiples_of(3) + sum_multiples_of(5) - sum_multiples_of(15)).to_string()
}
