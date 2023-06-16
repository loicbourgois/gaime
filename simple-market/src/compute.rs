use crate::Neuron;
pub fn compute(n: &Neuron, neuro_outs: &[f32], ins: &[f32]) -> f32 {
    return match n {
        Neuron::Const(n) => n.v,
        Neuron::Decision(n) => match n
            .vids
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| neuro_outs[**a].total_cmp(&neuro_outs[**b]))
            .map(|(index, _)| index)
        {
            Some(x) => (x as f32) / (n.vids.len() as f32),
            None => 0.0,
        },
        Neuron::HardTanh(n) => n
            .vids
            .iter()
            .map(|x| neuro_outs[*x])
            .sum::<f32>()
            .max(-1.0)
            .min(1.0),
        Neuron::Weight(n) => n.sids.iter().map(|x| neuro_outs[*x]).sum::<f32>() * n.v,
        Neuron::Output(n) => n.sids.iter().map(|x| neuro_outs[*x]).sum::<f32>(),
        Neuron::Input(n) => n.iids.iter().map(|x| ins[*x]).sum::<f32>(),
    };
}
