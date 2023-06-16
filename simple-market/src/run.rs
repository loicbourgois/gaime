use crate::dna_to_neural_net;
use crate::get_actions;
use crate::play_game;
use crate::reduce_nn;
use crate::Config;
use crate::Dna;
use crate::NeuralNet;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::time::Instant;

pub struct Clan {
    nns: Vec<NeuralNet>,
    clan_population: usize,
}

impl Clan {
    pub fn setup(&mut self, c: &Config) {
        let rng = &mut rand::thread_rng();
        {
            let base_dnas: Vec<_> = self.nns.iter().map(|x| x.dna.clone()).collect();
            self.nns.clear();
            for dna in &base_dnas {
                self.nns.push(dna_to_neural_net(dna, c));
            }
        }
        loop {
            let new_dna_to_add = if rng.gen::<f32>() < c.new_random || self.nns.is_empty() {
                let mut new_dna = vec![];
                for _ in 0..c.dna_size {
                    new_dna.push(rng.gen_range(0..255));
                }
                new_dna
            } else {
                let ii = rng.gen_range(0..self.nns.len());
                let mut new_dna = self.nns[ii].dna.clone();
                for _ in 0..rng.gen_range(0..c.mutations) {
                    let mutation_kind = rng.gen_range(0..3);
                    match mutation_kind {
                        0 => {
                            let i = rng.gen_range(0..new_dna.len());
                            new_dna[i] = rng.gen_range(0..255);
                        }
                        1 => {
                            let i = rng.gen_range(0..new_dna.len() / c.chunk);
                            for _ in 0..c.chunk {
                                new_dna.remove(i * c.chunk);
                            }
                            for _ in 0..c.chunk {
                                new_dna.push(rng.gen_range(0..255));
                            }
                        }
                        2 => {
                            let i = rng.gen_range(0..new_dna.len() / c.chunk);
                            for ic in 0..c.chunk {
                                new_dna.insert(i * c.chunk + ic, rng.gen_range(0..255));
                            }
                            new_dna.truncate(c.dna_size);
                        }
                        _ => {
                            panic!("invalid mutation kind");
                        }
                    }
                }
                new_dna
            };
            self.nns.push(dna_to_neural_net(&new_dna_to_add, c));
            if self.nns.len() >= self.clan_population {
                break;
            }
        }
        self.nns.shuffle(rng);
        for nn in &mut self.nns {
            if c.reduce {
                reduce_nn(nn);
            }
        }
    }
}

pub fn run(c: &Config, data: &mut Vec<Vec<f64>>, lines: &mut Vec<String>) -> (Vec<Dna>, f32) {
    let mut clans = vec![];
    for _ in 0..c.player_count {
        clans.push(Clan {
            nns: vec![],
            clan_population: c.population / c.player_count,
        });
    }
    let mut top_score_ever = 0.0;
    for generation in 0..=c.generations {
        run_generation(generation, &mut clans, c, &mut top_score_ever, data, lines);
    }
    let mut top_dnas = vec![];
    for clan in clans {
        top_dnas.push(clan.nns[0].dna.clone());
    }
    (top_dnas, top_score_ever)
}

pub fn run_generation(
    generation: usize,
    clans: &mut [Clan],
    c: &Config,
    top_score_ever: &mut f32,
    data: &mut Vec<Vec<f64>>,
    lines: &mut Vec<String>,
) {
    let rng = &mut rand::thread_rng();
    let actions = get_actions();
    let start = Instant::now();
    let setup_start = Instant::now();
    for clan in clans.iter_mut() {
        clan.setup(c);
    }
    let setup_elapsed = setup_start.elapsed();
    let game_start = Instant::now();
    for i in 0..(c.population / c.player_count) {
        unsafe {
            let mut nns = vec![];
            for clan in &mut *clans {
                let pointer = std::ptr::addr_of_mut!(clan.nns[i]);
                let nn = &mut (*pointer);
                nns.push(nn);
            }
            if c.shuffle {
                nns.shuffle(rng);
            }
            play_game(c.turns, &mut nns, &actions, false);
            let mut replay = false;
            for nn in &mut nns {
                data.push(vec![
                    generation as f64,
                    f64::from(nn.score),
                    (nn.neurons.len() as f64),
                    (nn.dna.len() as f64),
                ]);
                if nn.score > *top_score_ever {
                    replay = true;
                    *top_score_ever = nn.score;
                }
            }
            if replay {
                let mut oi = vec![];
                let mut oi2 = vec![];
                for nn in nns {
                    oi.push(dna_to_neural_net(&nn.dna, c));
                }
                for x in &mut oi {
                    oi2.push(x);
                }
                play_game(c.turns, &mut oi2, &actions, true);
            }
        }
    }
    for clan in clans.iter_mut() {
        clan.nns
            .sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        clan.nns.reverse();
        clan.nns
            .truncate((clan.clan_population as f32 * c.keep_top) as usize);
    }
    let game_elapsed = game_start.elapsed();
    let elapsed = start.elapsed();
    let mut line = Vec::new();
    line.push(format!("{}", generation));
    for clan in clans.iter() {
        let mean_score = clan.nns.iter().map(|x| x.score).sum::<f32>() / (clan.nns.len() as f32);
        line.push(format!("{mean_score}"));
    }
    lines.push(line.join(","));
    if generation % (c.generations / 100).max(1) == 0 {
        println!("# {generation}/{}\t{top_score_ever}\t{setup_elapsed:.2?} {game_elapsed:.2?} {elapsed:.2?}", c.generations);
        for clan in clans {
            let pop = clan.nns.len();
            let top_score = clan.nns[0].score;
            let mean_score =
                clan.nns.iter().map(|x| x.score).sum::<f32>() / (clan.nns.len() as f32);
            println!("{pop}\t{top_score}\t{mean_score:.2?}");
        }
    }
}
