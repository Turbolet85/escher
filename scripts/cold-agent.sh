#!/usr/bin/env bash
# The cold-agent run pipe: starts a fresh agent session whose only tools are the driver's own MCP server (the built
# `escher-session mcp`), gives it the stand's four lean tasks, captures the session as a stream-json transcript,
# counts its wrong calls from the server's own tool_call span lines, reads completion from the tool's own snapshot
# answers, replays the check the agent wrote on a fresh server and writes a verdict. Four verbs, each printing JSON
# lines and exiting 0 success · 1 the verb ran and failed · 2 usage · 3 a precondition is unmet. Run it from the
# repository root after `cargo build --locked -p seven_guis --bin escher-session`. State lives in target/cold-agent/
# only: verdict.json, events.jsonl, transcript.jsonl, client.log, server.log and check.json. The session runs from a
# per-run mktemp -d directory whose path names nothing, removed after the run. scripts/cold-agent.ps1 delegates here
# on Windows.
set -euo pipefail

VERBS=(run status cleanup logs)
TASKS=(stand)
STATE_DIR=target/cold-agent
BINARY=target/debug/escher-session
read -r -d '' STAND_PROMPT <<'PROMPT' || true
Use only your tools. The stand has four apps: counter, flight booker, timer and CRUD. Work on one app at a time: start it, reach its goal, take a snapshot of its final screen, then stop it before you start the next app.

The goals:
1. Counter: the display reads 3.
2. Flight booker: a return flight is booked, departing 24.12.2026 and returning 31.12.2026.
3. Timer: the duration is set to 10 seconds, and time is passed until the elapsed time reads 10.0s.
4. CRUD: the person Ada Lovelace is added, Roman Tisch's surname is changed to Tischler, and Hans Emil is deleted.

