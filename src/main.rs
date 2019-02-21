use std::process;

mod problems;

fn main() {
  if let Err(err) = problems::solve_all(false) {
    eprintln!("{}", err);
    process::exit(1);
  };
}
