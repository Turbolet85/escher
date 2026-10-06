"""Re-point `path:N[-M]` citations by the 2026-10-06-accessibility-tree-identity line map.

Usage: python3 repoint.py {--dry|--apply} FILE...
Skips lines inside a `## Session Additions` section and inside `USER:` marker blocks.
"""

import re
import sys


def acc(n):
    if n <= 9:
        return n
    if n <= 27:
        return n + 1
    if n <= 30:
        return n + 7
    if n <= 69:
        return n + 18
    return n + 26


def doc(n):
    return n if n <= 150 else n + 10


def dd(n):
    if n <= 17:
        return n
    if n <= 284:
        return n + 2
    return n + 20


def dnd_cargo(n):
    return n if n <= 38 else n + 1


def shell_acc(n):
    if n <= 44:
        return n
    if n <= 47:
        return 45
    return n - 2


def window(n):
    if n <= 523:
        return n
    if n == 524:
        return 523
    return n - 1


def flight(n):
    if n <= 81:
        return n
    if n <= 87:
        return n + 1
    return n + 2


MAPS = {
    "packages/blitz-dom/src/accessibility.rs": acc,
    "packages/blitz-dom/src/document.rs": doc,
    "packages/dioxus-native-dom/src/dioxus_document.rs": dd,
    "packages/dioxus-native-dom/Cargo.toml": dnd_cargo,
    "packages/blitz-shell/src/accessibility.rs": shell_acc,
    "packages/blitz-shell/src/window.rs": window,
    "examples/seven_guis/src/tasks/flight_booker.rs": flight,
}

PAT = re.compile(
    r"(?<![\w/.-])(" + "|".join(re.escape(p) for p in MAPS) + r"):(\d+)(?:-(\d+))?(?!\d)"
)


def remap(m):
    f = MAPS[m.group(1)]
    a = int(m.group(2))
    na = f(a)
    if m.group(3):
        b = int(m.group(3))
        nb = f(b)
        if na == nb:
            return f"{m.group(1)}:{na}"
        return f"{m.group(1)}:{na}-{nb}"
    return f"{m.group(1)}:{na}"


def main():
    mode = sys.argv[1]
    total = 0
    for path in sys.argv[2:]:
        with open(path, encoding="utf-8", newline="") as fh:
            lines = fh.read().split("\n")
        in_sa = False
        in_user = False
        changed = 0
        out = []
        for i, line in enumerate(lines, 1):
            if line.startswith("## "):
                in_sa = line.strip() == "## Session Additions"
            if "<!-- USER:" in line and line.rstrip().endswith("start -->"):
                in_user = True
            if in_sa or in_user:
                out.append(line)
                if "<!-- USER:" in line and line.rstrip().endswith("end -->"):
                    in_user = False
                continue
            new = PAT.sub(remap, line)
            if new != line:
                olds = [m.group(0) for m in PAT.finditer(line)]
                news = [remap(m) for m in PAT.finditer(line)]
                for o, n in zip(olds, news):
                    if o != n:
                        changed += 1
                        print(f"{path}:{i}: {o} -> {n}")
            out.append(new)
        total += changed
        if mode == "--apply" and changed:
            with open(path, "w", encoding="utf-8", newline="") as fh:
                fh.write("\n".join(out))
    print(f"total re-pointed citations: {total}")


main()
