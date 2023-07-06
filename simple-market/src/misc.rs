use crate::action::get_actions_str;
use crate::play_game::DecisionMaker;
use std::fs;
use std::io;
use std::io::Write;
use std::str::FromStr;

#[must_use]
pub fn file_to_dna(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

pub struct Human {
    pub score: f32,
}

impl DecisionMaker for Human {
    fn get_decisions(&mut self, _ins: &[f32]) -> Vec<(usize, f32)> {
        // for (i, action) in get_actions_str().iter().enumerate() {
        //     println!("[{i}] {action}");
        // }
        print!("action: ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        match usize::from_str(&buffer.replace('\n', "")) {
            Ok(u) => {
                vec![(u, 0.0)]
            }
            Err(_) => {
                vec![]
            }
        }
    }

    fn add_score(&mut self, score: f32) {
        self.score += score;
    }
}
