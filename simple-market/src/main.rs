#![cfg_attr(feature = "bench", feature(test))]
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs;

//
mod bench;

#[derive(Clone, Debug, Copy)]
pub struct Market {
    wood: i32,
    rice: i32,
    land: i32,
}

impl Market {
    fn new() -> Market {
        Market {
            wood: 1,
            rice: 1,
            land: 1,
        }
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Player {
    rice: i32,
    wood: i32,
    land: i32,
    farm: i32,
    forest: i32,
    gold: i32,
    debt: i32,
}

impl Player {
    fn new() -> Player {
        Player::default()
    }
}

// pub struct NeuralNet {
//     neurons: Vec<String>,
// }

pub fn buy_wood(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.wood {
        p.gold -= m.wood;
        p.wood += 1;
        m.wood += 1;
        return true;
    }
    false
}

pub fn sell_wood(m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 {
        p.wood -= 1;
        m.wood = 1.max(m.wood - 1);
        p.gold += m.wood;
        return true;
    }
    false
}

pub fn buy_rice(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.rice {
        p.gold -= m.rice;
        p.rice += 1;
        m.rice += 1;
        return true;
    }
    false
}

pub fn buy_land(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.land {
        p.gold -= m.land;
        p.land += 1;
        m.land += 1;
        return true;
    }
    false
}

pub fn sell_rice(m: &mut Market, p: &mut Player) -> bool {
    if p.rice >= 1 {
        p.rice -= 1;
        m.rice = 1.max(m.rice - 1);
        p.gold += m.rice;
        return true;
    }
    false
}

pub fn build_farm(_m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 && p.land >= 1 {
        p.farm += 1;
        p.wood -= 1;
        p.land -= 1;
        return true;
    }
    false
}

pub fn build_forest(_m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 && p.land >= 1 {
        p.forest += 1;
        p.wood -= 1;
        p.land -= 1;
        return true;
    }
    false
}

pub fn recolt_rice(_m: &mut Market, p: &mut Player) -> bool {
    if p.farm >= 1 {
        p.farm -= 1;
        p.rice += 10;
        return true;
    }
    false
}

pub fn recolt_wood(_m: &mut Market, p: &mut Player) -> bool {
    if p.forest >= 1 {
        p.forest -= 1;
        p.wood += 10;
        return true;
    }
    false
}

pub fn take_loan(_m: &mut Market, p: &mut Player) -> bool {
    p.gold += 10;
    p.debt += 10;
    true
}

#[must_use]
pub fn score(p: &Player) -> i32 {
    p.gold - p.debt + p.rice * 3 + p.wood * 3
}

pub fn print_state(m: &Market, ps: &[Player]) {
    println!(" --------------------------------------------------------------------------------");
    println!("|        |  rice  |  wood  |  land  |  farm  | forest |  gold  |  debt  |  score |");
    println!(
        "| market | {:06} | {:06} | {:06} |        |        |        |        |        |",
        m.rice, m.wood, m.land
    );
    for (i, p) in ps.iter().enumerate() {
        println!(
            "|     p{} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} |",
            i + 1,
            p.rice,
            p.wood,
            p.land,
            p.farm,
            p.forest,
            p.gold,
            p.debt,
            score(p),
        );
    }
    println!(" --------------------------------------------------------------------------------");
}

pub enum Neuron {
    NeuroRandom(NeuroRandom),
    NeuroDecision(NeuroDecision),
    NeuroConst(NeuroConst),
    NeuroHardTanh(NeuroHardTanh),
    NeuroInput(NeuroInput),
}

pub struct NeuroHardTanh {
    vids: Vec<usize>,
}

pub struct NeuroRandom {}

pub struct NeuroConst {
    v: f32,
}

pub struct NeuroDecision {
    vids: Vec<usize>,
}

pub struct NeuroInput {
    iids: Vec<usize>,
}

type Neurons = Vec<Neuron>;

pub fn compute(n: &Neuron, neuro_outs: &[f32], rng: &mut ThreadRng, ins: &[f32]) -> f32 {
    return match n {
        Neuron::NeuroRandom(_) => rng.gen(),
        Neuron::NeuroConst(n) => n.v,
        Neuron::NeuroDecision(n) => match n
            .vids
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                neuro_outs[(**a).min(neuro_outs.len() - 1)]
                    .total_cmp(&neuro_outs[(**b).min(neuro_outs.len() - 1)])
            })
            .map(|(index, _)| index)
        {
            Some(x) => (x as f32) / (n.vids.len() as f32),
            None => -1.0,
        },
        Neuron::NeuroHardTanh(n) => n
            .vids
            .iter()
            .map(|x| neuro_outs[(*x).min(neuro_outs.len() - 1)])
            .sum::<f32>()
            .max(-1.0)
            .min(1.0),
        Neuron::NeuroInput(n) => n
            .iids
            .iter()
            .map(|x| ins[(*x).min(ins.len() - 1)])
            .sum::<f32>(),
    };
}

