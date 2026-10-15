"""What escher's binaries write to stderr, by RUST_LOG level and by build. Counts only; report-only.

Run from the repository root:

    python3 escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.py

It builds twice, in turn, and measures each build before the next overwrites target/debug:

`alone`     — `cargo build -p seven_guis --locked`: the stand's own feature set, the engine's
              tracing call sites compiled out.
`workspace` — `cargo build --workspace --locked`: the binaries as the workspace builds them, the
              engine's tracing call sites compiled in.

Per build and per level (unset, info, debug, trace):

(a) `client` — the `escher-session` client's own stderr, each command run with RUST_LOG at the
    level, over one flow on CRUD: start · snapshot · click · type (a sentinel) · screenshot · stop.
    The host `start` spawns writes to /dev/null, so this is the client's alone.
(b) `host-counter`, `host-flight` — the host in its foreground role (`serve`), RUST_LOG at the level,
    stderr to a file, driven by the client (RUST_LOG unset) through the commands of
    examples/seven_guis/tests/flows/{counter,flight}.sh, then stopped.
(c) `windowed` — workspace build only: `timeout 10 target/debug/seven_guis_native`, on Home for 10 s,
    stderr to a file. The `timeout` this script starts ends it; no process is signalled by name.

For each run: stderr bytes and lines, lines by level and target, command-span lines, and the needle
counts — ids (every id holding a `-` that a snapshot answer of the run's task read), names (the
screen's texts of four characters or more), the typed sentinels, and the first 64 base64 characters
of the shot's PNG. State directories are relative, target/tmp/qg-*; every stream goes to
target/tmp/qg-streams/ and is removed. Nothing a binary wrote is printed.
"""
import collections
import json
import os
import re
import shutil
import subprocess
import sys
import time

BINARY = "target/debug/escher-session"
WINDOWED = "target/debug/seven_guis_native"
SCRATCH = "target/tmp/qg-streams"
LEVELS = ["(unset)", "info", "debug", "trace"]
BUILDS = [
    ("alone", ["cargo", "build", "-p", "seven_guis", "--locked"]),
    ("workspace", ["cargo", "build", "--workspace", "--locked"]),
]
LINE = re.compile(r"^\S+ (TRACE|DEBUG|INFO|WARN|ERROR) (\S+) ")
SENTINEL = "Typed-Sentinel-qg7Rm2"
ID = re.compile(r'id="([^"]+)"')
NAME = re.compile(r'^\s*\S+ "([^"]+)"', re.M)
HOME_NAMES = ["Temp Converter", "Flight Booker", "Circle Drawer", "Constraints", "Concurrency",
              "Reactivity", "List Ops", "Data Flow"]

CLIENT_FLOW = [
    ["start", "crud"],
    ["snapshot"],
    ["click", "--id", "crud-create"],
    ["type", "--id", "crud-name", "--text", SENTINEL],
    ["screenshot"],
    ["stop"],
]
FLOWS = {
    "counter": [
        ["snapshot"],
        ["click", "--id", "counter-increment"],
        ["snapshot"],
        ["click", "--id", "flow-names-no-element"],
        ["snapshot"],
    ],
    "flight-booker": [
        ["snapshot"],
        ["type", "--id", "flight-start", "--text", "next week"],
        ["click", "--id", "flight-book"],
        ["type", "--id", "flight-start", "--text", "24.12.2026"],
        ["click", "--id", "flight-book"],
    ],
}
TYPED = {"crud": [SENTINEL], "counter": [], "flight-booker": ["next week", "24.12.2026"]}


def env_at(level):
    env = dict(os.environ)
    env.pop("RUST_LOG", None)
    if level != "(unset)":
        env["RUST_LOG"] = level
    return env


def client(args, state, level="(unset)", err=subprocess.DEVNULL):
    done = subprocess.run([BINARY, *args, "--session", state], env=env_at(level), timeout=90,
                          stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=err)
    return done.returncode, done.stdout.decode("utf-8", "replace")


def needles_of(answers):
    """Ids and names a snapshot answer read, and the shot's first 64 base64 characters."""
    ids, names, shot = set(), set(), None
    for answer in answers:
        try:
            value = json.loads(answer)
        except ValueError:
            continue
        if isinstance(value.get("text"), str):
            ids.update(i for i in ID.findall(value["text"]) if "-" in i)
            names.update(n for n in NAME.findall(value["text"])
                         if len(n) >= 4 and any(c.isalpha() for c in n))
        if isinstance(value.get("png"), str):
            shot = value["png"][:64]
    return ids, names, shot


