"""What the `escher-session` host writes to stderr while it runs commands, by RUST_LOG level and
by build. Counts only. Report-only: it grades nothing.

Run from the repository root:

    python3 escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.py

It builds the host twice, in turn, and measures each build before the next overwrites it (both
write target/debug/escher-session):

`alone`     — `cargo build -p seven_guis --bin escher-session --locked`: the stand's own feature
              set, the engine's tracing call sites compiled out.
`workspace` — `cargo test --workspace --locked --no-run`: the binary as the workspace test leg
              makes it, the engine's tracing call sites compiled in.

Per build and per level (unset, info, debug, trace) it starts the host in its foreground role on
the CRUD task, runs three commands through the binary as a client — a `snapshot`, a `click` and
a `type` of a sentinel — stops the session, and counts the host's stderr: its lines by level and
target, the command-span lines, the id needles, the name needles and the sentinel; and the
host's stdout bytes.

Each run's streams go to files under target/tmp/by-level/, are counted, and are removed.
Nothing a binary wrote is printed: a needle is reported by its own spelling and the number of
lines that hold it, a target by its name and its number of lines.
"""
import collections
import os
import re
import shutil
import subprocess
import sys
import time

BINARY = "target/debug/escher-session"
SCRATCH = "target/tmp/by-level"
LEVELS = ["(unset)", "info", "debug", "trace"]
BUILDS = [
    ("alone", ["cargo", "build", "-p", "seven_guis", "--bin", "escher-session", "--locked"]),
    ("workspace", ["cargo", "test", "--workspace", "--locked", "--no-run"]),
]
LINE = re.compile(r"^\S+ (TRACE|DEBUG|INFO|WARN|ERROR) (\S+) service\.name=")
SENTINEL = "Typed-Sentinel-40pXgd"

IDS = ["task-shell", "task-header", "back-btn", "task-title", "task-body", "crud-filter",
       "crud-list", "crud-name", "crud-surname", "crud-create", "crud-update", "crud-delete",
       "crud-person-0", "crud-person-1", "crud-person-2"]
NAMES = ["Emil, Hans", "Mustermann, Max", "Tisch, Roman", "Filter prefix: ", "Name: ",
         "Surname: "]
COMMANDS = [
    ["snapshot"],
    ["click", "--id", "crud-create"],
    ["type", "--id", "crud-name", "--text", SENTINEL],
]


def client(args, state):
    """One command through the binary as a client, RUST_LOG unset: its status and its stdout."""
    env = dict(os.environ)
    env.pop("RUST_LOG", None)
    done = subprocess.run([BINARY, *args, "--session", state], env=env, timeout=60,
                          stdin=subprocess.DEVNULL, capture_output=True)
    return done.returncode, done.stdout


def run_host(env, err, out, tag):
    state = f"target/tmp/bl-{tag}"
    host = subprocess.Popen([BINARY, "serve", "crud", "--session", state], env=env,
                            stdin=subprocess.DEVNULL, stdout=out, stderr=err)
    deadline = time.time() + 60
    answered = False
    while time.time() < deadline and host.poll() is None:
        if os.path.exists(os.path.join(state, "session.sock")):
            answered = client(["status"], state)[0] == 0
            if answered:
                break
        time.sleep(0.01)
    statuses, held = [], []
    if answered:
        for command in COMMANDS:
            status, answer = client(command, state)
            statuses.append(status)
            held.append(answer)
        client(["stop"], state)
    try:
        code = host.wait(timeout=10)
    except subprocess.TimeoutExpired:
        host.kill()
        code = host.wait()
    # The commands did handle the screen: the snapshot's answer holds the ids, the type's the
    # sentinel. Counted, never printed.
    ids_answered = sum(1 for needle in IDS if held and needle.encode() in held[0])
    typed = len(held) == 3 and SENTINEL.encode() in held[2]
    detail = (f"answered {answered} · client statuses {statuses} · snapshot answer holds "
              f"{ids_answered} of {len(IDS)} ids · type answer holds the sentinel {typed} · "
              f"state dir left {os.path.exists(state)}")
    return code, detail


def measure(build):
    os.makedirs(SCRATCH, exist_ok=True)
    for level in LEVELS:
        tag = f"{build}-{level.strip('()')}"
        env = dict(os.environ)
        env.pop("RUST_LOG", None)
        if level != "(unset)":
            env["RUST_LOG"] = level
        err_path = os.path.join(SCRATCH, f"{tag}.err")
        out_path = os.path.join(SCRATCH, f"{tag}.out")
        with open(err_path, "wb") as err, open(out_path, "wb") as out:
            code, detail = run_host(env, err, out, tag)
        with open(err_path, encoding="utf-8", errors="replace") as f:
            text = f.read()
        lines = text.splitlines()
        targets = collections.Counter()
        id_lines = collections.Counter()
        name_lines = collections.Counter()
        spans = 0
        for line in lines:
            m = LINE.match(line)
            targets[f"{m.group(1)} {m.group(2)}" if m else "(not the sink's line shape)"] += 1
            if m and m.group(2) == "escher_driver" and ' span="command"' in line:
                spans += 1
            for needle in IDS:
                if needle in line:
                    id_lines[needle] += 1
            for needle in NAMES:
                if needle in line:
                    name_lines[needle] += 1
        print(f"## {build} · RUST_LOG {level} · host exit {code} · {detail}")
        print(f"   stderr {os.path.getsize(err_path)} B · {len(lines)} lines · stdout "
              f"{os.path.getsize(out_path)} B · command-span lines {spans} for {len(COMMANDS)} "
              f"commands · sentinel occurrences {text.count(SENTINEL)}")
        print("   targets: " + (" · ".join(f"{t} ×{n}" for t, n in sorted(targets.items())) or "none"))
        print(f"   ids found {len(id_lines)} of {len(IDS)}: "
              + (" · ".join(f"{k} ×{v}" for k, v in id_lines.items()) or "none"))
        print(f"   names found {len(name_lines)} of {len(NAMES)}: "
              + (" · ".join(f"{k!r} ×{v}" for k, v in name_lines.items()) or "none"))
        sys.stdout.flush()
    shutil.rmtree(SCRATCH)


def main():
    if len(sys.argv) != 1:
        sys.exit("usage: by-level.py")
    for build, command in BUILDS:
        built = subprocess.run(command, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
        print(f"# build {build}: {' '.join(command)} · exit {built.returncode}")
        sys.stdout.flush()
        if built.returncode != 0:
            sys.exit(1)
        measure(build)


if __name__ == "__main__":
    main()