pub struct NeuralNet {
    pub neurons: Neurons,
    pub values_a: Vec<f32>,
    pub values_b: Vec<f32>,
    pub break_point: usize,
    pub dna: Vec<u8>,
    pub score: i32,
}

pub fn dna_to_neural_net(dna: &[u8]) -> Result<NeuralNet, &'static str> {
    let mut neurons: Neurons = vec![];
    let mut values_a: Vec<f32> = vec![];
    let mut values_b: Vec<f32> = vec![];
    let break_point = 0;
    for i in 0..dna.len() / 2 {
        let action = dna[i * 2];
        let value = dna[i * 2 + 1];
        match action % 2 {
            0 => {
                match value % 3 {
                    // 0 => {
                    //     neurons.push(Neuron::NeuroRandom(NeuroRandom {}));
                    // }
                    0 => {
                        neurons.push(Neuron::NeuroDecision(NeuroDecision { vids: vec![] }));
                    }
                    1 => {
                        neurons.push(Neuron::NeuroConst(NeuroConst { v: 0.0 }));
                    }
                    2 => {
                        neurons.push(Neuron::NeuroHardTanh(NeuroHardTanh { vids: vec![] }));
                    }
                    _ => {
                        panic!("aa")
                    }
                }
                values_a.push(0.0);
                values_b.push(0.0);
            }
            1 => {
                let l = neurons.len();
                if l > 0 {
                    let nn = &mut neurons[l - 1];
                    match nn {
                        Neuron::NeuroRandom(_) => {}
                        Neuron::NeuroInput(n) => n.iids.push(usize::from(value)),
                        Neuron::NeuroConst(n) => n.v = f32::from(value) / 255.0,
                        Neuron::NeuroDecision(n) => n.vids.push(usize::from(value)),
                        Neuron::NeuroHardTanh(n) => n.vids.push(usize::from(value)),
                    }
                }
            }
            _ => {
                panic!("aa")
            }
        }
    }
    let rng = &mut rand::thread_rng();
    let ins: Vec<f32> = vec![
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
    ];
    for n in &neurons {
        compute(&n, &values_b, rng, &ins);
    }
    let neural_net = NeuralNet {
        neurons,
        values_a,
        values_b,
        break_point,
        dna: dna.to_vec(),
        score: i32::MIN,
    };
    Ok(neural_net)
}

