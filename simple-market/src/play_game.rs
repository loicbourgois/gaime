use crate::action::get_actions_str;
use crate::action::Action;
use crate::print_state;
use crate::score;
use crate::Market;
use crate::NeuralNet;
use crate::Player;

pub fn play_game(
    turns: usize,
    neural_nets: &mut [&mut NeuralNet],
    actions: &[Box<dyn Action>],
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
    for turn_id in 0..turns {
        for (player_id, neural_net) in neural_nets.iter_mut().enumerate() {
            let ins = get_inputs(&m, &ps, player_id, turn_id);
            let decisions = neural_net.get_decisions(&ins);
            let mut decision = None;
            for decision_ in &decisions {
                let r = actions[decision_.0].run(&mut m, &mut ps[player_id]);
                if r {
                    decision = Some(decision_);
                    break;
                }
            }
            assert!(decision.is_some());
            if verbose {
                println!("p{player_id} -> {decision:?}\n{decisions:?}",);
                print_state(&m, &ps);
            }
        }
    }
    for i in 0..neural_nets.len() {
        neural_nets[i].score += score(&ps[i]);
    }
}

pub fn get_inputs(m: &Market, ps: &[Player], player_id: usize, turn_id: usize) -> Vec<f32> {
    let mut ins: Vec<f32> = vec![
        turn_id as f32,
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
    actions: &[Box<dyn Action>],
    verbose: bool,
) {
    let mut m = Market::new();
    let mut ps = vec![];
    for _ in 0..decision_makers.len() {
        ps.push(Player::new());
    }
    for turn_id in 0..turns {
        let dl = decision_makers.len();
        for (player_id, decision_maker) in decision_makers.iter_mut().enumerate() {
            let ins = get_inputs(&m, &ps, player_id, turn_id);
            let (decision, decisions) = loop {
                if verbose {
                    let str_ = format!(
                        "| Turn {}/{turns}  -  Player {}/{dl} |",
                        turn_id + 1,
                        player_id + 1
                    );
                    let dashes = vec!["-"; str_.len() - 2].join("");
                    println!("\n {dashes} ");
                    println!("{str_}");
                    print_state(&m, &ps);
                }
                let decisions = decision_maker.get_decisions(&ins);
                let mut decision_opt = None;
                for decision_ in &decisions {
                    let r = actions[decision_.0].run(&mut m, &mut ps[player_id]);
                    if r {
                        decision_opt = Some(decision_);
                        break;
                    }
                }
                match decision_opt {
                    Some(d) => {
                        break (*d, decisions);
                    }
                    None => {
                        println!("invalid action");
                        // println!("invalid action: {decisions:?}");
                    }
                }
            };
            if verbose {
                // let aa = get_actions_str()[decision.0];
                // println!("# {decisions:?}\np{player_id} -> {decision:?} -> {aa}");
            }
        }
    }
    if verbose {
        print_state(&m, &ps);
    }
    for i in 0..decision_makers.len() {
        decision_makers[i].add_score(score(&ps[i]));
    }
}
