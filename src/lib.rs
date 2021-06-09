use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeId = u64;
pub type NodeVar = String;
pub type Size = u64;
pub type Cost = i64;
pub type VarMap = HashMap<NodeVar, NodeId>;
pub type FeatMap = HashMap<NodeId, Feat>;
pub type CoordMap = HashMap<NodeId, Coord>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Prim {
    Dsp,
    Lut,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feat {
    pub size: Size,
    pub prim: Prim,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    pub size: NodeId,
    pub src: Vec<NodeId>,
    pub dst: Vec<NodeId>,
    pub var: VarMap,
    pub feat: FeatMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coord {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
    pub name: String,
    pub cost: Cost,
    pub coord: CoordMap,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stack {
    pub layout: Vec<Layout>,
}

impl Default for Coord {
    fn default() -> Self {
        Coord { x: 0, y: 0 }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            name: String::new(),
            cost: 0,
            coord: CoordMap::new(),
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            size: 0,
            src: Vec::new(),
            dst: Vec::new(),
            var: VarMap::new(),
            feat: FeatMap::new(),
        }
    }
}

impl Graph {
    pub fn get_id(&self, var: &str) -> Option<&NodeId> {
        self.var.get(var)
    }
    pub fn add_node(&mut self, var: &str) {
        self.var.insert(var.to_string(), self.size);
        self.size += 1;
    }
    pub fn add_edge(&mut self, src: &str, dst: &str) {
        let s = self.var.get(src);
        let d = self.var.get(dst);
        match (s, d) {
            (Some(a), Some(b)) => {
                self.src.push(*a);
                self.dst.push(*b);
            }
            (_, _) => (),
        }
    }
    pub fn add_feat(&mut self, var: &str, feat: Feat) {
        if let Some(id) = self.var.get(var) {
            self.feat.insert(*id, feat);
        }
    }
}

impl Layout {
    pub fn set_cost(&mut self, cost: Cost) {
        self.cost = cost;
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_coord(&mut self, id: NodeId, x: u64, y: u64) {
        self.coord.insert(id, Coord { x, y });
    }
}

impl Stack {
    pub fn add_layout(&mut self, layout: Layout) {
        self.layout.push(layout);
    }
}
