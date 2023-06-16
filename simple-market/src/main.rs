#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
mod bench;
mod charts;
mod compute;
mod game;
mod nn;
mod play_game;
mod reduce_nn;
mod run;
#[cfg(feature = "test")]
mod test;
mod write_mermaid;
use crate::charts::write_chart_1;
use crate::charts::write_chart_2;
use crate::compute::compute;
use crate::game::get_actions;
use crate::game::get_actions_str;
use crate::game::print_state;
use crate::game::score;
use crate::game::Market;
use crate::game::Player;
use crate::nn::dna_to_file;
use crate::nn::dna_to_neural_net;
use crate::nn::NeuralNet;
use crate::nn::Neuron;
use crate::play_game::play_game;
use crate::play_game::play_game_2;
use crate::play_game::DecisionMaker;
use crate::reduce_nn::reduce_nn;
use crate::run::run;
use crate::write_mermaid::write_mermaid;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::str::FromStr;

type Dna = Vec<u8>;

pub struct Config {
    population: usize,
    dna_size: usize,
    generations: usize,
    keep_top: f32,
    mutations: usize,
    turns: usize,
    new_random: f32,
    reduce: bool,
    chunk: usize,
    thinking_turns: usize,
    ins_len: usize,
    player_count: usize,
    outs_len: usize,
    shuffle: bool,
}

fn get_base_config(player_count: usize) -> Config {
    Config {
        generations: 1000,
        population: 1000,
        dna_size: 500,
        turns: 20,
        keep_top: 0.5,
        new_random: 0.01,
        mutations: 10,
        chunk: 3,
        reduce: false,
        thinking_turns: 2,
        ins_len: 4 + player_count * 7,
        outs_len: 10,
        player_count,
        shuffle: true,
    }
}

#[must_use]
pub fn file_to_dna(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

pub struct Human {
    score: f32,
    rng: ThreadRng,
}

impl DecisionMaker for Human {
    fn get_decisions(&mut self, ins: &[f32]) -> Vec<(usize, f32)> {
        for (i, action) in get_actions_str().iter().enumerate() {
            println!("[{i}] {action}");
        }
        print!("action: ");
        io::stdout().flush();
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        return vec![(usize::from_str(&buffer.replace("\n", "")).unwrap(), 0.0)];
    }

    fn add_score(&mut self, score: f32) {
        self.score += score;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player_count = 2;
    let c = get_base_config(player_count);
    let actions = get_actions();
    // let mut lines = vec![];
    // let mut data = vec![];
    // let (top_dnas, best_score) = run(&c, &mut data, &mut lines);
    // fs::write(format!("./latest/mean.csv"), lines.join("\n")).unwrap();
    // dna_to_file("./latest/last_best.dna", &top_dnas[0]);
    // let mut neural_nets = vec![];
    // for top_dna in &top_dnas {
    //     neural_nets.push(dna_to_neural_net(top_dna, &c));
    // }
    // let mut nns = vec![];
    // for x in &mut neural_nets {
    //     nns.push(x);
    // }
    // play_game(c.turns, &mut nns, &actions, true);
    // for (i, top_dna) in top_dnas.iter().enumerate() {
    //     dna_to_file(&format!("./latest/{i}.dna"), top_dna);
    //     write_mermaid(
    //         &format!("./latest/{i}"),
    //         &dna_to_neural_net(top_dna, &c),
    //         player_count,
    //     );
    // }
    // write_chart_1(&c, &data, best_score).unwrap();
    // write_chart_2(&data, best_score);
    let dna_0 = file_to_dna("./latest/0.dna");
    let dna_1 = file_to_dna("./latest/1.dna");
    let mut nn = dna_to_neural_net(&dna_0, &c);
    let nn_2 = dna_to_neural_net(&dna_1, &c);
    let mut human = Human {
        score: 0.0,
        rng: rand::thread_rng(),
    };
    // let mut decision_makers = vec![nn, human];
    let mut decision_makers_2: Vec<&mut dyn DecisionMaker> = vec![];
    decision_makers_2.push(&mut nn);
    decision_makers_2.push(&mut human);
    // for x in &mut decision_makers {
    //     decision_makers_2.push(x);
    // }
    play_game_2(c.turns, &mut decision_makers_2, &actions, true);
    Ok(())
}
// todo: better output structure
// we don't want any turn passing
// each turn the player must take a valid action
// so we need to go through each one of them
//
