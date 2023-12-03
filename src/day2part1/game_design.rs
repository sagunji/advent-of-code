use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct GameData {
  game: u32,
  max_red: u32,
  max_green: u32,
  max_blue: u32,
}

fn parse_game_number(input: &str) -> u32 {
  let parts: Vec<&str> = input.split(":").collect();
  if let Some(first_part) = parts.first() {
      return first_part.trim().split_whitespace().last().unwrap_or("0").parse().unwrap_or(0);
  }
  0
}

fn get_game_entries(input: &str) -> Vec<String> {
  let parts: Vec<&str> = input.split(": ").collect();
  let game_entries = parts.get(1).map_or(vec![], |s| {
      s.split("; ").map(|entry| entry.to_string()).collect()
  });

  game_entries
}

fn parse_game_data(input: &str) -> Vec<GameData> {
  let mut games = Vec::new();
  let mut m_red: u32 = 0;
  let mut m_green: u32 = 0;
  let mut m_blue: u32 = 0;

  let game_number = parse_game_number(input);

  let game_entries = get_game_entries(input);

  for game in game_entries {
    let game_sets: Vec<&str> = game.trim().split(",").collect();
    for game_set in game_sets {
      let per_sets: Vec<&str> = game_set.split_whitespace().collect();
      if let Ok(j_num) = per_sets[0].parse::<u32>() {
        if per_sets[1] == "red" && m_red < j_num {
          m_red = j_num;
        } else if per_sets[1] == "blue" && m_blue < j_num {
          m_blue = j_num;
        } else if per_sets[1] == "green" && m_green < j_num {
          m_green = j_num;
        }
      }
    }
  }

  games.push(GameData {
    game: game_number,
    max_red: m_red,
    max_green: m_green,
    max_blue: m_blue,
  });

  games
}

fn compare_game_data(game_data: &GameData) -> bool {
  const COM_RED: u32 = 12;
  const COM_GREEN: u32 = 13;
  const COM_BLUE: u32 = 14;

  COM_RED >= game_data.max_red && COM_GREEN >= game_data.max_green && COM_BLUE >= game_data.max_blue
}

fn main() -> io::Result<()> {
  let path = Path::new("src/game.txt");
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  let mut sum: u32 = 0;

  let mut results = Vec::new();

  for line in reader.lines() {
    let line = line?;

    let parsed_data = parse_game_data(&line);
    results.extend(parsed_data); // Use extend instead of push
  }

  for game in results {
    
    if compare_game_data(&game) {
      sum = sum + game.game;
    }
  }
  
  println!("Game sum: {}", sum);
  Ok(())
}
