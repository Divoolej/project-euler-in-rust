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

// fn largest_prime_divisor(n: i64) -> i64 {
//   for i in ((n as f64).sqrt().floor() as i64)..=(n / 2) {
//     if i == 1 { return n; }
//     if n % i == 0 {
//       let a = largest_prime_divisor(i);
//       let b = if n != n / i { largest_prime_divisor(n / i) } else { a };
//       return if a > b { a } else { b }
//     }
//   }
//   n
// }

pub fn solve() -> String {
  largest_prime_divisor(NUMBER).to_string()
}
