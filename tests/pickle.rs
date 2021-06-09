use rust_pickle::*;
use serde_pickle::to_writer;
use std::fs::File;
use std::process::Command;
use serde::Serialize;

fn run_test() -> bool {
    let output = Command::new("python3")
        .arg("examples/test.py")
        .output()
        .expect("failed to execute python3 examples/test.py");
    output.status.success()
}

fn serialize<T: Serialize>(ty: &T, name: &str) {
    let mut file = File::create(format!("examples/{}.pickle", name)).unwrap();
    to_writer(&mut file, ty, true).unwrap();
}

#[test]
fn test_graph() {
    let mut graph = Graph::default();
    let feat = Feat {
        size: 1,
        prim: Prim::Lut,
    };
    graph.add_node("t0");
    graph.add_node("t1");
    graph.add_node("t2");
    graph.add_edge("t0", "t1");
    graph.add_edge("t0", "t2");
    graph.add_feat("t0", feat.clone());
    graph.add_feat("t1", feat.clone());
    graph.add_feat("t2", feat.clone());
    serialize(&graph, "graph");
    assert!(run_test());
}

#[test]
fn test_stack() {
    let mut layout = Layout::default();
    layout.set_name("i0");
    layout.set_cost(45);
    layout.set_coord(0, 0, 0);
    layout.set_coord(1, 0, 1);
    layout.set_coord(2, 0, 2);
    let mut stack = Stack::default();
    stack.add_layout(layout);
    serialize(&stack, "stack");
    assert!(run_test());
}
