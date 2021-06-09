import pickle


def save_pickle(fname, obj):
    with open(fname, "wb") as fp:
        pickle.dump(obj, fp)


if __name__ == "__main__":
    stack = {
        "layout": [
            {
                "name": "layout",
                "cost": 28,
                "coord": {
                    2: {"x": 2, "y": 0},
                    1: {"x": 1, "y": 0},
                    0: {"x": 0, "y": 0},
                },
            }
        ]
    }
    save_pickle(
        "examples/stack_from_py.pickle",
        stack,
    )
