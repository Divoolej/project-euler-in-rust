use std::collections::HashMap;
use std::io::Write;
use std::fs;

mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;

const PROBLEMS: u32 = 5;

const OUTPUT_PATH: &'static str = "data/output.txt";
const SOLUTIONS_PATH: &'static str = "data/solutions.txt";

fn solve(problem_number: u32) -> Result<String, String> {
  match problem_number {
    1 => Ok(problem_1::solve()),
    2 => Ok(problem_2::solve()),
    3 => Ok(problem_3::solve()),
    4 => Ok(problem_4::solve()),
    5 => Ok(problem_5::solve()),
    _ => Err(format!("No implementation for Problem {}", problem_number))
  }
}

pub fn solve_all(cached: bool) -> Result<(), String> {
  let mut results = if cached {
    get_problems_map(read_results()?, OUTPUT_PATH)?
  } else {
    HashMap::new()
  };
  let solutions = get_problems_map(read_solutions()?, SOLUTIONS_PATH)?;

  solve_problems(&mut results, solutions)?;
  save_results(&results)?;

  Ok(())
}

fn get_problems_map(content: String, filename: &str) -> Result<HashMap<u32, String>, String> {
  let mut output_map = HashMap::new();
  for (i, line) in content.lines().enumerate() {
    if let Some(s) = line.split("Problem ").skip(1).collect::<Vec<_>>().pop() {
      let parsed_line: Vec<_> = s.split(": ").collect();
      if parsed_line.len() == 2 {
        let problem_number = parsed_line[0].parse().or_else(|_| {
          Err(format!("Could not parse Problem number at line {}: \"{}\"", i + 1, parsed_line[0]))
        })?;
        let problem_output = parsed_line[1].to_string();
        if let Some(_) = output_map.get(&problem_number) {
          return Err(
            format!("Multiple values for Problem {}", problem_number),
          );
        }
        output_map.insert(problem_number, problem_output);
        continue;
      }
    }
    return Err(
      format!("Error parsing \"{}\" at line {}: \"{}\"", filename, i + 1, line),
    );
  }
  Ok(output_map)
}

fn read_results() -> Result<String, String> {
  fs::read_to_string(OUTPUT_PATH).or_else(|err| {
    Err(format!("Error opening the output file ({}): {}", OUTPUT_PATH, err))
  })
}

fn read_solutions() -> Result<String, String> {
  fs::read_to_string(SOLUTIONS_PATH).or_else(|err| {
    Err(format!("Error opening the solutions file ({}): {}", SOLUTIONS_PATH, err))
  })
}

fn solve_problems(results: &mut HashMap<u32, String>,
                  solutions: HashMap<u32, String>) -> Result<(), String> {
  for problem_number in 1..=PROBLEMS {
    let result = match results.get(&problem_number) {
      Some(value) => value.clone(),
      None => solve(problem_number)?,
    };
    if let Some(solution) = solutions.get(&problem_number) {
      if result != solution.to_string() {
        return Err(
          format!(
            "Problem {} returned an incorrect value of {} (expected {})",
            problem_number,
            result,
            solution,
          ),
        );
      }
    }
    println!("Problem {} was solved with a value of {}", problem_number, result);
    results.insert(problem_number, result);
  }

  Ok(())
}

fn save_results(results: &HashMap<u32, String>) -> Result<(), String> {
  let mut output = fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(OUTPUT_PATH).or_else(|err| {
      Err(format!("Could not open the output file for writing: {}", err))
    })?;

  for (problem_number, result) in results.iter() {
    output
      .write_all(format!("Problem {}: {}\n", problem_number, result)
      .as_bytes())
      .or_else(|err| {
        Err(format!("Could not write to the output file ({}): {}", OUTPUT_PATH, err))
      })?;
  }

  Ok(())
}
