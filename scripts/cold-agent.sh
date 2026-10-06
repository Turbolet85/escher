#!/usr/bin/env bash
# The cold-agent run pipe: starts a fresh agent session whose only tools are the stub's
# (scripts/cold_agent_stub.py), gives it one task, captures the session as a stream-json transcript, counts its
# wrong calls from the stub's own call log and writes a verdict. Four verbs, each printing JSON lines and exiting
# 0 success · 1 the verb ran and failed · 2 usage · 3 a precondition is unmet. Run it from the repository root.
# State lives in target/cold-agent/ only: verdict.json, events.jsonl, transcript.jsonl, client.log, calls.jsonl and
# stub-state.json. The session runs from a per-run mktemp -d directory whose path names nothing, removed after the
# run. scripts/cold-agent.ps1 delegates here on Windows.
set -euo pipefail

VERBS=(run status cleanup logs)
TASKS=(counter)
STATE_DIR=target/cold-agent
STUB="$(dirname "${BASH_SOURCE[0]}")/cold_agent_stub.py"
COUNTER_PROMPT="Operate the counter through your tools until its display reads 3, then reply with the display's final reading."

usage() {
    echo "usage: $0 <verb> [task]" >&2
    echo "verbs: ${VERBS[*]}" >&2
    echo "run tasks: ${TASKS[*]}" >&2
    exit 2
}

# Every event and the verdict are encoded by python's json module. No transcript text, tool argument, tool result
# or final reply is copied into either: the verdict carries counts and identities only.
read -r -d '' EVENTS_PY <<'PY' || true
import datetime, json, os, sys

STATE = "target/cold-agent"
EVENTS = os.path.join(STATE, "events.jsonl")
VERDICT = os.path.join(STATE, "verdict.json")
TRANSCRIPT = os.path.join(STATE, "transcript.jsonl")
CALLS = os.path.join(STATE, "calls.jsonl")
STUB_STATE = os.path.join(STATE, "stub-state.json")
STUB_PREFIX = "mcp__stub__"
STUB_TOOLS = {STUB_PREFIX + t for t in ("list", "read", "press")}
TARGET_COUNT = 3


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
        and set(init["tools"]) == STUB_TOOLS
        and len(init["tools"]) == len(STUB_TOOLS)
        and isinstance(servers, list)
        and len(servers) == 1
        and isinstance(servers[0], dict)
        and servers[0].get("name") == "stub"
        and servers[0].get("status") == "connected"
        and init.get("skills") == []
        and init.get("slash_commands") == []
        and init.get("apiKeySource") == "none"
        and bool(named)
        and not any(named in s.casefold() for s in strings(init.get("cwd")) + strings(init.get("memory_paths")))
    )


def verdict(task, client_exit, root_name):
    transcript = json_lines(TRANSCRIPT)
    init = next((r for r in transcript if r.get("type") == "system" and r.get("subtype") == "init"), None)
    result = next((r for r in reversed(transcript) if r.get("type") == "result"), None)
    uses = {}
    for record in transcript:
        if record.get("type") == "assistant":
            for block in blocks(record, "tool_use"):
                uses[block.get("id")] = block.get("name")
    results = {}
    for record in transcript:
        if record.get("type") == "user":
            for block in blocks(record, "tool_result"):
                results[block.get("tool_use_id")] = block.get("is_error") is True
    calls = json_lines(CALLS)
    refused = sum(1 for c in calls if c.get("outcome") == "refused")
    stub_uses = [i for i, name in uses.items() if isinstance(name, str) and name.startswith(STUB_PREFIX)]
    stub_errors = sum(1 for i in stub_uses if results.get(i))
    denials = result.get("permission_denials") if result else None
    denials = denials if isinstance(denials, list) else []
    state = load_json(STUB_STATE)
    final_count = state.get("count") if isinstance(state, dict) else None
    usage = (result or {}).get("usage") or {}
    succeeded = result is not None and result.get("subtype") == "success" and result.get("is_error") is False
    v = {
        "task": task,
        "client_exit": client_exit,
        "isolated": isolated(init, root_name),
        "session_tools": init.get("tools") if init else None,
        "client_version": init.get("claude_code_version") if init else None,
        "model": init.get("model") if init else None,
        "tool_calls": len(uses),
        "wrong_calls": refused + sum(1 for n in uses.values() if not (isinstance(n, str) and n.startswith(STUB_PREFIX)))
        + len(denials),
        "transcript_errors": sum(1 for e in results.values() if e),
        "counts_agree": stub_errors == refused and len(stub_uses) == len(calls),
        "final_count": final_count,
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
    if final_count != TARGET_COUNT:
        reasons.append("task-unmet")
    v["outcome"] = "failed" if reasons else "passed"
    v["reasons"] = reasons
    return v


def main(verb, *args):
    if verb == "config":
        session = args[0]
        config = {"mcpServers": {"stub": {"type": "stdio", "command": "python3", "args": [
            os.path.join(session, "stub.py"),
            "--state", os.path.join(session, "stub-state.json"),
            "--calls", os.path.join(session, "calls.jsonl"),
        ]}}}
        with open(os.path.join(session, "mcp.json"), "w", encoding="utf-8", newline="\n") as f:
            json.dump(config, f)
        return 0
    if verb == "run-start":
        emit({"event": "run.start", "ts": now(), "task": args[0]})
        return 0
    if verb == "run-end":
        task, client_exit, root_name = args[0], int(args[1]), args[2]
        v = verdict(task, client_exit, root_name)
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
    root_name=$(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")
    rm -rf "$STATE_DIR"
    mkdir -p "$STATE_DIR"
    : >"$STATE_DIR/events.jsonl"
    state="$PWD/$STATE_DIR"
    session=$(mktemp -d)
    trap 'rm -rf "$session"' EXIT
    cp "$STUB" "$session/stub.py"
    py config "$session"
    py run-start "$task"
    client_exit=0
    (
        cd "$session"
        timeout 600 claude -p "$COUNTER_PROMPT" --output-format stream-json --verbose --tools "" \
            --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__stub --setting-sources "" \
            --disable-slash-commands --no-session-persistence --max-budget-usd 1
    ) </dev/null >"$state/transcript.jsonl" 2>"$state/client.log" || client_exit=$?
    for f in calls.jsonl stub-state.json; do
        if [[ -f "$session/$f" ]]; then
            cp "$session/$f" "$state/$f"
        fi
    done
    rm -rf "$session"
    py run-end "$task" "$client_exit" "$root_name"
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
        echo "cold-agent: no events — run \`$0 run counter\` first" >&2
        exit 3
    fi
    cat "$STATE_DIR/events.jsonl"
    ;;
esac
