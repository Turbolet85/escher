#!/usr/bin/env bash
# The agent-driven test contract over the stand checks and blitz-tests: five verbs, each printing JSON lines and
# exiting 0 success · 1 the verb ran and failed · 2 usage · 3 a precondition is unmet. Run it from the repository
# root. State lives in target/agent-run/ only: status.json, events.jsonl (harness events, never captured test
# output) and run.log (the raw cargo output). scripts/agent-run.ps1 delegates here on Windows.
set -euo pipefail

VERBS=(boot run status cleanup logs)
STATE_DIR=target/agent-run
TESTS_DIR=tests/blitz-tests/tests

usage() {
    echo "usage: $0 <verb> [selection]" >&2
    echo "verbs: ${VERBS[*]}" >&2
    echo "run selections: stand · all · <blitz-tests file name>" >&2
    exit 2
}

ensure_fresh_artifacts() {
    # A no-op: boot rebuilds every blitz-tests binary itself, so no stale artifact can reach a run.
    :
}

# libtest's JSON format is nightly-only, so per-test events are parsed from its pretty output. Every event is
# encoded by python's json module: doc-test names carry spaces, parentheses and slashes.
read -r -d '' EVENTS_PY <<'PY' || true
import datetime, json, os, re, sys

STATE = "target/agent-run"
STATUS = os.path.join(STATE, "status.json")
EVENTS = os.path.join(STATE, "events.jsonl")
RUN_LOG = os.path.join(STATE, "run.log")

RUNNING = re.compile(r"^\s*Running .*\((.+)\)\s*$")
DOC_TESTS = re.compile(r"^\s*Doc-tests (\S+)\s*$")
TEST = re.compile(r"^test (.+) \.\.\. (ok|FAILED|ignored)(?:,.*)?$")
OUTCOMES = {"ok": "ok", "FAILED": "failed", "ignored": "ignored"}


def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def emit(event, append=True):
    line = json.dumps(event, ensure_ascii=False)
    print(line, flush=True)
    if append:
        with open(EVENTS, "a", encoding="utf-8", newline="\n") as f:
            f.write(line + "\n")


def load_status():
    try:
        with open(STATUS, encoding="utf-8") as f:
            return json.load(f)
    except (OSError, ValueError):
        return None


def save_status(status):
    with open(STATUS, "w", encoding="utf-8", newline="\n") as f:
        json.dump(status, f)


def run_state(selection, state, passed=0, failed=0, ignored=0):
    return {"selection": selection, "state": state, "passed": passed, "failed": failed, "ignored": ignored}


def is_booted(status):
    return bool(status and status.get("booted"))


def target_stem(binary):
    name = re.split(r"[\\/]", binary)[-1]
    if name.endswith(".exe"):
        name = name[:-4]
    return re.sub(r"-[0-9a-f]+$", "", name)


def parse_tests(log):
    # Captured output is printed under `failures:`; nothing there is read, so no test's own text reaches an event.
    tests, target, in_failures = [], None, False
    for raw in log.splitlines():
        line = raw.rstrip("\r")
        running, doc = RUNNING.match(line), DOC_TESTS.match(line)
        if running:
            target, in_failures = target_stem(running.group(1)), False
        elif doc:
            target, in_failures = "doc:" + doc.group(1), False
        elif line.strip() == "failures:":
            in_failures = True
        elif line.startswith("test result:"):
            in_failures = False
        elif not in_failures:
            test = TEST.match(line)
            if test:
                tests.append((target, test.group(1), OUTCOMES[test.group(2)]))
    return tests


