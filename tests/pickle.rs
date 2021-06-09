use rust_pickle::*;
use serde::Serialize;
use serde_pickle::{from_reader, to_writer};
use std::fs::File;
use std::process::Command;

fn run_python(file: &str) {
    let output = Command::new("python3")
        .arg(file)
        .output()
        .expect("failed to execute python3 examples/gen.py");
    assert!(output.status.success())
}

fn serialize<T: Serialize>(ty: &T, name: &str) {
    let mut file = File::create(format!("examples/{}.pickle", name)).unwrap();
    to_writer(&mut file, ty, true).unwrap();
}

fn deserialize_stack(name: &str) -> Stack {
    let file = File::open(format!("examples/{}.pickle", name)).unwrap();
    from_reader(file).unwrap()
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
    run_python("examples/test.py");
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
    run_python("examples/test.py");
}

#[test]
fn test_stack_from_py() {
    run_python("examples/gen.py");
    let mut layout = Layout::default();
    layout.set_name("layout");
    layout.set_cost(28);
    layout.set_coord(0, 0, 0);
    layout.set_coord(1, 1, 0);
    layout.set_coord(2, 2, 0);
    let mut stack = Stack::default();
    stack.add_layout(layout);
    let expect = deserialize_stack("stack_from_py");
    assert_eq!(stack, expect)
}
