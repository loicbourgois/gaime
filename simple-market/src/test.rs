use crate::compute;
use crate::dna_to_neural_net;
use crate::get_actions;
use crate::get_base_config;
use crate::print_state;
use crate::reduce_nn::reduce_nn;
use crate::score;
use crate::write_mermaid;
use crate::Market;
use crate::NeuralNet;
use crate::Player;
use rand::Rng;
#[test]
fn test_1() {
    let c = get_base_config(1);
    let rng = &mut rand::thread_rng();
    let turns = 1;
    let actions = get_actions();
    let dna_size = 10;
    for _ in 0..1000 {
        let mut dna = vec![];
        for _ in 0..dna_size {
            dna.push(rng.gen_range(0..255));
        }
        let mut neural_net_1 = dna_to_neural_net(&dna, &c);
        let mut neural_net_2 = dna_to_neural_net(&dna, &c);
        reduce_nn(&mut neural_net_2);
        play_game_test(turns, &mut neural_net_1, &mut neural_net_2, &actions, false);
    }
}

fn play_game_test(
    turns: usize,
    neural_net_1: &mut NeuralNet,
    neural_net_2: &mut NeuralNet,
    actions: &[fn(&mut Market, &mut Player) -> bool],
    verbose: bool,
) -> f32 {
    let mut m = Market::new();
    let mut ps = vec![Player::new()];
    if verbose {
        print_state(&m, &ps);
    }
    for step in 0..turns {
        let ins: Vec<f32> = vec![
            0.0,
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
        if neural_net_1.neurons.is_empty() {
            break;
        }
        let decisions_1 = neural_net_1.get_decisions(&ins);
        let decisions_2 = neural_net_2.get_decisions(&ins);
        if decisions_1 != decisions_2 || step == turns - 1 {
            println!("{:?}", decisions_1);
            println!("{:?}", decisions_2);
            write_mermaid("test/1", &neural_net_1, 1);
            write_mermaid("test/2", &neural_net_2, 1);
        }
        assert!(decisions_1 == decisions_2);
        // let decision = ((decision_f * actions.len() as f32) as usize)
        //     .max(0)
        //     .min(actions.len() - 1);
        // actions[decision](&mut m, &mut ps[0]);
        // if verbose {
        //     println!("{decision}");
        //     print_state(&m, &ps);
        // }
    }
    score(&ps[0])
}
