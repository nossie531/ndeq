use crate::net::Net;
use easy_node::Nr;
use ndeq::{NdeqNet, NodeView};
use std::collections::BTreeMap;

pub struct NetBinder {
    pub net: NdeqNet<f32>,
    pub sync: Box<dyn FnMut(&NdeqNet<f32>)>,
}

impl NetBinder {
    pub fn new(src_net: Nr<Net>) -> Self {
        let mut net = NdeqNet::<f32>::new();
        let mut key_to_index = BTreeMap::new();
        let src_nodes = src_net.nodes().collect::<Vec<_>>();

        for src_node in src_nodes.iter() {
            let key = src_node.key();
            let index = net.add_node(src_node.value());
            key_to_index.insert(key, index);
        }

        for (bwd_index, src_node) in src_nodes.iter().enumerate() {
            for (key, weight) in src_node.edges() {
                let fwd_index = key_to_index[&key];
                net.add_edge(bwd_index, fwd_index, weight);
            }
        }

        let sync = Self::make_sync(src_nodes, key_to_index);

        Self { net, sync }
    }

    fn make_sync(
        src_nodes: Vec<Nr<dyn NodeView<f32>>>,
        key_to_index: BTreeMap<usize, usize>,
    ) -> Box<impl FnMut(&NdeqNet<f32>)> {
        Box::new(move |net: &NdeqNet<f32>| {
            for src_node in src_nodes.iter() {
                let index = key_to_index[&src_node.key()];
                let value = net.nodes()[index].value();
                src_node.set_value(value);
            }
        })
    }
}
