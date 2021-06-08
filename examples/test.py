import pickle


def load_pickle(fname):
    with open(fname, "rb") as fp:
        return pickle.load(fp)


def test_feat_prim(data):
    for (_, v) in data["feat"].items():
        assert v["prim"][0] == "Lut"


def test_feat_size(data):
    for (_, v) in data["feat"].items():
        assert v["size"] == 1


def test_edge(data):
    edge = [{"src": 0, "dst": 1}, {"src": 0, "dst": 2}]
    assert data["edge"] == edge


def test_size(data):
    size = 3
    assert data["size"] == size


if __name__ == "__main__":
    data = load_pickle("examples/fromrust.pickle")
    test_feat_prim(data)
    test_feat_size(data)