Finally, reply with a check as the LAST thing in your reply: one fenced ```json block holding a JSON array of steps, each {"tool": <a tool name without its server prefix>, "arguments": {...}, "expect": [texts]} ("arguments" and "expect" optional). Run in order on a fresh server, the steps must redo all four tasks from fresh sessions, starting and stopping each app in turn; each app's part ends with a snapshot step whose "expect" lists texts from that screen which prove the task is done, before that app is stopped.
PROMPT

usage() {
    echo "usage: $0 <verb> [task]" >&2
    echo "verbs: ${VERBS[*]}" >&2
    echo "run tasks: ${TASKS[*]}" >&2
    exit 2
}

# Every event and the verdict are encoded by python's json module. No transcript text, tool argument, tool result,
# final reply or check step is copied into either: the verdict carries counts, booleans and closed words only.
read -r -d '' EVENTS_PY <<'PY' || true
import datetime, json, os, queue, re, subprocess, sys, threading

STATE = "target/cold-agent"
EVENTS = os.path.join(STATE, "events.jsonl")
VERDICT = os.path.join(STATE, "verdict.json")
TRANSCRIPT = os.path.join(STATE, "transcript.jsonl")
SERVER_LOG = os.path.join(STATE, "server.log")
CHECK = os.path.join(STATE, "check.json")
PREFIX = "mcp__escher__"
VERBS = ("snapshot", "click", "type", "press", "advance", "scroll", "screenshot", "lint", "start", "status", "stop", "help")
TOOLS = {PREFIX + verb for verb in VERBS}
ACTING = {"click", "type", "press", "advance", "scroll"}
SPACER = "task-header-spacer"
# Each app's done state as its snapshot text reads it: every `contains` text present, no `lacks` text.
DONE = {
    "counter": (['Paragraph "3" id="counter-value"'], []),
    "flight-booker": (['Paragraph "Booked return flight: 24.12.2026 → 31.12.2026" id="flight-booked"'], []),
    "timer": (['Paragraph "Elapsed: 10.0s" id="timer-elapsed"', 'Unknown "10.0s" id="timer-duration-value"'], []),
    "crud": (['ListBoxOption "Lovelace, Ada"', 'ListBoxOption "Tischler, Roman"'], ['id="crud-person-0"']),
}
APPS = tuple(DONE)
MAX_CHECK_BYTES = 65536
MAX_CHECK_STEPS = 400
REPLY_BOUND = 30
FENCE = re.compile(r"```json[ \t]*\r?\n(.*?)```", re.DOTALL)


def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def emit(event, append=True):
    line = json.dumps(event, ensure_ascii=False)
    print(line, flush=True)
    if append:
        with open(EVENTS, "a", encoding="utf-8", newline="\n") as f:
            f.write(line + "\n")


def json_lines(name):
    records = []
    try:
        with open(name, encoding="utf-8", errors="replace") as f:
            for line in f:
                try:
                    record = json.loads(line)
                except ValueError:
                    continue
                if isinstance(record, dict):
                    records.append(record)
    except OSError:
        pass
    return records


def load_json(name):
    try:
        with open(name, encoding="utf-8") as f:
            return json.load(f)
    except (OSError, ValueError):
        return None


def blocks(record, kind):
    message = record.get("message")
    content = message.get("content") if isinstance(message, dict) else None
    return [b for b in content if isinstance(b, dict) and b.get("type") == kind] if isinstance(content, list) else []


def text_of(content):
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        return "".join(b["text"] for b in content if isinstance(b, dict) and isinstance(b.get("text"), str))
    return ""


def strings(value):
    if isinstance(value, str):
        return [value]
    if isinstance(value, dict):
        return [s for v in value.values() for s in strings(v)]
    if isinstance(value, list):
        return [s for v in value for s in strings(v)]
    return []


def isolated(init, root_name):
    if init is None:
        return False
    servers = init.get("mcp_servers")
    named = root_name.casefold()
    return (
        isinstance(init.get("tools"), list)
        and set(init["tools"]) == TOOLS
        and len(init["tools"]) == len(TOOLS)
        and isinstance(servers, list)
        and len(servers) == 1
        and isinstance(servers[0], dict)
        and servers[0].get("name") == "escher"
        and servers[0].get("status") == "connected"
        and init.get("skills") == []
        and init.get("slash_commands") == []
        and init.get("apiKeySource") == "none"
        and bool(named)
        and not any(named in s.casefold() for s in strings(init.get("cwd")) + strings(init.get("memory_paths")))
    )


def span_counts():
    lines = []
    try:
        with open(SERVER_LOG, encoding="utf-8", errors="replace") as f:
            lines = [line for line in f if ' span="tool_call" ' in line]
    except OSError:
        pass
    refused = sum(1 for line in lines if ' ending="refused" ' in line)
    errors = sum(1 for line in lines if ' ending="error" ' in line)
    return len(lines), refused, errors


def done(app, text):
    contains, lacks = DONE[app]
    return all(t in text for t in contains) and not any(t in text for t in lacks)


def tasks_met(transcript, uses):
    started = None
    last = {}
    for record in transcript:
        if record.get("type") != "user":
            continue
        for block in blocks(record, "tool_result"):
            name, arguments = uses.get(block.get("tool_use_id"), (None, {}))
            accepted = block.get("is_error") is not True
            if name == PREFIX + "start" and accepted:
                app = arguments.get("app")
                started = app if isinstance(app, str) and app in DONE else None
            elif name == PREFIX + "stop" and accepted:
                started = None
            elif name == PREFIX + "snapshot" and started is not None:
                last[started] = text_of(block.get("content"))
    return [app for app in APPS if app in last and done(app, last[app])]


def extract_check(result):
    reply = (result or {}).get("result")
    found = FENCE.findall(reply) if isinstance(reply, str) else []
    if not found or len(found[-1].encode("utf-8")) > MAX_CHECK_BYTES:
        return None
    try:
        check = json.loads(found[-1])
    except ValueError:
        return None
    return check if isinstance(check, list) else None


def valid_step(step):
    return (
        isinstance(step, dict)
        and set(step) <= {"tool", "arguments", "expect"}
        and step.get("tool") in VERBS
        and isinstance(step.get("arguments", {}), dict)
        and isinstance(step.get("expect", []), list)
        and all(isinstance(t, str) for t in step.get("expect", []))
    )


class Server:
    """The built server as a stdio child, every reply read under REPLY_BOUND; closed by kill and reap."""

    def __init__(self, binary):
        self.proc = subprocess.Popen([binary, "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     stderr=subprocess.DEVNULL, text=True, encoding="utf-8", errors="replace")
        self.lines = queue.Queue()
        threading.Thread(target=self.pump, daemon=True).start()
        self.next_id = 0

    def pump(self):
        for line in self.proc.stdout:
            self.lines.put(line)
        self.lines.put(None)

    def send(self, frame):
        self.proc.stdin.write(json.dumps(frame) + "\n")
        self.proc.stdin.flush()

    def rpc(self, method, params):
        self.next_id += 1
        self.send({"jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params})
        while True:
            line = self.lines.get(timeout=REPLY_BOUND)
            if line is None:
                raise EOFError("the server closed its output")
            try:
                frame = json.loads(line)
            except ValueError:
                continue
            if isinstance(frame, dict) and frame.get("id") == self.next_id:
                return frame.get("result")

    def close(self):
        try:
            self.proc.stdin.close()
        except OSError:
            pass
        self.proc.kill()
        self.proc.wait()


def replay(binary, steps):
    """Each step's (isError, answer text) on a fresh server, or None when the server failed to answer."""
    server = Server(binary)
    try:
        server.rpc("initialize", {"protocolVersion": "2025-06-18", "capabilities": {},
                                  "clientInfo": {"name": "cold-agent-replay", "version": "1"}})
        server.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
        answers = []
        for step in steps:
            result = server.rpc("tools/call", {"name": step["tool"], "arguments": step.get("arguments", {})})
            if not isinstance(result, dict):
                return None
            answers.append((result.get("isError") is not False, text_of(result.get("content"))))
        return answers
    except (OSError, ValueError, EOFError, queue.Empty):
        return None
    finally:
        server.close()


def started_apps(check):
    started, apps = None, []
    for step in check:
        if step["tool"] == "start":
            app = step.get("arguments", {}).get("app")
            started = app if isinstance(app, str) and app in DONE else None
        elif step["tool"] == "stop":
            started = None
        apps.append(started)
    return apps


def judge_check(check, binary):
    """(check_passed, check_can_fail, replayed) for a parsed check."""
    if len(check) > MAX_CHECK_STEPS or not all(valid_step(step) for step in check):
        return False, False, False
    answers = replay(binary, check)
    apps = started_apps(check)
    passed = (
        answers is not None
        and all(not is_error and all(t in answer for t in step.get("expect", []))
                for step, (is_error, answer) in zip(check, answers))
        and any(step["tool"] in ACTING for step in check)
        and any(step.get("expect") for step in check)
        and set(APPS) <= {app for app in apps if app is not None}
    )
    kept = [i for i, step in enumerate(check) if step["tool"] not in ACTING]
    ablated = replay(binary, [check[i] for i in kept])
    can_fail = False
    if ablated is not None:
        answer_of = dict(zip(kept, (answer for _, answer in ablated)))
        can_fail = all(
            any(t not in answer_of[i] for i, step in enumerate(check)
                if apps[i] == app and step["tool"] == "snapshot" for t in step.get("expect", []))
            for app in APPS
        )
    return bool(passed), can_fail, True


def verdict(task, client_exit, root_name, binary):
    transcript = json_lines(TRANSCRIPT)
    init = next((r for r in transcript if r.get("type") == "system" and r.get("subtype") == "init"), None)
    result = next((r for r in reversed(transcript) if r.get("type") == "result"), None)
    uses = {}
    for record in transcript:
        if record.get("type") == "assistant":
            for block in blocks(record, "tool_use"):
                arguments = block.get("input")
                uses[block.get("id")] = (block.get("name"), arguments if isinstance(arguments, dict) else {})
    results = {}
    for record in transcript:
        if record.get("type") == "user":
            for block in blocks(record, "tool_result"):
                results[block.get("tool_use_id")] = block.get("is_error") is True
    server_calls, refused, errors = span_counts()
    escher_uses = [i for i, (name, _) in uses.items() if isinstance(name, str) and name.startswith(PREFIX)]
    escher_errors = sum(1 for i in escher_uses if results.get(i))
    foreign = len(uses) - len(escher_uses)
    denials = result.get("permission_denials") if result else None
    denials = denials if isinstance(denials, list) else []
    check = extract_check(result)
    if check is not None:
        with open(CHECK, "w", encoding="utf-8", newline="\n") as f:
            json.dump(check, f, indent=2, ensure_ascii=False)
            f.write("\n")
        check_passed, check_can_fail, replayed = judge_check(check, binary)
    else:
        check_passed, check_can_fail, replayed = False, False, False
    check_steps = check if check is not None else []
    usage = (result or {}).get("usage") or {}
    succeeded = result is not None and result.get("subtype") == "success" and result.get("is_error") is False
    met = tasks_met(transcript, uses)
    v = {
        "task": task,
        "client_exit": client_exit,
        "isolated": isolated(init, root_name),
        "session_tools": init.get("tools") if init else None,
        "client_version": init.get("claude_code_version") if init else None,
        "model": init.get("model") if init else None,
        "tool_calls": len(uses),
        "server_tool_calls": server_calls,
        "refused_calls": refused,
        "error_calls": errors,
        "wrong_calls": refused + errors + foreign + len(denials),
        "transcript_errors": sum(1 for e in results.values() if e),
        "counts_agree": server_calls == len(escher_uses) and refused + errors == escher_errors,
        "spacer_calls": sum(1 for i in escher_uses if uses[i][1].get("id") == SPACER),
        "tasks_met": met,
        "check_steps": len(check_steps),
        "check_expects": sum(len(s["expect"]) for s in check_steps
                             if isinstance(s, dict) and isinstance(s.get("expect"), list)),
        "check_passed": check_passed,
        "check_can_fail": check_can_fail,
        "num_turns": (result or {}).get("num_turns"),
        "duration_ms": (result or {}).get("duration_ms"),
        "cost_usd": (result or {}).get("total_cost_usd"),
        "input_tokens": usage.get("input_tokens"),
        "output_tokens": usage.get("output_tokens"),
        "cache_read_input_tokens": usage.get("cache_read_input_tokens"),
        "cache_creation_input_tokens": usage.get("cache_creation_input_tokens"),
        "transcript_file": TRANSCRIPT,
    }
    reasons = []
    if client_exit != 0:
        reasons.append("client-exit")
    if not succeeded:
        reasons.append("no-result")
    if not v["isolated"]:
        reasons.append("not-isolated")
    if not v["counts_agree"]:
        reasons.append("counts-disagree")
    if v["tool_calls"] < 1:
        reasons.append("no-tool-call")
    if len(met) != len(APPS):
        reasons.append("task-unmet")
    if check is None:
        reasons.append("no-check")
    else:
        if not check_passed:
            reasons.append("check-failed")
        if replayed and not check_can_fail:
            reasons.append("check-cannot-fail")
    v["outcome"] = "failed" if reasons else "passed"
    v["reasons"] = reasons
    return v


def main(verb, *args):
    if verb == "config":
        session, binary, server_log = args
        server = 'exec env RUST_LOG=info "$1" mcp 2>"$2"'
        config = {"mcpServers": {"escher": {"type": "stdio", "command": "sh",
                                            "args": ["-c", server, "sh", binary, server_log]}}}
        with open(os.path.join(session, "mcp.json"), "w", encoding="utf-8", newline="\n") as f:
            json.dump(config, f)
        return 0
    if verb == "run-start":
        emit({"event": "run.start", "ts": now(), "task": args[0]})
        return 0
    if verb == "run-end":
        task, client_exit, root_name, binary = args[0], int(args[1]), args[2], args[3]
        v = verdict(task, client_exit, root_name, binary)
        v["ts"] = now()
        with open(VERDICT, "w", encoding="utf-8", newline="\n") as f:
            json.dump(v, f, indent=2)
            f.write("\n")
        emit(dict({"event": "run.end"}, **v))
        return 0 if v["outcome"] == "passed" else 1
    if verb == "status":
        v = load_json(VERDICT)
        if not isinstance(v, dict):
            emit({"event": "status", "outcome": "none"}, append=False)
            return 3
        emit(dict({"event": "status"}, **v), append=False)
        return 0
    if verb == "cleanup":
        emit({"event": "cleanup", "outcome": "done"}, append=False)
        return 0
    return 2


sys.exit(main(*sys.argv[1:]))
PY

py() {
    python3 -c "$EVENTS_PY" "$@"
}

contains() {
    local item="$1" v
    shift
    for v in "$@"; do
        [[ "$v" == "$item" ]] && return 0
    done
    return 1
}

[[ $# -ge 1 ]] && contains "$1" "${VERBS[@]}" || usage
verb="$1"

case "$verb" in
run)
    [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
    task="$2"
    for tool in claude python3; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            echo "cold-agent: $tool is not on PATH" >&2
            exit 3
        fi
    done
    if [[ ! -f "$BINARY" || ! -x "$BINARY" ]]; then
        echo "cold-agent: $BINARY is not built — run \`cargo build --locked -p seven_guis --bin escher-session\` first" >&2
        exit 3
    fi
    root_name=$(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")
    rm -rf "$STATE_DIR"
    mkdir -p "$STATE_DIR"
    : >"$STATE_DIR/events.jsonl"
    state="$PWD/$STATE_DIR"
    binary="$PWD/$BINARY"
    session=$(mktemp -d)
    trap 'rm -rf "$session"' EXIT
    py config "$session" "$binary" "$state/server.log"
    py run-start "$task"
    client_exit=0
    (
        cd "$session"
        timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
            --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__escher --setting-sources "" \
            --disable-slash-commands --no-session-persistence --model claude-opus-5-5 --max-budget-usd 3
    ) </dev/null >"$state/transcript.jsonl" 2>"$state/client.log" || client_exit=$?
    rm -rf "$session"
    py run-end "$task" "$client_exit" "$root_name" "$binary"
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
        echo "cold-agent: no events — run \`$0 run stand\` first" >&2
        exit 3
    fi
    cat "$STATE_DIR/events.jsonl"
    ;;
esac
