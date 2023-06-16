extern crate test;
use crate::compute;
use crate::dna_to_neural_net;
use crate::file_to_dna;
use crate::get_actions;
use crate::play_game;
use crate::reduce_nn::reduce_nn;
use crate::Market;
use crate::NeuralNet;
use crate::NeuroConst;
use crate::Neuron;
use crate::Player;
use test::Bencher;

#[bench]
fn bench_1(b: &mut Bencher) {
    let dna = file_to_dna("./latest/best.dna");
    let neural_net = dna_to_neural_net(&dna).unwrap();
    let rng = &mut rand::thread_rng();
    let l = neural_net.neurons.len();
    let m = Market::new();
    let ps = vec![Player::new()];
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
    b.iter(|| {
        for i in 0..l {
            compute(&neural_net.neurons[i], &neural_net.values_b, rng, &ins);
        }
    });
}

#[bench]
fn bench_2(b: &mut Bencher) {
    let turns = 2000;
    let dna = file_to_dna("./latest/best.dna");
    let rng = &mut rand::thread_rng();
    let actions = get_actions();
    b.iter(|| {
        let mut neural_net = dna_to_neural_net(&dna).unwrap();
        play_game(turns, &mut neural_net, rng, &actions, false);
    });
}

#[bench]
fn bench_3(b: &mut Bencher) {
    let turns = 2000;
    let dna = file_to_dna("./latest/best.dna");
    let rng = &mut rand::thread_rng();
    let actions = get_actions();
    b.iter(|| {
        let mut neural_net = dna_to_neural_net(&dna).unwrap();
        reduce_nn(&mut neural_net);
        play_game(turns, &mut neural_net, rng, &actions, false);
    });
}

#[bench]
fn bench_4(b: &mut Bencher) {
    let rng = &mut rand::thread_rng();
    let m = Market::new();
    let ps = vec![Player::new()];
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
    let neural_net = NeuralNet {
        neurons: vec![Neuron::NeuroConst(NeuroConst { v: 2.0 })],
        values_a: vec![],
        values_b: vec![],
        dna: vec![],
        score: 0,
    };
    b.iter(|| {
        compute(&neural_net.neurons[0], &neural_net.values_b, rng, &ins);
    });
}
