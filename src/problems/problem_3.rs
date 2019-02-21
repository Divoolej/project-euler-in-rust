const NUMBER: i64 = 600851475143;

fn largest_prime_divisor(mut n: i64) -> i64 {
  let mut last_factor = 1;
  let mut factor = 3;

  if n % 2 == 0 {
    last_factor = 2;
    while n % 2 == 0 { n /= 2; }
  }

  while n > 1 {
    if n % factor == 0 {
      last_factor = factor;
      while n % factor == 0 { n /= factor }
    }
    factor += 2;
  }

  last_factor
}

pub fn solve() -> String {
  largest_prime_divisor(NUMBER).to_string()
}
