#![cfg_attr(feature = "bench", feature(test))]
mod action;
#[cfg(feature = "bench")]
mod bench;
mod charts;
mod compute;
mod game;
mod misc;
mod nn;
mod play_game;
mod reduce_nn;
mod run;
#[cfg(feature = "test")]
mod test;
mod write_mermaid;
use crate::action::get_actions;
use crate::action::get_actions_str;
use crate::action::Action;
use crate::charts::write_chart_1;
use crate::charts::write_chart_2;
use crate::compute::compute;
use crate::game::print_state;
use crate::game::score;
use crate::game::Market;
use crate::game::Player;
use crate::misc::file_to_dna;
use crate::misc::Human;
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
use std::fs;

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

fn get_base_config(player_count: usize, actions: &Vec<Box<dyn Action>>) -> Config {
    Config {
        generations: 2000,
        population: 2000,
        dna_size: 500,
        turns: 20,
        keep_top: 0.25,
        new_random: 0.1,
        mutations: 10,
        chunk: 3,
        reduce: false,
        thinking_turns: 2,
        ins_len: 5 + player_count * 7,
        outs_len: actions.len(),
        player_count,
        shuffle: true,
    }
}

#[allow(dead_code)]
enum Mode {
    Play,
    Train,
}

fn main() {
    // let mode = Mode::Play;
    let mode = Mode::Train;
    let player_count = 3;
    let actions = get_actions();
    let c = get_base_config(player_count, &actions);
    match mode {
        Mode::Play => {
            let mut nns = vec![];
            let mut decision_makers_2: Vec<&mut dyn DecisionMaker> = vec![];
            for i in 0..(player_count - 1) {
                let dna = file_to_dna(&format!("./latest/{i}.dna"));
                nns.push(dna_to_neural_net(&dna, &c));
            }
            for x in &mut nns {
                decision_makers_2.push(x);
            }
            let mut human = Human { score: 0.0 };
            decision_makers_2.push(&mut human);
            play_game_2(c.turns, &mut decision_makers_2, &actions, true);
        }
        Mode::Train => {
            let mut lines = vec![];
            let mut data = vec![];
            let (top_dnas, best_score) = run(&c, &mut data, &mut lines);
            fs::write("./latest/mean.csv", lines.join("\n")).unwrap();
            dna_to_file("./latest/last_best.dna", &top_dnas[0]);
            let mut neural_nets = vec![];
            for top_dna in &top_dnas {
                neural_nets.push(dna_to_neural_net(top_dna, &c));
            }
            let mut nns = vec![];
            for x in &mut neural_nets {
                nns.push(x);
            }
            play_game(c.turns, &mut nns, &actions, true);
            for (i, top_dna) in top_dnas.iter().enumerate() {
                dna_to_file(&format!("./latest/{i}.dna"), top_dna);
                write_mermaid(
                    &format!("./latest/{i}"),
                    &dna_to_neural_net(top_dna, &c),
                    player_count,
                );
            }
            write_chart_1(&c, &data, best_score).unwrap();
            write_chart_2(&data, best_score).unwrap();
        }
    }
}
