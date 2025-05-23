use crate::net::Net;
use crate::node::Node;
use easy_node::prelude::*;
use ndeq::net_ode::solver::adapters::NetEuler;
use ndeq::prelude::*;
use std::ops::Range;

type Xy = (f32, f32);
pub const T_RANGE: Range<f32> = 0.0..1.0;
pub const V_RANGE: Range<f32> = 0.0..1.0;
const H: f32 = 0.2;

pub struct Sample {
    net: Nr<Net>,
    nodes: [Nr<Node>; 3],
    seriese_vec: [Vec<Xy>; 3],
}

impl Sample {
    pub fn new() -> Self {
        let net = Net::new();
        let nodes = [(); 3].map(|_| net.add_node());
        let seriese_vec = [(); 3].map(|_| vec![]);

        nodes[0].set_value(0.1);
        nodes[1].set_value(0.3);
        nodes[2].set_value(0.9);
        nodes[0].add_edge(&nodes[1], 1.0);
        nodes[1].add_edge(&nodes[2], 1.0);
        nodes[2].add_edge(&nodes[0], 1.0);

        Self {
            net,
            nodes,
            seriese_vec,
        }
    }

    pub fn series_vec(&self) -> &[Vec<Xy>; 3] {
        &self.seriese_vec
    }

    pub fn run_simulation(&mut self) {
        let net = &*self.net as &dyn NdeqNet<f32>;
        let solver = NetEuler::new(H);
        let mut sim = NdeqSim::new(net, &solver);

        let mut t = T_RANGE.start;
        while t <= T_RANGE.end {
            let values = self.nodes.iter().map(|x| x.value());
            let values = values.collect::<Vec<_>>();
            self.seriese_vec[0].push((t, values[0]));
            self.seriese_vec[1].push((t, values[1]));
            self.seriese_vec[2].push((t, values[2]));
            sim.run(H);
            t += H;
        }
    }
}
