#!/usr/bin/env python3
"""One live agent session through the escher MCP server: the operator pass's trial, fired once.

Run from the repository root after `cargo build --locked -p seven_guis --bin escher-session`. A fresh
`claude` client runs from a per-run temporary directory whose path names nothing, with one stdio MCP server,
`escher` — the built `escher-session mcp` — and no other tool, and is asked to operate the Counter to 3. The
client's transcript and stderr, the requests it sent and the server's stderr stay in target/mcp-trial/; nothing of
them is printed. Stdout is one line of figures. Exit 0 when the client ran and exited 0, 1 when it ran and did not,
3 when `claude` is not on PATH or the binary is not built.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile

STATE = os.path.join("target", "mcp-trial")
BINARY = os.path.join("target", "debug", "escher-session")
PROMPT = (
    "Using only your tools, start the counter app, operate it until its display reads 3, take a snapshot, "
    "stop the session, then reply with the display's final reading."
)
VERBS = ["snapshot", "click", "type", "press", "advance", "scroll", "start", "status", "stop"]
TOOLS = {"mcp__escher__" + verb for verb in VERBS}
READS_THREE = ' "3" id="counter-value" '


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


def blocks(record, kind):
    message = record.get("message")
    content = message.get("content") if isinstance(message, dict) else None
    return [b for b in content if isinstance(b, dict) and b.get("type") == kind] if isinstance(content, list) else []


def text_of(content):
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        return "".join(b.get("text", "") for b in content if isinstance(b, dict) and isinstance(b.get("text"), str))
    return ""


def isolated(transcript):
    init = next((r for r in transcript if r.get("type") == "system" and r.get("subtype") == "init"), None)
    if init is None:
        return False
    servers = init.get("mcp_servers")
    return (
        isinstance(init.get("tools"), list)
        and set(init["tools"]) == TOOLS
        and len(init["tools"]) == len(TOOLS)
        and isinstance(servers, list)
        and len(servers) == 1
        and isinstance(servers[0], dict)
        and servers[0].get("name") == "escher"
        and servers[0].get("status") == "connected"
    )


def end_state(transcript):
    snapshots = set()
    last = None
    for record in transcript:
        if record.get("type") == "assistant":
            for block in blocks(record, "tool_use"):
                if block.get("name") == "mcp__escher__snapshot":
                    snapshots.add(block.get("id"))
        if record.get("type") == "user":
            for block in blocks(record, "tool_result"):
                if block.get("tool_use_id") in snapshots:
                    last = text_of(block.get("content"))
    return "reached" if last is not None and READS_THREE in last else "unmet"


def spans(log):
    lines = []
    try:
        with open(log, encoding="utf-8", errors="replace") as f:
            lines = [line for line in f if ' span="tool_call" ' in line]
    except OSError:
        pass
    refused = sum(1 for line in lines if ' ending="refused" ' in line)
    errors = sum(1 for line in lines if ' ending="error" ' in line)
    return len(lines), refused, errors


def requested(requests):
    records = json_lines(requests)
    version = next(
        (
            r["params"].get("protocolVersion")
            for r in records
            if r.get("method") == "initialize" and isinstance(r.get("params"), dict)
        ),
        None,
    )
    discover = any(r.get("method") == "server/discover" for r in records)
    return (version if isinstance(version, str) else "none"), ("sent" if discover else "not sent")


def main():
    if shutil.which("claude") is None:
        print("live-trial: claude is not on PATH", file=sys.stderr)
        return 3
    if not os.path.isfile(BINARY):
        print("live-trial: the escher-session binary is not built", file=sys.stderr)
        return 3
    os.makedirs(STATE, exist_ok=True)
    state = os.path.abspath(STATE)
    requests = os.path.join(state, "requests.jsonl")
    server_log = os.path.join(state, "server.log")
    transcript_path = os.path.join(state, "transcript.jsonl")
    client_log = os.path.join(state, "client.log")
    for name in (requests, server_log, transcript_path, client_log):
        if os.path.exists(name):
            os.remove(name)

    session = tempfile.mkdtemp()
    try:
        # The server: its stdin copied to the requests file on its way in, its stderr to the server log.
        server = 'tee "$1" | exec env RUST_LOG=info "$2" mcp 2>"$3"'
        config = {
            "mcpServers": {
                "escher": {
                    "type": "stdio",
                    "command": "sh",
                    "args": ["-c", server, "sh", requests, os.path.abspath(BINARY), server_log],
                }
            }
        }
        config_path = os.path.join(session, "mcp.json")
        with open(config_path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(config, f)
        command = [
            "timeout", "600", "claude", "-p", PROMPT, "--output-format", "stream-json", "--verbose",
            "--tools", "", "--strict-mcp-config", "--mcp-config", config_path, "--allowedTools", "mcp__escher",
            "--setting-sources", "", "--disable-slash-commands", "--no-session-persistence",
            "--max-budget-usd", "1",
        ]
        with open(transcript_path, "w", encoding="utf-8") as out, open(client_log, "w", encoding="utf-8") as err:
            client_exit = subprocess.run(command, cwd=session, stdin=subprocess.DEVNULL, stdout=out, stderr=err).returncode
    finally:
        shutil.rmtree(session, ignore_errors=True)

    transcript = json_lines(transcript_path)
    tool_calls, refused, errors = spans(server_log)
    version, discover = requested(requests)
    print(
        f"trial: client_exit {client_exit} · isolated {'true' if isolated(transcript) else 'false'} · "
        f"tool_calls {tool_calls} · refused {refused} · errors {errors} · end_state {end_state(transcript)} · "
        f"protocol_version {version} · discover {discover}"
    )
    return 0 if client_exit == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
