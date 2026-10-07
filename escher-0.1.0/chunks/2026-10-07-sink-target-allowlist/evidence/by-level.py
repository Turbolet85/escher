"""What a sink-installing seven_guis binary writes to stderr, by RUST_LOG level. Counts only.

Run from the repository root, after `cargo build -p seven_guis --bins --locked` (or name the
build you measured beside the table you record):

    python3 escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.py host
    python3 escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.py windowed

`host`     — target/debug/escher-session on the CRUD task: one `hello v1`, then `stop v1` over its
             socket. Headless.
`windowed` — target/debug/seven_guis_native, left on Home for 10 s and stopped by `timeout`
             (exit 124 is the stand staying up). Needs a display; opens a window each run.

Each level's stdout and stderr go to files under target/tmp/by-level/, are counted, and are
removed. Nothing a binary wrote is printed: a needle is reported by its own spelling and the
number of lines that hold it, a target by its name and its number of lines.
"""
import collections
import os
import re
import shutil
import socket
import subprocess
import sys
import time

SCRATCH = "target/tmp/by-level"
LEVELS = ["(unset)", "info", "debug", "trace"]
LINE = re.compile(r"^\S+ (TRACE|DEBUG|INFO|WARN|ERROR) (\S+) service\.name=")

HOST_IDS = ["task-shell", "task-header", "back-btn", "task-title", "task-body", "crud-filter",
            "crud-list", "crud-name", "crud-surname", "crud-create", "crud-update", "crud-delete",
            "crud-person-0", "crud-person-1", "crud-person-2"]
HOST_NAMES = ["Emil, Hans", "Mustermann, Max", "Tisch, Roman", "Filter prefix: ", "Name: ",
              "Surname: "]
HOME_IDS = ["home-header", "home-title", "home-subtitle", "task-grid", "task-card-counter",
            "task-card-temp-converter", "task-card-flight-booker", "task-card-timer",
            "task-card-crud", "task-card-circle-drawer", "task-card-cells"]
HOME_NAMES = ["7GUIs", "Seven benchmark tasks for GUI frameworks", "Temp Converter",
              "Flight Booker", "Circle Drawer"]


def ask(path, line):
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
        s.settimeout(5)
        s.connect(path)
        s.sendall(line.encode())
        return s.recv(256).decode().strip()


def run_host(env, err, out, tag):
    state = f"target/tmp/bl-{tag}"
    host = subprocess.Popen(["target/debug/escher-session", "crud", state], env=env,
                            stdin=subprocess.DEVNULL, stdout=out, stderr=err)
    sock = os.path.join(state, "session.sock")
    deadline = time.time() + 60
    answered = False
    while time.time() < deadline and host.poll() is None:
        if os.path.exists(sock):
            try:
                answered = ask(sock, "hello v1\n").startswith("ok v1 pid=")
                break
            except OSError:
                pass
        time.sleep(0.01)
    at_ready = os.fstat(err.fileno()).st_size
    if answered:
        ask(sock, "stop v1\n")
    try:
        code = host.wait(timeout=10)
    except subprocess.TimeoutExpired:
        host.kill()
        code = host.wait()
    return code, f"answered {answered} · {at_ready} B before the answer · state dir left {os.path.exists(state)}"


def run_windowed(env, err, out, _tag):
    code = subprocess.call(["timeout", "10", "target/debug/seven_guis_native"], env=env,
                           stdin=subprocess.DEVNULL, stdout=out, stderr=err)
    return code, "exit 124 = still up when timeout stopped it"


def main():
    mode = sys.argv[1] if len(sys.argv) == 2 else ""
    if mode not in ("host", "windowed"):
        sys.exit("usage: by-level.py host|windowed")
    run, ids, names = (run_host, HOST_IDS, HOST_NAMES) if mode == "host" else (
        run_windowed, HOME_IDS, HOME_NAMES)
    os.makedirs(SCRATCH, exist_ok=True)
    for level in LEVELS:
        tag = level.strip("()")
        env = dict(os.environ)
        env.pop("RUST_LOG", None)
        if level != "(unset)":
            env["RUST_LOG"] = level
        err_path = os.path.join(SCRATCH, f"{mode}-{tag}.err")
        out_path = os.path.join(SCRATCH, f"{mode}-{tag}.out")
        with open(err_path, "wb") as err, open(out_path, "wb") as out:
            code, detail = run(env, err, out, tag)
        with open(err_path, encoding="utf-8", errors="replace") as f:
            lines = f.read().splitlines()
        targets = collections.Counter()
        id_lines = collections.Counter()
        name_lines = collections.Counter()
        for line in lines:
            m = LINE.match(line)
            targets[f"{m.group(1)} {m.group(2)}" if m else "(not the sink's line shape)"] += 1
            for needle in ids:
                if needle in line:
                    id_lines[needle] += 1
            for needle in names:
                if needle in line:
                    name_lines[needle] += 1
        print(f"## {mode} · RUST_LOG {level} · exit {code} · {detail}")
        print(f"   stderr {os.path.getsize(err_path)} B · {len(lines)} lines · stdout "
              f"{os.path.getsize(out_path)} B")
        print("   targets: " + (" · ".join(f"{t} ×{n}" for t, n in sorted(targets.items())) or "none"))
        print(f"   ids found {len(id_lines)} of {len(ids)}: "
              + (" · ".join(f"{k} ×{v}" for k, v in id_lines.items()) or "none"))
        print(f"   names found {len(name_lines)} of {len(names)}: "
              + (" · ".join(f"{k!r} ×{v}" for k, v in name_lines.items()) or "none"))
    shutil.rmtree(SCRATCH)


if __name__ == "__main__":
    main()
