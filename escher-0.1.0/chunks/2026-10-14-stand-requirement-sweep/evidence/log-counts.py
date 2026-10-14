"""What the `escher-session` host and its `mcp` role write to stderr while they run one CRUD flow,
at RUST_LOG `info` and `trace`, under both builds. Counts only. Report-only: it grades nothing.

Run from the repository root:

    python3 escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.py

It builds the binary twice, in turn, and measures each build before the next overwrites it (both
write target/debug/escher-session):

`alone`     — `cargo build -p seven_guis --bin escher-session --locked`: the stand's own feature
              set, the engine's tracing call sites compiled out.
`workspace` — `cargo build --workspace --locked`: the engine's tracing call sites compiled in.

The host flow is 2026-10-10-driver-cli's by-level procedure: the host in its foreground role on
CRUD, three commands through the binary as a client with RUST_LOG unset — a `snapshot`, a `click`
on Create and a `type` of a sentinel into the name field — then `stop`. The `mcp` flow is
2026-10-10-mcp-surface's: `initialize`, then tool calls `start crud`, `snapshot`, `type` into
`crud-name`, `click crud-create`, `stop`, fed on the role's stdin, its stdout drained on a thread.

Each run's streams go to files under target/tmp/log-counts/, are counted, and are removed. Nothing
a binary wrote is printed: per run, the stderr's lines and bytes, its lines per level and target,
the `command` and `tool_call` span lines, the id and name needles found, and the sentinel's
occurrences.
"""
import collections
import json
import os
import re
import shutil
import subprocess
import sys
import threading
import time

BINARY = "target/debug/escher-session"
SCRATCH = "target/tmp/log-counts"
LEVELS = ["info", "trace"]
BUILDS = [
    ("alone", ["cargo", "build", "-p", "seven_guis", "--bin", "escher-session", "--locked"]),
    ("workspace", ["cargo", "build", "--workspace", "--locked"]),
]
LINE = re.compile(r"^\S+ (TRACE|DEBUG|INFO|WARN|ERROR) (\S+) service\.name=")
SENTINEL = "Typed-Sentinel-40pXgd"

IDS = ["task-shell", "task-header", "back-btn", "task-title", "task-body", "crud-filter",
       "crud-list", "crud-name", "crud-surname", "crud-create", "crud-update", "crud-delete",
       "crud-person-0", "crud-person-1", "crud-person-2"]
NAMES = ["Emil, Hans", "Mustermann, Max", "Tisch, Roman", "Filter prefix:", "Name:", "Surname:",
         "People"]
COMMANDS = [
    ["snapshot"],
    ["click", "--id", "crud-create"],
    ["type", "--id", "crud-name", "--text", SENTINEL],
]
TOOL_CALLS = [
    ("start", {"app": "crud"}),
    ("snapshot", {}),
    ("type", {"id": "crud-name", "text": SENTINEL}),
    ("click", {"id": "crud-create"}),
    ("stop", {}),
]


def env_at(level):
    env = dict(os.environ)
    env["RUST_LOG"] = level
    return env


def client(args, state):
    """One command through the binary as a client, RUST_LOG unset: its status."""
    env = dict(os.environ)
    env.pop("RUST_LOG", None)
    done = subprocess.run([BINARY, *args, "--session", state], env=env, timeout=60,
                          stdin=subprocess.DEVNULL, capture_output=True)
    return done.returncode


def host_flow(level, err, out, tag):
    state = f"target/tmp/lc-{tag}"
    host = subprocess.Popen([BINARY, "serve", "crud", "--session", state], env=env_at(level),
                            stdin=subprocess.DEVNULL, stdout=out, stderr=err)
    deadline = time.time() + 60
    answered = False
    while time.time() < deadline and host.poll() is None:
        if os.path.exists(os.path.join(state, "session.sock")):
            answered = client(["status"], state) == 0
            if answered:
                break
        time.sleep(0.01)
    statuses = []
    if answered:
        statuses = [client(command, state) for command in COMMANDS]
        client(["stop"], state)
    try:
        code = host.wait(timeout=10)
    except subprocess.TimeoutExpired:
        host.kill()
        code = host.wait()
    return code, f"client statuses {statuses} · state dir left {os.path.exists(state)}"


