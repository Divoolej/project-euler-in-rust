fn reverse(mut n: u32) -> u32 {
  let mut reversed = 0;
  while n > 0 {
    reversed = 10 * reversed + n % 10;
    n /= 10
  }
  reversed
}

fn is_palindrome(n: u32) -> bool { n == reverse(n) }

fn largest_palindrome() -> String {
  let mut largest = 0;
  let mut i = 999;
  let mut j;
  let mut delta_j;
  while i >= 100 {
    if i % 11 == 0 {
      j = 999;
      delta_j = 1;
    } else {
      j = 990;
      delta_j = 11;
    }
    while j >= i {
      let k = i * j;
      if k < largest { break; }
      if is_palindrome(k) { largest = k; }
      j -= delta_j;
    }
    i -= 1;
  }
  largest.to_string()
}

pub fn solve() -> String {
  largest_palindrome()
}