fn play_game(
    turns: usize,
    neural_net: &mut NeuralNet,
    rng: &mut ThreadRng,
    actions: &[fn(&mut Market, &mut Player) -> bool],
    verbose: bool,
) -> i32 {
    let mut m = Market::new();
    let mut ps = vec![Player::new()];
    if verbose {
        print_state(&m, &ps);
    }
    for step in 0..turns {
        let ins: Vec<f32> = vec![
            m.wood as f32,
            m.rice as f32,
            m.land as f32,
            ps[0].rice as f32,
            ps[0].wood as f32,
            ps[0].land as f32,
            ps[0].farm as f32,
            ps[0].forest as f32,
            ps[0].gold as f32,
            ps[0].debt as f32,
        ];
        if neural_net.neurons.is_empty() {
            break;
        }
        let l = neural_net.neurons.len();
        let decision_f = if step % 2 == 0 {
            for i in 0..l {
                neural_net.values_a[i] =
                    compute(&neural_net.neurons[i], &neural_net.values_b, rng, &ins);
            }
            neural_net.values_a[0]
        } else {
            for i in 0..l {
                neural_net.values_b[i] =
                    compute(&neural_net.neurons[i], &neural_net.values_a, rng, &ins);
            }
            neural_net.values_b[0]
        }
        .max(0.0)
        .min(1.0);
        let decision = ((decision_f * actions.len() as f32) as usize)
            .max(0)
            .min(actions.len() - 1);
        actions[decision](&mut m, &mut ps[0]);
        if verbose {
            println!("{decision}");
            print_state(&m, &ps);
        }
    }
    return score(&ps[0]);
}

pub fn get_actions() -> Vec<fn(&mut Market, &mut Player) -> bool> {
    vec![
        take_loan,
        buy_wood,
        sell_wood,
        buy_rice,
        sell_rice,
        build_farm,
        build_forest,
        recolt_rice,
        recolt_wood,
        buy_land,
    ]
}

fn main() {
    let mut c_error = 0;
    let population = 1000;
    let tries = 10_000_000;
    let dna_size = 300;
    let generations = 2000;
    let keep_top = 0.1;
    let mutations = 10;
    let turns = 20;
    let rng = &mut rand::thread_rng();
    let actions = get_actions();
    let mut neural_nets = vec![];
    for _ in 0..population {
        let mut dna: Vec<u8> = vec![];
        for _ in 0..dna_size {
            dna.push(rng.gen_range(0..255));
        }
        let neural_net = dna_to_neural_net(&dna).unwrap();
        neural_nets.push(neural_net);
    }
    println!("errors:      {c_error}");
    println!("neural_nets: {}", neural_nets.len());
    let mut best_score = -100_000;
    for generation in 0..=generations {
        let base_dnas: Vec<_> = neural_nets.iter().map(|x| x.dna.clone()).collect();
        neural_nets.clear();
        for dna in base_dnas {
            neural_nets.push(dna_to_neural_net(&dna).unwrap());
        }
        for _ in 0..tries {
            let i = rng.gen_range(0..neural_nets.len());
            let mut new_dna = neural_nets[i].dna.clone();
            for _ in 0..mutations {
                let i = rng.gen_range(0..new_dna.len());
                new_dna[i] = rng.gen_range(0..255);
            }
            let neural_net = dna_to_neural_net(&new_dna);
            match neural_net {
                Ok(neural_net) => {
                    neural_nets.push(neural_net);
                }
                Err(_) => c_error += 1,
            }
            if neural_nets.len() >= population {
                break;
            }
        }
        for neural_net in &mut neural_nets {
            neural_net.score = play_game(turns, neural_net, rng, &actions, false);
        }
        neural_nets.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        neural_nets.reverse();
        neural_nets.truncate(((neural_nets.len() as f32) * keep_top) as usize);
        if neural_nets[0].score > best_score {
            best_score = neural_nets[0].score;
            let aa = neural_nets[0].neurons.len();
            println!("{generation}, {best_score}, {aa}");
            dna_to_file("./src/best.dna", &neural_nets[0].dna);
        } else if generation % (generations / 100).max(1) == 0 {
            let aa = neural_nets[0].neurons.len();
            let bs = neural_nets[0].score;
            println!("{generation}, {bs}, {aa}");
        }
    }
    // let champion = dna_to_neural_net(&neural_nets[0].dna).unwrap();
}

pub fn dna_to_file(path: &str, data: &[u8]) {
    return fs::write(path, data).unwrap();
}

pub fn file_to_dna(path: &str) -> Vec<u8> {
    return fs::read(path).unwrap();
}