def mcp_flow(level, err, out, tag):
    role = subprocess.Popen([BINARY, "mcp"], env=env_at(level), stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, stderr=err)
    answers = []

    def drain():
        for line in role.stdout:
            out.write(line)
            answers.append(line)

    reader = threading.Thread(target=drain)
    reader.start()
    requests = [{"jsonrpc": "2.0", "id": 1, "method": "initialize",
                 "params": {"protocolVersion": "2025-11-25", "capabilities": {},
                            "clientInfo": {"name": "log-counts", "version": "0"}}},
                {"jsonrpc": "2.0", "method": "notifications/initialized"}]
    for n, (tool, arguments) in enumerate(TOOL_CALLS, start=2):
        requests.append({"jsonrpc": "2.0", "id": n, "method": "tools/call",
                         "params": {"name": tool, "arguments": arguments}})
    expected = 1 + len(TOOL_CALLS)
    for request in requests:
        role.stdin.write((json.dumps(request) + "\n").encode())
        role.stdin.flush()
        if "id" in request:
            deadline = time.time() + 30
            while len(answers) < request["id"] and time.time() < deadline:
                time.sleep(0.01)
    role.stdin.close()
    try:
        code = role.wait(timeout=30)
    except subprocess.TimeoutExpired:
        role.kill()
        code = role.wait()
    reader.join(timeout=5)
    errors = sum(1 for line in answers if b'"isError":true' in line)
    return code, f"stdout lines {len(answers)} of {expected} · isError {errors}"


def count(build, flow, level, run):
    tag = f"{build}-{flow}-{level}"
    err_path = os.path.join(SCRATCH, f"{tag}.err")
    out_path = os.path.join(SCRATCH, f"{tag}.out")
    with open(err_path, "wb") as err, open(out_path, "wb") as out:
        code, detail = run(level, err, out, tag)
    with open(err_path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    lines = text.splitlines()
    targets = collections.Counter()
    spans = collections.Counter()
    ids = set()
    names = set()
    for line in lines:
        m = LINE.match(line)
        targets[f"{m.group(1)} {m.group(2)}" if m else "(not the sink's line shape)"] += 1
        for span in ("command", "tool_call"):
            if m and m.group(2) == "escher_driver" and f' span="{span}"' in line:
                spans[span] += 1
        ids.update(needle for needle in IDS if needle in line)
        names.update(needle for needle in NAMES if needle in line)
    print(f"## {build} · {flow} · RUST_LOG {level} · exit {code} · {detail}")
    print(f"   stderr {os.path.getsize(err_path)} B · {len(lines)} lines · command {spans['command']}"
          f" · tool_call {spans['tool_call']} · id needles {len(ids)} of {len(IDS)} · name "
          f"needles {len(names)} of {len(NAMES)} · sentinel {text.count(SENTINEL)}")
    print("   targets: " + (" · ".join(f"{t} ×{n}" for t, n in sorted(targets.items())) or "none"))
    sys.stdout.flush()


def main():
    if len(sys.argv) != 1:
        sys.exit("usage: log-counts.py")
    for build, command in BUILDS:
        built = subprocess.run(command, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
        print(f"# build {build}: {' '.join(command)} · exit {built.returncode}")
        sys.stdout.flush()
        if built.returncode != 0:
            sys.exit(1)
        os.makedirs(SCRATCH, exist_ok=True)
        for level in LEVELS:
            count(build, "host", level, host_flow)
            count(build, "mcp", level, mcp_flow)
        shutil.rmtree(SCRATCH)


if __name__ == "__main__":
    main()