def main(verb, *args):
    if verb == "boot":
        cargo_exit, head = int(args[0]), args[1] or None
        booted = cargo_exit == 0
        ts = now()
        save_status({"booted": booted, "boot_ts": ts, "head": head, "run": run_state(None, "none")})
        emit({"event": "boot", "ts": ts, "outcome": "ready" if booted else "failed", "cargo_exit": cargo_exit,
              "head": head})
        return 0 if booted else 1
    if verb == "booted":
        return 0 if is_booted(load_status()) else 3
    if verb == "status":
        status = load_status() or {}
        emit({"event": "status", "booted": is_booted(status), "boot_ts": status.get("boot_ts"),
              "head": status.get("head"), "run": status.get("run") or run_state(None, "none")}, append=False)
        return 0 if is_booted(status) else 3
    if verb == "run-start":
        selection, files = args[0], list(args[1:])
        status = load_status()
        status["run"] = run_state(selection, "running")
        save_status(status)
        emit({"event": "run.start", "ts": now(), "selection": selection, "files": files})
        return 0
    if verb == "run-end":
        selection, cargo_exit = args[0], int(args[1]) if args[1] else None
        with open(RUN_LOG, encoding="utf-8", errors="replace") as f:
            tests = parse_tests(f.read())
        for target, name, outcome in tests:
            emit({"event": "test", "file": target, "test": name, "outcome": outcome})
        counts = {o: sum(1 for t in tests if t[2] == o) for o in ("ok", "failed", "ignored")}
        passed = cargo_exit == 0 and counts["failed"] == 0 and len(tests) > 0
        status = load_status()
        status["run"] = run_state(selection, "passed" if passed else "failed", counts["ok"], counts["failed"],
                                  counts["ignored"])
        save_status(status)
        emit({"event": "run.end", "ts": now(), "selection": selection, "passed": counts["ok"],
              "failed": counts["failed"], "ignored": counts["ignored"], "cargo_exit": cargo_exit,
              "outcome": "passed" if passed else "failed"})
        return 0 if passed else 1
    if verb == "cleanup":
        emit({"event": "cleanup", "outcome": "done"}, append=False)
        return 0
    return 2


sys.exit(main(*sys.argv[1:]))
PY

py() {
    python3 -c "$EVENTS_PY" "$@"
}

is_verb() {
    local v
    for v in "${VERBS[@]}"; do
        [[ "$v" == "$1" ]] && return 0
    done
    return 1
}

[[ $# -ge 1 ]] && is_verb "$1" || usage
verb="$1"

case "$verb" in
boot)
    [[ $# -eq 1 ]] || usage
    ensure_fresh_artifacts
    rm -rf "$STATE_DIR"
    mkdir -p "$STATE_DIR"
    : >"$STATE_DIR/events.jsonl"
    cargo_exit=0
    cargo test -p blitz-tests --locked --no-run >"$STATE_DIR/run.log" 2>&1 || cargo_exit=$?
    head=$(git rev-parse HEAD 2>/dev/null) || head=""
    py boot "$cargo_exit" "$head"
    ;;
run)
    [[ $# -eq 2 ]] || usage
    selection="$2"
    files=()
    case "$selection" in
    stand)
        for f in "$TESTS_DIR"/stand_*.rs; do
            [[ -e "$f" ]] && files+=("$(basename "$f" .rs)")
        done
        ;;
    all) ;;
    *)
        [[ "$selection" =~ ^[a-z0-9_]+$ && -f "$TESTS_DIR/$selection.rs" ]] || usage
        files=("$selection")
        ;;
    esac
    if ! py booted; then
        echo "agent-run: not booted — run \`$0 boot\` first" >&2
        exit 3
    fi
    py run-start "$selection" ${files[@]+"${files[@]}"}
    test_args=()
    for f in ${files[@]+"${files[@]}"}; do
        test_args+=(--test "$f")
    done
    cargo_exit=""
    if [[ "$selection" == all || ${#files[@]} -gt 0 ]]; then
        cargo_exit=0
        cargo test -p blitz-tests --locked ${test_args[@]+"${test_args[@]}"} >"$STATE_DIR/run.log" 2>&1 ||
            cargo_exit=$?
    else
        : >"$STATE_DIR/run.log"
    fi
    py run-end "$selection" "$cargo_exit"
    ;;
status)
    [[ $# -eq 1 ]] || usage
    py status
    ;;
cleanup)
    [[ $# -eq 1 ]] || usage
    rm -rf "$STATE_DIR"
    py cleanup
    ;;
logs)
    [[ $# -eq 1 ]] || usage
    if [[ ! -f "$STATE_DIR/events.jsonl" ]]; then
        echo "agent-run: no events — run \`$0 boot\` first" >&2
        exit 3
    fi
    cat "$STATE_DIR/events.jsonl"
    ;;
esac