def count(path, ids, names, typed, shot):
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    lines = text.splitlines()
    targets = collections.Counter()
    spans = 0
    for line in lines:
        m = LINE.match(line)
        targets[f"{m.group(1)} {m.group(2)}" if m else "(not the sink's line shape)"] += 1
        if m and m.group(2) == "escher_driver" and ' span="command"' in line:
            spans += 1
    return {
        "bytes": os.path.getsize(path),
        "lines": len(lines),
        "targets": " · ".join(f"{t} ×{n}" for t, n in sorted(targets.items())) or "none",
        "spans": spans,
        "ids": sum(1 for n in ids if n in text),
        "names": sum(1 for n in names if n in text),
        "typed": sum(text.count(t) for t in typed),
        "shot": text.count(shot) if shot else None,
    }


def report(build, level, run, detail, c, ids, names, typed, shot):
    shot_word = "no shot" if shot is None else f"{c['shot']}"
    print(f"| {build} | {run} | {level} | {c['bytes']} B · {c['lines']} | {c['targets']} | "
          f"{c['spans']} | {c['ids']} of {len(ids)} | {c['names']} of {len(names)} | "
          f"{c['typed']} of {len(typed)} sentinels | {shot_word} | {detail} |")
    sys.stdout.flush()


def run_client(build, level, tag):
    state = f"target/tmp/qg-c-{tag}"
    err_path = os.path.join(SCRATCH, f"client-{tag}.err")
    statuses, answers = [], []
    with open(err_path, "wb") as err:
        for command in CLIENT_FLOW:
            status, answer = client(command, state, level, err)
            statuses.append(status)
            answers.append(answer)
    if os.path.exists(state):
        client(["stop"], state)
    ids, names, shot = needles_of(answers)
    c = count(err_path, ids, names, TYPED["crud"], shot)
    detail = f"statuses {statuses} · state dir left {os.path.exists(state)}"
    report(build, level, "(a) client · crud", detail, c, ids, names, TYPED["crud"], shot)


def run_host(build, level, tag, task):
    state = f"target/tmp/qg-h-{tag}-{task[:6]}"
    err_path = os.path.join(SCRATCH, f"host-{tag}-{task}.err")
    out_path = os.path.join(SCRATCH, f"host-{tag}-{task}.out")
    with open(err_path, "wb") as err, open(out_path, "wb") as out:
        host = subprocess.Popen([BINARY, "serve", task, "--session", state], env=env_at(level),
                                stdin=subprocess.DEVNULL, stdout=out, stderr=err)
        deadline = time.time() + 60
        ready = False
        while time.time() < deadline and host.poll() is None:
            if os.path.exists(os.path.join(state, "session.sock")):
                ready = client(["status"], state)[0] == 0
                if ready:
                    break
            time.sleep(0.01)
        statuses, answers = [], []
        if ready:
            for command in FLOWS[task]:
                status, answer = client(command, state)
                statuses.append(status)
                answers.append(answer)
            client(["stop"], state)
        try:
            code = host.wait(timeout=15)
        except subprocess.TimeoutExpired:
            host.kill()
            code = host.wait()
    ids, names, shot = needles_of(answers)
    c = count(err_path, ids, names, TYPED[task], shot)
    detail = (f"host exit {code} · statuses {statuses} · stdout {os.path.getsize(out_path)} B · "
              f"state dir left {os.path.exists(state)}")
    report(build, level, f"(b) host · {task}", detail, c, ids, names, TYPED[task], shot)
    return ids, names


def run_windowed(build, level, tag, ids, names):
    err_path = os.path.join(SCRATCH, f"windowed-{tag}.err")
    with open(err_path, "wb") as err:
        app = subprocess.Popen(["timeout", "10", WINDOWED], env=env_at(level),
                               stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=err)
        try:
            code = app.wait(timeout=40)
        except subprocess.TimeoutExpired:
            app.kill()
            code = app.wait()
    all_names = names | set(HOME_NAMES)
    c = count(err_path, ids, all_names, [], None)
    report(build, level, "(c) windowed · home", f"exit {code} (124: ended by its timeout)", c,
           ids, all_names, [], None)


def main():
    if len(sys.argv) != 1:
        sys.exit("usage: stderr-by-level.py")
    print("| build | run | RUST_LOG | stderr | lines by level and target | command spans | ids | "
          "names | typed | shot base64 | run |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for build, command in BUILDS:
        built = subprocess.run(command, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
        print(f"| {build} | build | `{' '.join(command)}` exit {built.returncode} | | | | | | | | |")
        sys.stdout.flush()
        if built.returncode != 0:
            sys.exit(1)
        os.makedirs(SCRATCH, exist_ok=True)
        for level in LEVELS:
            tag = f"{build[0]}{level.strip('()')[0]}"
            run_client(build, level, tag)
            all_ids, all_names = set(), set()
            for task in FLOWS:
                ids, names = run_host(build, level, tag, task)
                all_ids |= ids
                all_names |= names
            if build == "workspace":
                run_windowed(build, level, tag, all_ids, all_names)
        shutil.rmtree(SCRATCH)


if __name__ == "__main__":
    main()
