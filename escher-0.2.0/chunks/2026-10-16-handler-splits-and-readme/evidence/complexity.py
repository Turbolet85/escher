"""Reads `rust-code-analysis-cli -m -O json` output the way the version-end audit did.

Every space of kind `function` (closures included) is measured by `metrics.cognitive.sum`
(`.andromeda/runs/2026-10-15T10-44-59-code-audit/record.json`, `commands.complexity`). One line
is printed per function at or over 16, `{file}:{start_line} {name} {cognitive}`, then
`over-ceiling: {names}` in ascending byte order. Exit 1 when a parent handler reads its HEAD
figure or more (`handle_click` 74, `handle_pointerdown` 40, `handle_key_or_input_event` 56),
2 on a usage error, else 0.
"""

import json
import os
import sys

CEILING = 15
HEAD_FIGURES = {
    "handle_click": 74,
    "handle_pointerdown": 40,
    "handle_key_or_input_event": 56,
}


def functions(path):
    with open(path, encoding="utf-8") as f:
        root = json.load(f)
    stack = [root]
    while stack:
        space = stack.pop()
        stack.extend(space.get("spaces", []))
        if space["kind"] == "function":
            yield (root["name"], space["start_line"], space.get("name"),
                   space["metrics"]["cognitive"]["sum"])


def main(argv):
    if len(argv) != 2 or not os.path.isdir(argv[1]):
        print("usage: complexity.py <rust-code-analysis-cli json directory>")
        return 2
    found = []
    for dirpath, _, filenames in os.walk(argv[1]):
        for filename in filenames:
            if filename.endswith(".json"):
                found.extend(functions(os.path.join(dirpath, filename)))
    found.sort(key=lambda fn: (fn[0], fn[1]))

    over = [fn for fn in found if fn[3] > CEILING]
    for file, line, name, cognitive in over:
        print(f"{file}:{line} {name} {cognitive:g}")
    names = sorted((fn[2] for fn in over), key=lambda name: name.encode("utf-8"))
    print("over-ceiling: " + " ".join(names))

    at_head = [fn for fn in found if fn[2] in HEAD_FIGURES and fn[3] >= HEAD_FIGURES[fn[2]]]
    return 1 if at_head else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
