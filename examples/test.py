import pickle


def load_pickle(fname):
    with open(fname, "rb") as fp:
        return pickle.load(fp)


def test_graph_feat_prim(graph):
    for (_, v) in graph["feat"].items():
        assert v["prim"][0] == "Lut"


def test_graph_feat_size(graph):
    for (_, v) in graph["feat"].items():
        assert v["size"] == 1


def test_graph_src(graph):
    src = [0, 0]
    assert graph["src"] == src


def test_graph_dst(graph):
    dst = [1, 2]
    assert graph["dst"] == dst


def test_graph_size(graph):
    size = 3
    assert graph["size"] == size


def test_stack_size(stack):
    size = 1
    assert len(stack["layout"]) == size


def test_stack_layout_name(stack):
    name = "i0"
    assert stack["layout"][0]["name"] == name


def test_stack_layout_cost(stack):
    cost = 45
    assert stack["layout"][0]["cost"] == cost


def test_stack_layout_coord(stack):
    coord = {0: {"x": 0, "y": 0}, 2: {"x": 0, "y": 2}, 1: {"x": 0, "y": 1}}
    assert stack["layout"][0]["coord"] == coord


if __name__ == "__main__":
    graph = load_pickle("examples/graph.pickle")
    stack = load_pickle("examples/stack.pickle")
    test_graph_size(graph)
    test_graph_src(graph)
    test_graph_dst(graph)
    test_graph_feat_prim(graph)
    test_graph_feat_size(graph)
    test_stack_size(stack)
    test_stack_layout_name(stack)
    test_stack_layout_cost(stack)
    test_stack_layout_coord(stack)
