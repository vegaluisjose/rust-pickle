use rust_pickle::*;
use serde_pickle::to_writer;
use std::fs::File;
use std::process::Command;

#[test]
fn test_fromrust() {
    let mut edge: Vec<Edge> = Vec::new();
    edge.push(Edge::new(0, 1));
    edge.push(Edge::new(0, 2));
    let node = Feat {
        size: 1,
        prim: Prim::Lut,
    };
    let mut feat = FeatMap::new();
    feat.insert(0, node.clone());
    feat.insert(1, node.clone());
    feat.insert(2, node.clone());
    let graph = Graph {
        size: 3,
        edge,
        feat,
    };
    let mut file = File::create("examples/fromrust.pickle").unwrap();
    to_writer(&mut file, &graph, true).unwrap();
    let output = Command::new("python3")
        .arg("examples/test.py")
        .output()
        .expect("failed to execute python3 examples/test.py");
    assert!(output.status.success());
}
