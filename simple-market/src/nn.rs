use crate::compute;
use crate::play_game::DecisionMaker;
use crate::Config;
use std::fs;
#[derive(Debug)]
pub enum Neuron {
    Decision(NeuroDecision),
    Const(NeuroConst),
    HardTanh(NeuroHardTanh),
    Input(NeuroInput),
    Weight(NeuroWeight),
    Output(NeuroOutput),
}
#[derive(Debug)]
pub struct NeuroHardTanh {
    pub vids: Vec<usize>,
}

#[derive(Debug)]
pub struct NeuroConst {
    pub v: f32,
}

#[derive(Debug)]
pub struct NeuroOutput {
    pub sids: Vec<usize>, // source_ids
}

#[derive(Debug)]
pub struct NeuroWeight {
    pub v: f32,
    pub sids: Vec<usize>, // source_ids
}
#[derive(Debug)]
pub struct NeuroDecision {
    pub vids: Vec<usize>,
}
#[derive(Debug)]
pub struct NeuroInput {
    pub iids: Vec<usize>,
}

type Neurons = Vec<Neuron>;

pub struct NeuralNet {
    pub neurons: Neurons,
    pub values_a: Vec<f32>,
    pub values_b: Vec<f32>,
    pub dna: Vec<u8>,
    pub score: f32,
    pub breakpoint: usize,
    pub thinking_turns: usize,
    pub step: usize,
    pub ins_len: usize,
    pub outs_len: usize,
    pub place: f32,
}

impl DecisionMaker for NeuralNet {
    fn get_decisions(&mut self, ins: &[f32]) -> Vec<(usize, f32)> {
        assert!(self.ins_len == ins.len());
        let l = self.neurons.len();
        for _ in 0..self.thinking_turns {
            if self.step % 2 == 0 {
                for i in 0..l {
                    self.values_a[i] = compute(&self.neurons[i], &self.values_b, ins);
                }
            } else {
                for i in 0..l {
                    self.values_b[i] = compute(&self.neurons[i], &self.values_a, ins);
                }
            }
            self.step += 1;
        }
        let values = if (self.step - 1) % 2 == 0 {
            &self.values_a
        } else {
            &self.values_b
        };
        let decision_values = &values[self.ins_len..(self.ins_len + self.outs_len)];
        let mut decisions: Vec<(usize, f32)> = decision_values
            .iter()
            .enumerate()
            .map(|(index, v)| (index, *v))
            .collect::<Vec<(usize, f32)>>();
        decisions.sort_by(|(_, a), (_, b)| b.total_cmp(a));
        decisions
    }

    fn add_score(&mut self, score: f32) {
        self.score += score;
    }
}

// impl NeuralNet {
//     pub fn reset(&mut self) {
//         self.values_a = vec![0.0; self.values_a.len()];
//         self.values_b = vec![0.0; self.values_b.len()];
//         self.step = 0;
//     }
// }

pub fn dna_to_neural_net(dna: &[u8], c: &Config) -> NeuralNet {
    let mut neurons: Neurons = vec![];
    let mut values_a: Vec<f32> = vec![];
    let mut values_b: Vec<f32> = vec![];
    let mut breakpoint = dna.len();
    let ra = 0.25;
    let rb = 0.725;
    let rc = 0.025;
    let r_all: f32 = ra + rb + rc - 1.0;
    assert!(r_all.abs() < 0.0001);
    let a = (255.0 * ra) as u8;
    let b = (255.0 * (ra + rb)) as u8;
    for i in 0..c.ins_len {
        neurons.push(Neuron::Input(NeuroInput { iids: vec![i] }));
        values_a.push(0.0);
        values_b.push(0.0);
    }
    for _ in 0..c.outs_len {
        neurons.push(Neuron::Output(NeuroOutput { sids: vec![] }));
        values_a.push(0.0);
        values_b.push(0.0);
    }
    for i in 0..dna.len() / c.chunk {
        let action = if i == 0 { 0 } else { dna[i * c.chunk] };
        let v1 = dna[i * c.chunk + 1];
        let v2 = dna[i * c.chunk + 2];
        if action < a {
            match v1 % 4 {
                0 => {
                    neurons.push(Neuron::Decision(NeuroDecision { vids: vec![] }));
                }
                1 => {
                    neurons.push(Neuron::Const(NeuroConst {
                        v: f32::from(v2) / 255.0,
                    }));
                }
                2 => {
                    neurons.push(Neuron::HardTanh(NeuroHardTanh { vids: vec![] }));
                }
                3 => {
                    neurons.push(Neuron::Weight(NeuroWeight {
                        v: f32::from(v2) / 255.0 * 2.0 - 1.0,
                        sids: vec![],
                    }));
                }
                _ => {
                    panic!("aa")
                }
            }
            values_a.push(0.0);
            values_b.push(0.0);
        } else if action < b {
            let l_max = (neurons.len() - 1) as f32;
            let l2_max = (neurons.len() - c.ins_len) as f32;
            let source_id = (f32::from(v1) / 255.0 * l_max) as usize;
            let target_id = (f32::from(v2) / 255.0 * l2_max + c.ins_len as f32) as usize;
            let nn = &mut neurons[target_id];
            match nn {
                Neuron::Const(_) => {}
                Neuron::Decision(n) => n.vids.push(source_id),
                Neuron::HardTanh(n) => n.vids.push(source_id),
                Neuron::Weight(n) => n.sids.push(source_id),
                Neuron::Output(n) => n.sids.push(source_id),
                Neuron::Input(_) => {
                    panic!("aa")
                }
            };
        } else {
            breakpoint = i * c.chunk;
            break;
        }
    }
    neurons.push(Neuron::Decision(NeuroDecision { vids: vec![] }));
    values_a.push(0.0);
    values_b.push(0.0);
    NeuralNet {
        neurons,
        values_a,
        values_b,
        dna: dna.to_vec(),
        breakpoint,
        score: 0.0,
        step: 0,
        thinking_turns: c.thinking_turns,
        ins_len: c.ins_len,
        place: 0.0,
        outs_len: c.outs_len,
    }
}

pub fn dna_to_file(path: &str, data: &[u8]) {
    fs::write(path, data).unwrap();
}
