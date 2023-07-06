use crate::NeuralNet;
use crate::Neuron;
use std::fs;
pub fn write_mermaid(path: &str, neural_net: &NeuralNet, player_count: usize) {
    let mut lines: Vec<String> = vec![];
    lines.push("flowchart LR".to_string());
    lines.push("subgraph input".to_string());
    let mut txts: Vec<String> = vec![
        "turn".to_string(),
        "player".to_string(),
        "m.wood".to_string(),
        "m.rice".to_string(),
        "m.land".to_string(),
    ];
    for i in 0..player_count {
        txts.push(format!("p{i}.rice"));
        txts.push(format!("p{i}.wood"));
        txts.push(format!("p{i}.land"));
        txts.push(format!("p{i}.farm"));
        txts.push(format!("p{i}.forest"));
        txts.push(format!("p{i}.gold"));
        txts.push(format!("p{i}.debt"));
    }
    for (i, txt) in txts.iter().enumerate().take(neural_net.ins_len) {
        // for i in 0..neural_net.ins_len {
        // let txt = &txts[i];
        lines.push(format!("i{i}({txt})"));
    }
    lines.push("end".to_string());
    lines.push("subgraph net".to_string());
    for (i, n) in neural_net.neurons.iter().enumerate() {
        let mut txt = format!("n{i}");
        match n {
            Neuron::Input(n2) => {
                txt += &format!("(n{i} : in)");
                for iid in &n2.iids {
                    lines.push(format!("i{iid} -.-> n{i} "));
                }
            }
            Neuron::Const(n2) => {
                txt += &format!("(n{i} : const : {:.3})", n2.v);
            }
            Neuron::Weight(n2) => {
                txt += &format!("(n{i} : weight : {:.3})", n2.v);
                for sid in &n2.sids {
                    lines.push(format!("n{sid} -.-> n{i} "));
                }
            }
            Neuron::Output(n2) => {
                txt += &format!("(n{i} : out)");
                for sid in &n2.sids {
                    lines.push(format!("n{sid} -.-> n{i} "));
                }
            }
            Neuron::Decision(n2) => {
                txt += &format!("(n{i} : decision)");
                for nid in &n2.vids {
                    lines.push(format!("n{nid} -.-> n{i} "));
                }
            }
            Neuron::HardTanh(n2) => {
                txt += &format!("(n{i} : hardtanh)");
                for nid in &n2.vids {
                    lines.push(format!("n{nid} -.-> n{i} "));
                }
            }
        }
        lines.push(txt.clone());
    }
    lines.push("end".to_string());
    for i in neural_net.ins_len..(neural_net.ins_len + neural_net.outs_len) {
        lines.push(format!("n{i} -.-> decision"));
    }
    fs::write(format!("{path}_nn.mmd"), lines.join("\n")).unwrap();
    loop {
        let smallmermaid = lines.join("\n");
        let mut continue_ = false;
        for i in 0..neural_net.neurons.len() {
            let aa = format!("n{i} -.->");
            if !smallmermaid.contains(&aa) {
                for l in &mut lines {
                    let bb = format!("n{i} ");
                    if l.contains(&bb) {
                        *l = String::new();
                        continue_ = true;
                    }
                }
            }
        }
        if !continue_ {
            break;
        }
    }
    fs::write(
        format!("{path}_nn_small.mmd"),
        lines
            .iter()
            .cloned()
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .unwrap();
}
