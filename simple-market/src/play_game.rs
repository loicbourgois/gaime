use crate::game::get_actions_str;
use crate::print_state;
use crate::score;
use crate::Market;
use crate::NeuralNet;
use crate::Player;

pub fn play_game(
    turns: usize,
    neural_nets: &mut [&mut NeuralNet],
    actions: &[fn(&mut Market, &mut Player) -> bool],
    verbose: bool,
) {
    let mut m = Market::new();
    let mut ps = vec![];
    for _ in 0..neural_nets.len() {
        ps.push(Player::new());
    }
    if verbose {
        print_state(&m, &ps);
    }
    for step in 0..turns {
        for (i, neural_net) in neural_nets.iter_mut().enumerate() {
            let mut ins: Vec<f32> = vec![i as f32, m.wood as f32, m.rice as f32, m.land as f32];
            for p in &ps {
                ins.push(p.rice as f32);
                ins.push(p.wood as f32);
                ins.push(p.land as f32);
                ins.push(p.farm as f32);
                ins.push(p.forest as f32);
                ins.push(p.gold as f32);
                ins.push(p.debt as f32);
            }
            let decisions = neural_net.get_decisions(&ins);
            let mut decision = None;
            for decision_ in &decisions {
                let r = actions[decision_.0](&mut m, &mut ps[i]);
                if r {
                    decision = Some(decision_);
                    break;
                }
            }
            assert!(!decision.is_none());
            if verbose {
                println!("p{i} -> {decision:?}\n{decisions:?}",);
                print_state(&m, &ps);
            }
        }
    }
    let sum_score: f32 = ps.iter().map(score).sum::<f32>().max(1.0);
    for i in 0..neural_nets.len() {
        neural_nets[i].score += score(&ps[i]);
    }
}

pub fn get_inputs(m: &Market, ps: &[Player], player_id: usize) -> Vec<f32> {
    let mut ins: Vec<f32> = vec![
        player_id as f32,
        m.wood as f32,
        m.rice as f32,
        m.land as f32,
    ];
    for p in ps {
        ins.push(p.rice as f32);
        ins.push(p.wood as f32);
        ins.push(p.land as f32);
        ins.push(p.farm as f32);
        ins.push(p.forest as f32);
        ins.push(p.gold as f32);
        ins.push(p.debt as f32);
    }
    ins
}

pub trait DecisionMaker {
    fn get_decisions(&mut self, ins: &[f32]) -> Vec<(usize, f32)>;
    fn add_score(&mut self, score: f32);
}

pub fn play_game_2(
    turns: usize,
    decision_makers: &mut [&mut dyn DecisionMaker],
    actions: &[fn(&mut Market, &mut Player) -> bool],
    verbose: bool,
) {
    let mut m = Market::new();
    let mut ps = vec![];
    for _ in 0..decision_makers.len() {
        ps.push(Player::new());
    }
    if verbose {
        print_state(&m, &ps);
    }
    for step in 0..turns {
        for (player_id, decision_maker) in decision_makers.iter_mut().enumerate() {
            let ins = get_inputs(&m, &ps, player_id);
            let (decision, decisions) = loop {
                let decisions = decision_maker.get_decisions(&ins);
                let mut decision_opt = None;
                for decision in decisions.iter() {
                    let r = actions[decision.0](&mut m, &mut ps[player_id]);
                    if r {
                        decision_opt = Some(decision);
                        break;
                    }
                }
                match decision_opt {
                    Some(d) => {
                        break (d.clone(), decisions);
                    }
                    None => {
                        println!("invalid decision: {:?}", decisions)
                    }
                }
            };
            if verbose {
                let aa = get_actions_str()[decision.0];
                println!("p{player_id} -> {decision:?} -> {aa}\n{decisions:?}");
                print_state(&m, &ps);
            }
        }
    }
    let sum_score: f32 = ps.iter().map(score).sum::<f32>().max(1.0);
    for i in 0..decision_makers.len() {
        decision_makers[i].add_score(score(&ps[i]));
    }
}
