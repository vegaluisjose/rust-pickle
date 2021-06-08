use serde::Serialize;
use std::collections::HashMap;

pub type NodeId = u64;
pub type Size = u64;
pub type FeatMap = HashMap<NodeId, Feat>;

#[derive(Clone, Debug, Serialize)]
pub struct Edge {
    pub src: NodeId,
    pub dst: NodeId,
}

#[derive(Clone, Debug, Serialize)]
pub enum Prim {
    Dsp,
    Lut,
}

#[derive(Clone, Debug, Serialize)]
pub struct Feat {
    pub size: Size,
    pub prim: Prim,
}

#[derive(Clone, Debug, Serialize)]
pub struct Graph {
    pub size: NodeId,
    pub edge: Vec<Edge>,
    pub feat: FeatMap,
}

impl Edge {
    pub fn new(src: NodeId, dst: NodeId) -> Self {
        Edge { src, dst }
    }
}
