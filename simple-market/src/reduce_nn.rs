use crate::NeuralNet;
use crate::Neuron;
pub fn reduce_nn(nn: &mut NeuralNet) {
    loop {
        let mut breakout = true;
        let l = nn.neurons.len();
        // do we keep n1 ?
        // yes, for all input
        // yes, for all output
        for nid1 in (nn.ins_len + nn.outs_len + 1..l).rev() {
            let mut keep = false;
            for (nid2, neuro2) in nn.neurons.iter().enumerate() {
                if nid1 == nid2 {
                    continue;
                }
                match neuro2 {
                    Neuron::Input(_) => {}
                    Neuron::Const(_) => {}
                    Neuron::Decision(n2) => {
                        if n2.vids.contains(&nid1) {
                            keep = true;
                            break;
                        }
                    }
                    Neuron::Weight(n2) => {
                        if n2.sids.contains(&nid1) {
                            keep = true;
                            break;
                        }
                    }
                    Neuron::Output(n2) => {
                        if n2.sids.contains(&nid1) {
                            keep = true;
                            break;
                        }
                    }
                    Neuron::HardTanh(n2) => {
                        if n2.vids.contains(&nid1) {
                            keep = true;
                            break;
                        }
                    }
                }
            }
            if !keep {
                breakout = false;
                nn.neurons.remove(nid1);
                for neuro2 in &mut nn.neurons {
                    match neuro2 {
                        Neuron::Input(_) => {}
                        Neuron::Const(_) => {}
                        Neuron::Weight(n2) => {
                            for x in &mut n2.sids {
                                if *x >= nid1 {
                                    *x -= 1;
                                }
                            }
                        }
                        Neuron::Output(n2) => {
                            for x in &mut n2.sids {
                                if *x >= nid1 {
                                    *x -= 1;
                                }
                            }
                        }
                        Neuron::Decision(n2) => {
                            for x in &mut n2.vids {
                                if *x >= nid1 {
                                    *x -= 1;
                                }
                            }
                        }
                        Neuron::HardTanh(n2) => {
                            for x in &mut n2.vids {
                                if *x >= nid1 {
                                    *x -= 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if breakout {
            break;
        }
    }
}
