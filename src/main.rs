use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_calibration_digit(s: &str) -> String {
  let mut first_digit: Option<char> = None;
  let mut last_digit: Option<char> = None;

  for char in s.chars() {
    if char.is_digit(10) {
      if first_digit.is_none() {
        first_digit = Some(char);
      }
      last_digit = Some(char);
    }
  }

  match (first_digit, last_digit) {
    (Some(first), Some(last)) => format!("{}{}", first, last),
    _ => String::from("No digits found in the string"),
  }
}

fn main() -> io::Result<()> {
  let path = Path::new("src/calibration.txt");
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  let mut results = Vec::new();
  let mut sum = 0;

  for line in reader.lines() {
    let line = line?;
    let result = get_calibration_digit(&line);
    results.push(result);
  }

  for result in &results {
    if let Ok(number) = result.parse::<i32>() {
      sum += number;
    }
  }

  println!("Sum of all numbers: {}", sum);

  Ok(())
}
