use crate::net::Net;
use crate::node::Node;
use easy_node::prelude::*;
use ndeq::prelude::*;
use std::array;
use std::ops::Range;

type Xy = (f32, f32);
pub const T_RANGE: Range<f32> = 0.0..1.0;
pub const V_RANGE: Range<f32> = 0.0..1.0;
const H: f32 = 0.2;

pub struct Sample {
    nodes: [Nr<Node>; 3],
    seriese_vec: [Vec<Xy>; 3],
}

impl Sample {
    pub fn new() -> Self {
        let net = Net::new();
        let nodes = array::from_fn::<Nr<Node>, 3, _>(|_| net.add_node());
        let seriese_vec = array::from_fn::<Vec<Xy>, 3, _>(|_| vec![]);

        nodes[0].set_value(0.1);
        nodes[1].set_value(0.3);
        nodes[2].set_value(0.9);
        nodes[0].set_edge(&nodes[1], 1.0);
        nodes[1].set_edge(&nodes[2], 1.0);
        nodes[2].set_edge(&nodes[0], 1.0);

        Self { nodes, seriese_vec }
    }

    pub fn series_vec(&self) -> &[Vec<Xy>; 3] {
        &self.seriese_vec
    }

    pub fn run_simulation(&mut self) {
        let mut sim = NdeqSim::new(Euler::new(H));
        let nodes = self.nodes.iter().map(Node::conv).collect::<Vec<_>>();
        let nodes = nodes.iter().map(|x| &**x).collect::<Vec<_>>();
        sim.set_nodes(nodes.iter().cloned());

        let mut t = T_RANGE.start;
        while t <= T_RANGE.end {
            let values = nodes.iter().map(|x| x.value() as f32);
            let values = values.collect::<Vec<_>>();
            self.seriese_vec[0].push((t, values[0]));
            self.seriese_vec[1].push((t, values[1]));
            self.seriese_vec[2].push((t, values[2]));
            sim.run(H);
            t += H;
        }
    }
}
