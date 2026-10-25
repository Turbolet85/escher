#!/usr/bin/env python3
"""Contract of scripts/cold-agent.sh under a claude shim and a fake escher-session (no live model, no credential, no
built binary). Run with `python3 -m unittest discover -s .github/scripts`."""

import copy
import json
import os
import shutil
import stat
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PIPE = ROOT / "scripts" / "cold-agent.sh"

SCRUB_KEYS = {"url", "href", "src", "html", "text", "value", "attrs", "path", "request", "error", "panic.payload"}
MARKER = "cold-agent-reply-marker"
ARGUMENT_MARKER = "cold-agent-argument-marker"
ANSWER_MARKER = "cold-agent-answer-marker"
CHECK_MARKER = "cold-agent-check-marker"
VERBS = ["snapshot", "click", "type", "press", "advance", "scroll", "screenshot", "lint", "start", "status", "stop", "help"]
TOOLS = ["mcp__escher__" + verb for verb in VERBS]
APPS = ("counter", "flight-booker", "timer", "crud", "cells", "load")
STAND_IDS = ("counter-", "flight-", "timer-", "crud-", "cell-", "load-", "task-header", "id=")

HEADER = 'Header "" id="task-header" @0,0 800x46\n  GenericContainer "" id="task-header-spacer" @181,22.5 603x0\n'


def cells(b1, z25):
    """A Cells screen of eight lines: read in two parts, B1 in the first and Z25 in the second."""
    rows = [("A0", "5"), ("B1", b1), ("C2", ""), ("D3", ""), ("E4", ""), ("Z25", z25)]
    return HEADER + "".join(f'GenericContainer "{value}" id="cell-{cell}"\n' for cell, value in rows)


SCREENS = {
    "counter": [HEADER + f'Paragraph "{n}" id="counter-value" @314,309 60x28\n' for n in range(4)],
    "flight-booker": [
        HEADER + 'Paragraph "" id="flight-booked" @24,200 0x0\n',
        HEADER + 'Paragraph "Booked return flight: 24.12.2026 → 31.12.2026" id="flight-booked" @24,200 300x18\n',
    ],
    "timer": [
        HEADER + 'Paragraph "Elapsed: 0.0s" id="timer-elapsed"\nUnknown "15.0s" id="timer-duration-value"\n',
        HEADER + 'Paragraph "Elapsed: 10.0s" id="timer-elapsed"\nUnknown "10.0s" id="timer-duration-value"\n',
    ],
    "crud": [
        HEADER + 'ListBoxOption "Emil, Hans" id="crud-person-0"\nListBoxOption "Tisch, Roman" id="crud-person-2"\n',
        HEADER + 'ListBoxOption "Tischler, Roman" id="crud-person-2"\nListBoxOption "Lovelace, Ada" id="crud-person-3"\n',
    ],
    "cells": [cells("", ""), cells("10", "11")],
    "load": [
        HEADER + 'Button "Load" id="load-fetch"\n',
        HEADER + 'Button "Load" id="load-fetch"\nParagraph "Loaded" id="load-done"\n',
    ],
}
# The apps whose screen reads in parts, with the count of parts.
PAGES = {"cells": 2}
ACTS = {
    "counter": [("click", {"id": "counter-increment"})] * 3,
    "flight-booker": [("click", {"id": "flight-book"})],
    "timer": [("advance", {"ms": 10000})],
    "crud": [("click", {"id": "crud-create"})],
    "cells": [("type", {"id": "cell-input", "text": "=B1+1"})],
    "load": [("click", {"id": "load-fetch"})],
}
# The calls each app's flow makes after its final snapshot: the capabilities the verdict counts.
EXTRA = {
    "flight-booker": [("screenshot", {})],
    "cells": [("snapshot", {"part": 2}), ("lint", {})],
}
EXPECT = {
    "counter": ['"3" id="counter-value"'],
    "flight-booker": ["Booked return flight: 24.12.2026"],
    "timer": ["Elapsed: 10.0s", '"10.0s" id="timer-duration-value"'],
    "crud": ["Lovelace, Ada", "Tischler, Roman"],
    "cells": ['"10" id="cell-B1"'],
    "load": ['"Loaded" id="load-done"'],
}
SPACER_CLICK = {"tool": "mcp__escher__click", "arguments": {"id": "task-header-spacer"}}
LEFT = {"tool": "mcp__escher__press", "arguments": {"key": "arrowleft"}}
STOP = {"tool": "mcp__escher__stop"}
HELP = {"tool": "mcp__escher__help"}

# The fake stands in for the built `escher-session`: its `mcp` role answers the twelve tools, holds one app at a
# time, answers a snapshot with the scenario's screen for that app indexed by its count of accepted acting calls — a
# paged app's as the JSON line of the part asked, its lines cut into the scenario's count of parts — writes one
# tool_call span line per tools/call on stderr when RUST_LOG=info, as escher's sink does, and under `--record <dir>`
# keeps one check per accepted start in that directory, opening with the start's record.
FAKE = """\
#!/usr/bin/env python3
import json, os, sys

with open(__SCENARIO__, encoding="utf-8") as f:
    scenario = json.load(f)
with open(__SPAWNS__, "a", encoding="utf-8") as f:
    f.write(" ".join(sys.argv[1:]) + "\\n")
VERBS = __VERBS__
ACTING = {"click", "type", "press", "advance", "scroll"}
screens = scenario["screens"]
pages = scenario.get("pages", {})
refuse = scenario["refuse"]
record = sys.argv[sys.argv.index("--record") + 1] if "--record" in sys.argv else None
spans = os.environ.get("RUST_LOG") == "info"
drop = scenario.get("drop_span", False)
started, acts = None, 0


def call(name, arguments):
    global started, acts
    if name == "start":
        if arguments.get("app") not in screens:
            return "refused", '{"refused":{"cause":"malformed"}}'
        if started is not None:
            return "error", '{"error":{"kind":"already-running"}}'
        started, acts = arguments["app"], 0
        if record is not None and not scenario.get("no_record"):
            kept = len(os.listdir(record)) + 1
            with open(os.path.join(record, "check-%d.json" % kept), "w", encoding="utf-8") as f:
                json.dump([{"tool": "start", "arguments": {"app": scenario.get("record_as", started)}}], f)
        return "accepted", json.dumps({"label": started})
    if name in ("help", "status"):
        return "accepted", "{}"
    if name not in VERBS:
        return "refused", '{"refused":{"cause":"malformed"}}'
    if started is None:
        return "error", '{"error":{"kind":"no-session"}}'
    if name == "stop":
        started = None
        return "accepted", '{"stopped":true}'
    if name in ("snapshot", "screenshot"):
        screen = screens[started][min(acts, len(screens[started]) - 1)]
        if name == "screenshot" or started not in pages:
            return "accepted", screen
        lines, parts = screen.splitlines(keepends=True), pages[started]
        size = -(-len(lines) // parts)
        part = arguments.get("part", 1)
        text = "".join(lines[(part - 1) * size:part * size])
        return "accepted", json.dumps({"text": text, "part": part, "parts": parts})
    if name == "lint":
        return "accepted", '{"findings":[]}'
    if any(value in refuse for value in arguments.values()):
        return "refused", '{"refused":{"cause":"covered"}}'
    acts += 1
    return "accepted", '{"settled":true}'


for line in sys.stdin:
    frame = json.loads(line)
    method = frame.get("method")
    if method == "initialize":
        result = {"protocolVersion": "2025-06-18", "capabilities": {"tools": {}}, "serverInfo": {"name": "escher"}}
    elif method == "tools/list":
        result = {"tools": [{"name": verb, "inputSchema": {"type": "object"}} for verb in VERBS]}
    elif method == "tools/call":
        params = frame["params"]
        ending, text = call(params["name"], params.get("arguments") or {})
        if drop:
            drop = False
        elif spans:
            sys.stderr.write('2026-10-15T00:00:00.000000Z INFO escher_driver service.name=seven_guis span="tool_call" '
                             'tool="%s" ending="%s" message="close"\\n' % (params["name"], ending))
            sys.stderr.flush()
        result = {"content": [{"type": "text", "text": text}], "isError": ending != "accepted"}
    else:
        continue
    sys.stdout.write(json.dumps({"jsonrpc": "2.0", "id": frame["id"], "result": result}) + "\\n")
    sys.stdout.flush()
"""

# The shim stands in for `claude -p`: it records its argv, cwd and MCP config, spawns the configured server exactly
# as mcp.json names it, drives it as a minimal MCP client through the scenario's calls, and prints a stream-json
# transcript built from what the server returned, its reply ending with the scenario's check block.
SHIM = """\
#!/usr/bin/env python3
import json, os, subprocess, sys

args = sys.argv[1:]
with open(args[args.index("--mcp-config") + 1], encoding="utf-8") as f:
    config = json.load(f)
with open(__RECORD__, "w", encoding="utf-8") as f:
    json.dump({"argv": args, "cwd": os.getcwd(), "mcp": config}, f)
with open(__SCENARIO__, encoding="utf-8") as f:
    scenario = json.load(f)
server = config["mcpServers"]["escher"]
proc = subprocess.Popen([server["command"], *server["args"]], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)


def send(frame):
    proc.stdin.write(json.dumps(frame) + "\\n")
    proc.stdin.flush()


def rpc(n, method, params):
    send({"jsonrpc": "2.0", "id": n, "method": method, "params": params})
    return json.loads(proc.stdout.readline())["result"]


rpc(1, "initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "shim"}})
send({"jsonrpc": "2.0", "method": "notifications/initialized"})
tools = ["mcp__escher__" + t["name"] for t in rpc(2, "tools/list", {})["tools"]]
cwd = os.getcwd()
init = {"type": "system", "subtype": "init", "cwd": cwd, "tools": tools + scenario.get("extra_tools", []),
        "mcp_servers": [{"name": "escher", "status": "connected"}] + scenario.get("extra_servers", []),
        "model": "shim-model", "slash_commands": [], "skills": [], "apiKeySource": "none",
        "claude_code_version": "0.0.0-shim",
        "memory_paths": {"auto": "/home/shim/.claude/projects/" + cwd.replace("/", "-") + "/memory/"}}
init.update(scenario.get("init", {}))
out = [init]
for n, call in enumerate(scenario["calls"]):
    use = "toolu_%d" % n
    name, arguments = call["tool"], call.get("arguments", {})
    out.append({"type": "assistant", "message": {"content": [
        {"type": "tool_use", "id": use, "name": name, "input": arguments}]}})
    if name.startswith("mcp__escher__"):
        result = rpc(10 + n, "tools/call", {"name": name[len("mcp__escher__"):], "arguments": arguments})
        content, is_error = result["content"], result["isError"]
    else:
        content, is_error = "No such tool available: " + name, True
    out.append({"type": "user", "message": {"content": [
        {"type": "tool_result", "tool_use_id": use, "content": content, "is_error": is_error}]}})
proc.stdin.close()
proc.wait()
reply = "final reply " + __MARKER__
if "check" in scenario:
    reply += "\\n\\n```json\\n" + json.dumps(scenario["check"], indent=1) + "\\n```\\n"
subtype = scenario.get("subtype", "success")
out.append({"type": "assistant", "message": {"content": [{"type": "text", "text": reply}]}})
out.append({"type": "result", "subtype": subtype, "is_error": subtype != "success", "num_turns": 4,
            "duration_ms": 1234, "total_cost_usd": 0.0123, "result": reply,
            "usage": {"input_tokens": 100, "output_tokens": 20, "cache_read_input_tokens": 300,
                      "cache_creation_input_tokens": 40},
            "permission_denials": scenario.get("denials", [])})
for record in out:
    print(json.dumps(record))
sys.exit(scenario.get("exit", 0))
"""


def flow(counter_clicks=3, snapshot_apps=APPS, acts=ACTS, extra=EXTRA):
    """The agent's calls: each app started, driven, snapshotted, its counted capabilities used and stopped in turn."""
    calls = []
    for app in APPS:
        driven = acts[app] if app != "counter" else acts[app][:1] * counter_clicks
        calls.append({"tool": "mcp__escher__start", "arguments": {"app": app}})
        calls += [{"tool": "mcp__escher__" + tool, "arguments": arguments} for tool, arguments in driven]
        if app in snapshot_apps:
            calls.append({"tool": "mcp__escher__snapshot"})
        calls += [{"tool": "mcp__escher__" + tool, "arguments": arguments} for tool, arguments in extra.get(app, [])]
        calls.append(STOP)
    return calls


def with_wrong(wrong, total):
    """The flow with `wrong` refused presses inside the counter's session, padded with help calls to `total` calls."""
    calls = flow()
    calls = calls[:1] + [LEFT] * wrong + calls[1:]
    return calls + [HELP] * (total - len(calls))


def stand_check(expect=None, apps=APPS):
    """The agent's check: the same flow without the server prefix, each snapshot step expecting its done texts."""
    expect = EXPECT if expect is None else expect
    steps = []
    for app in apps:
        steps.append({"tool": "start", "arguments": {"app": app}})
        steps += [{"tool": tool, "arguments": arguments} for tool, arguments in ACTS[app]]
        steps.append({"tool": "snapshot", "expect": expect[app]})
        steps.append({"tool": "stop"})
    return steps


class ColdAgentPipeTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        (self.root / "scripts").mkdir()
        shutil.copy(PIPE, self.root / "scripts" / PIPE.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.record = self.root / "claude-record.json"
        self.scenario_file = self.root / "scenario.json"
        self.spawns = self.root / "spawns.log"
        self.state = self.root / "target" / "cold-agent"
        self.server = self.root / "target" / "debug" / "escher-session"
        self.server.parent.mkdir(parents=True)
        self.server.write_text(
            FAKE.replace("__SCENARIO__", repr(str(self.scenario_file)))
            .replace("__SPAWNS__", repr(str(self.spawns)))
            .replace("__VERBS__", repr(VERBS)),
            encoding="utf-8",
        )
        self.server.chmod(self.server.stat().st_mode | stat.S_IXUSR)
        shim = self.bin / "claude"
        shim.write_text(
            SHIM.replace("__RECORD__", repr(str(self.record)))
            .replace("__SCENARIO__", repr(str(self.scenario_file)))
            .replace("__MARKER__", repr(MARKER)),
            encoding="utf-8",
        )
        shim.chmod(shim.stat().st_mode | stat.S_IXUSR)

    def tearDown(self):
        self._tmp.cleanup()

    def pipe(self, *args, path=None):
        path = path if path is not None else f"{self.bin}{os.pathsep}{os.environ['PATH']}"
        return subprocess.run(
            ["bash", str(self.root / "scripts" / PIPE.name), *args],
            cwd=self.root,
            env=dict(os.environ, PATH=path),
            capture_output=True,
            text=True,
        )

    def run_scenario(self, calls=None, **knobs):
        scenario = {"calls": flow() if calls is None else calls, "check": stand_check(), "screens": SCREENS,
                    "pages": PAGES, "refuse": ["task-header-spacer", "arrowleft"]}
        scenario.update(knobs)
        if scenario["check"] is None:
            del scenario["check"]
        self.scenario_file.write_text(json.dumps(scenario), encoding="utf-8")
        if self.spawns.exists():
            self.spawns.unlink()
        return self.pipe("run", "stand")

    def json_lines(self, text):
        return [json.loads(line) for line in text.splitlines()]

    def verdict(self):
        return json.loads((self.state / "verdict.json").read_text(encoding="utf-8"))

    def client_record(self):
        return json.loads(self.record.read_text(encoding="utf-8"))

    def live_spawn(self):
        """The live server's arguments as the fake records them: the `mcp` role keeping its sessions under record/."""
        return "mcp --record " + str((self.state / "record").resolve())

    def spawned(self):
        return self.spawns.read_text(encoding="utf-8").splitlines() if self.spawns.exists() else []

    def assert_failed_with(self, proc, reasons):
        self.assertEqual(proc.returncode, 1, proc.stderr)
        verdict = self.verdict()
        self.assertEqual((verdict["outcome"], verdict["reasons"]), ("failed", reasons))
        return verdict

    def assert_passed(self, proc):
        self.assertEqual(proc.returncode, 0, proc.stderr)
        verdict = self.verdict()
        self.assertEqual((verdict["outcome"], verdict["reasons"]), ("passed", []))
        return verdict

    def test_a_six_app_run_passes_with_no_wrong_call(self):
        proc = self.run_scenario()
        verdict = self.assert_passed(proc)
        start, end = self.json_lines(proc.stdout)
        self.assertEqual((start["event"], start["task"]), ("run.start", "stand"))
        self.assertEqual(end["event"], "run.end")
        calls = len(flow())
        self.assertEqual(
            {k: verdict[k] for k in ("isolated", "counts_agree", "tool_calls", "server_tool_calls", "refused_calls",
                                     "error_calls", "wrong_calls", "wrong_call_bar_pct", "spacer_calls", "lint_calls",
                                     "screenshot_calls", "advance_calls", "part_reads", "recorded_sessions",
                                     "recording_kept", "tasks_met", "check_steps", "check_expects", "check_passed",
                                     "check_can_fail")},
            {"isolated": True, "counts_agree": True, "tool_calls": calls, "server_tool_calls": calls,
             "refused_calls": 0, "error_calls": 0, "wrong_calls": 0, "wrong_call_bar_pct": 5, "spacer_calls": 0,
             "lint_calls": 1, "screenshot_calls": 1, "advance_calls": 1, "part_reads": 1,
             "recorded_sessions": len(APPS), "recording_kept": True, "tasks_met": list(APPS),
             "check_steps": len(stand_check()), "check_expects": sum(len(texts) for texts in EXPECT.values()),
             "check_passed": True, "check_can_fail": True},
        )
        self.assertEqual(sorted(verdict["session_tools"]), sorted(TOOLS))
        self.assertEqual((verdict["client_version"], verdict["model"]), ("0.0.0-shim", "shim-model"))
        self.assertEqual(
            [verdict[k] for k in ("input_tokens", "output_tokens", "cache_read_input_tokens",
                                  "cache_creation_input_tokens", "cost_usd", "num_turns", "duration_ms")],
            [100, 20, 300, 40, 0.0123, 4, 1234],
        )
        self.assertEqual(verdict["transcript_file"], "target/cold-agent/transcript.jsonl")
        self.assertNotIn("final_count", verdict)
        self.assertEqual({k: v for k, v in end.items() if k != "event"}, verdict)
        self.assertEqual(sorted(p.name for p in self.state.iterdir()),
                         ["check.json", "client.log", "events.jsonl", "record", "server.log", "transcript.jsonl",
                          "verdict.json"])
        self.assertEqual(json.loads((self.state / "check.json").read_text(encoding="utf-8")), stand_check())
        self.assertEqual(self.spawned(), [self.live_spawn(), "mcp", "mcp"])

    def test_wrong_calls_are_counted_from_the_span_lines_and_agree_with_the_transcript(self):
        calls = flow()
        proc = self.run_scenario([STOP, calls[0], LEFT, LEFT] + calls[1:])
        verdict = self.assert_failed_with(proc, ["wrong-calls-over-bar"])
        self.assertEqual(
            [verdict[k] for k in ("refused_calls", "error_calls", "wrong_calls", "transcript_errors", "counts_agree",
                                  "server_tool_calls")],
            [2, 1, 3, 3, True, len(calls) + 3],
        )

    def test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar(self):
        verdict = self.assert_passed(self.run_scenario(with_wrong(5, 100)))
        self.assertEqual((verdict["wrong_calls"], verdict["tool_calls"], verdict["counts_agree"]), (5, 100, True))
        verdict = self.assert_failed_with(self.run_scenario(with_wrong(6, 100)), ["wrong-calls-over-bar"])
        self.assertEqual((verdict["wrong_calls"], verdict["tool_calls"]), (6, 100))

    def test_a_tool_outside_the_escher_server_is_a_wrong_call(self):
        proc = self.run_scenario([{"tool": "Bash", "arguments": {"command": "ls"}}] + flow())
        verdict = self.assert_passed(proc)
        self.assertEqual(
            (verdict["wrong_calls"], verdict["transcript_errors"], verdict["counts_agree"], verdict["refused_calls"]),
            (1, 1, True, 0),
        )

    def test_a_permission_denial_is_a_wrong_call(self):
        proc = self.run_scenario(denials=[{"tool_name": "mcp__escher__click"}])
        self.assertEqual(self.assert_passed(proc)["wrong_calls"], 1)

    def test_a_call_on_the_header_spacer_is_counted(self):
        calls = flow()
        verdict = self.assert_passed(self.run_scenario(calls[:1] + [SPACER_CLICK] + calls[1:]))
        self.assertEqual((verdict["spacer_calls"], verdict["refused_calls"], verdict["wrong_calls"]), (1, 1, 1))

    def test_an_unmet_app_fails(self):
        verdict = self.assert_failed_with(self.run_scenario(flow(counter_clicks=2)), ["task-unmet"])
        self.assertEqual(verdict["tasks_met"], list(APPS[1:]))

    def test_an_app_without_a_final_snapshot_is_unmet(self):
        calls = flow(snapshot_apps=APPS[:3]) + [{"tool": "mcp__escher__snapshot"}]
        verdict = self.assert_failed_with(self.run_scenario(calls), ["task-unmet"])
        self.assertEqual((verdict["tasks_met"], verdict["error_calls"]), (list(APPS[:3]), 1))

    def test_a_paged_answer_is_read_as_its_parts_joined(self):
        verdict = self.assert_passed(self.run_scenario())
        self.assertIn("cells", verdict["tasks_met"])
        transcript = (self.state / "transcript.jsonl").read_text(encoding="utf-8")
        self.assertNotIn('"10" id="cell-B1"', transcript)
        self.assertIn(r'\\\"10\\\" id=\\\"cell-B1\\\"', transcript)
        extra = dict(EXTRA, cells=[("lint", {})])
        verdict = self.assert_failed_with(self.run_scenario(flow(extra=extra)), ["task-unmet", "capability-unused"])
        self.assertEqual((verdict["tasks_met"], verdict["part_reads"]), ([app for app in APPS if app != "cells"], 0))

    def test_each_unused_capability_fails(self):
        timer_pressed = dict(ACTS, timer=[("press", {"key": "arrowright"})])
        cases = (
            ("lint_calls", {"extra": dict(EXTRA, cells=[("snapshot", {"part": 2})])}, {}),
            ("screenshot_calls", {"extra": dict(EXTRA, **{"flight-booker": []})}, {}),
            ("advance_calls", {"acts": timer_pressed}, {}),
            ("part_reads", {"extra": dict(EXTRA, cells=[("lint", {})])}, {"pages": {"cells": 1}}),
        )
        for field, shape, knobs in cases:
            with self.subTest(field=field):
                verdict = self.assert_failed_with(self.run_scenario(flow(**shape), **knobs), ["capability-unused"])
                self.assertEqual(verdict[field], 0)
                self.assertEqual(verdict["tasks_met"], list(APPS))

    def test_a_start_that_left_no_kept_check_fails(self):
        for knobs, recorded in (({"no_record": True}, 0), ({"record_as": "counter"}, len(APPS))):
            with self.subTest(knobs=knobs):
                verdict = self.assert_failed_with(self.run_scenario(**knobs), ["recording-not-kept"])
                self.assertEqual((verdict["recorded_sessions"], verdict["recording_kept"]), (recorded, False))

    def test_a_reply_without_a_check_fails(self):
        verdict = self.assert_failed_with(self.run_scenario(check=None), ["no-check"])
        self.assertEqual((verdict["check_steps"], verdict["check_passed"], verdict["check_can_fail"]), (0, False, False))
        self.assertFalse((self.state / "check.json").exists())
        self.assertEqual(self.spawned(), [self.live_spawn()])

    def test_a_check_step_answered_is_error_fails(self):
        verdict = self.assert_failed_with(self.run_scenario(check=[{"tool": "stop"}] + stand_check()),
                                          ["check-failed"])
        self.assertTrue(verdict["check_can_fail"])

    def test_a_check_expectation_not_met_fails(self):
        expect = dict(EXPECT, counter=['"4" id="counter-value"'])
        self.assert_failed_with(self.run_scenario(check=stand_check(expect)), ["check-failed"])

    def test_a_malformed_check_fails_and_is_never_replayed(self):
        steps = stand_check()
        for check in (
            steps + [{"tool": "Bash", "arguments": {"command": "ls"}}],
            steps + [{"tool": "status", "note": "x"}],
            steps + [{"tool": "status", "arguments": []}],
            steps + [{"tool": "status", "expect": [1]}],
            steps + [{"tool": "status"}] * 401,
        ):
            with self.subTest(last=check[-1], steps=len(check)):
                verdict = self.assert_failed_with(self.run_scenario(check=check), ["check-failed"])
                self.assertFalse(verdict["check_can_fail"])
                self.assertEqual(self.spawned(), [self.live_spawn()])

    def test_a_check_that_cannot_fail_fails(self):
        expect = {app: ['id="task-header-spacer"'] for app in APPS}
        verdict = self.assert_failed_with(self.run_scenario(check=stand_check(expect)), ["check-cannot-fail"])
        self.assertTrue(verdict["check_passed"])

    def test_a_check_missing_an_app_fails(self):
        self.assert_failed_with(self.run_scenario(check=stand_check(apps=APPS[:3])),
                                ["check-failed", "check-cannot-fail"])

    def test_an_empty_session_is_never_a_pass(self):
        verdict = self.assert_failed_with(self.run_scenario([], check=None),
                                          ["no-tool-call", "task-unmet", "capability-unused", "recording-not-kept",
                                           "no-check"])
        self.assertEqual((verdict["tool_calls"], verdict["tasks_met"]), (0, []))

    def test_a_tool_beside_the_escher_tools_is_not_isolated(self):
        self.assert_failed_with(self.run_scenario(extra_tools=["Bash"]), ["not-isolated"])

    def test_a_second_server_is_not_isolated(self):
        self.assert_failed_with(self.run_scenario(extra_servers=[{"name": "other", "status": "connected"}]),
                                ["not-isolated"])

    def test_a_session_cwd_naming_the_repo_root_is_not_isolated(self):
        cwd = f"/work/{self.root.name.upper()}/session"
        self.assert_failed_with(self.run_scenario(init={"cwd": cwd}), ["not-isolated"])

    def test_an_api_key_source_is_not_isolated(self):
        self.assert_failed_with(self.run_scenario(init={"apiKeySource": "ANTHROPIC_API_KEY"}), ["not-isolated"])

    def test_a_failed_client_fails_the_run(self):
        verdict = self.assert_failed_with(self.run_scenario(exit=1), ["client-exit"])
        self.assertEqual(verdict["client_exit"], 1)

    def test_an_unsuccessful_result_fails_the_run(self):
        self.assert_failed_with(self.run_scenario(subtype="error_max_budget_usd"), ["no-result"])

    def test_a_missing_transcript_fails_the_run(self):
        (self.bin / "claude").write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        proc = self.pipe("run", "stand")
        self.assertEqual(proc.returncode, 1, proc.stderr)
        verdict = self.verdict()
        self.assertIn("no-result", verdict["reasons"])
        self.assertFalse(verdict["isolated"])

    def test_counts_that_disagree_fail_the_run(self):
        verdict = self.assert_failed_with(self.run_scenario(drop_span=True), ["counts-disagree"])
        self.assertEqual(verdict["server_tool_calls"], verdict["tool_calls"] - 1)

    def test_usage_errors_exit_2_with_empty_stdout(self):
        cases = ((), ("no-such-verb",), ("run",), ("run", "counter"), ("run", "nonesuch"), ("run", "stand", "extra"),
                 ("status", "x"))
        for args in cases:
            with self.subTest(args=args):
                proc = self.pipe(*args)
                self.assertEqual(proc.returncode, 2)
                self.assertEqual(proc.stdout, "")
                self.assertIn("usage:", proc.stderr)
        self.assertFalse(self.record.exists())
        self.assertFalse(self.state.exists())

    def test_a_missing_client_exits_3_before_any_state(self):
        dirs = [d for d in os.environ["PATH"].split(os.pathsep) if d and not (Path(d) / "claude").exists()]
        proc = self.pipe("run", "stand", path=os.pathsep.join(dirs))
        self.assertEqual(proc.returncode, 3)
        self.assertEqual(proc.stdout, "")
        self.assertEqual(len(proc.stderr.splitlines()), 1)
        self.assertFalse(self.state.exists())

    def test_a_missing_server_binary_exits_3_before_any_state(self):
        self.server.unlink()
        proc = self.pipe("run", "stand")
        self.assertEqual(proc.returncode, 3)
        self.assertEqual(proc.stdout, "")
        self.assertEqual(len(proc.stderr.splitlines()), 1)
        self.assertIn("cargo build --locked -p seven_guis --bin escher-session", proc.stderr)
        self.assertFalse(self.state.exists())
        self.assertFalse(self.record.exists())

    def test_status_and_logs_before_any_run_exit_3(self):
        status = self.pipe("status")
        self.assertEqual(status.returncode, 3)
        self.assertEqual(self.json_lines(status.stdout), [{"event": "status", "outcome": "none"}])
        logs = self.pipe("logs")
        self.assertEqual((logs.returncode, logs.stdout), (3, ""))

    def test_status_reports_the_last_verdict(self):
        self.run_scenario(flow(counter_clicks=2))
        proc = self.pipe("status")
        self.assertEqual(proc.returncode, 0)
        (event,) = self.json_lines(proc.stdout)
        self.assertEqual({k: v for k, v in event.items() if k != "event"}, self.verdict())

    def test_the_client_runs_isolated_with_only_the_escher_server(self):
        self.run_scenario()
        record = self.client_record()
        argv = record["argv"]
        for flag in ("--strict-mcp-config", "--disable-slash-commands", "--no-session-persistence", "-p", "--verbose"):
            self.assertIn(flag, argv)
        for flag, value in (("--tools", ""), ("--setting-sources", ""), ("--allowedTools", "mcp__escher"),
                            ("--output-format", "stream-json"), ("--model", "claude-opus-5-5"),
                            ("--max-budget-usd", "3")):
            self.assertEqual(argv[argv.index(flag) + 1], value, flag)
        self.assertTrue(Path(argv[argv.index("--mcp-config") + 1]).is_absolute())
        (name, server), = record["mcp"]["mcpServers"].items()
        self.assertEqual((name, server["type"], server["command"]), ("escher", "stdio", "sh"))
        self.assertEqual(server["args"], ["-c", 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"', "sh",
                                          str(self.server.resolve()), str((self.state / "server.log").resolve()),
                                          str((self.state / "record").resolve())])
        self.assertIn(' span="tool_call" ', (self.state / "server.log").read_text(encoding="utf-8"))
        self.assertEqual(stat.S_IMODE((self.state / "record").stat().st_mode), 0o700)

    def test_the_prompt_names_no_stand_id(self):
        self.run_scenario()
        argv = self.client_record()["argv"]
        prompt = argv[argv.index("-p") + 1]
        for token in STAND_IDS:
            self.assertNotIn(token, prompt.casefold())
        for goal in ("six apps", "reads 3", "24.12.2026", "31.12.2026", "flight type control", "screenshot",
                     "10 seconds", "10.0s", "Ada Lovelace", "Tischler", "Hans Emil", "A0 holds 5", "=A0*2", "reads 10",
                     "Z25", "=B1+1", "reads 11", "visual lint", "Load page", "message is shown", "```json"):
            self.assertIn(goal, prompt)

    def test_the_session_runs_outside_the_root_in_a_directory_removed_after(self):
        self.run_scenario()
        cwd = Path(self.client_record()["cwd"])
        self.assertNotIn(self.root.resolve(), [cwd, *cwd.parents])
        self.assertNotIn(self.root.name.casefold(), str(cwd).casefold())
        self.assertFalse(cwd.exists())

    def run_planted(self):
        """A passing run whose tool argument, tool answer, reply and check each carry a marker."""
        calls = flow()
        calls.insert(-1, {"tool": "mcp__escher__click", "arguments": {"id": ARGUMENT_MARKER}})
        check = stand_check()
        check.insert(-1, {"tool": "click", "arguments": {"id": CHECK_MARKER}})
        screens = copy.deepcopy(SCREENS)
        screens["counter"] = [s + f'Paragraph "{ANSWER_MARKER}" id="note"\n' for s in screens["counter"]]
        proc = self.run_scenario(calls, check=check, screens=screens)
        self.assert_passed(proc)
        transcript = (self.state / "transcript.jsonl").read_text(encoding="utf-8")
        for marker in (ARGUMENT_MARKER, ANSWER_MARKER, MARKER, CHECK_MARKER):
            self.assertIn(marker, transcript)
        self.assertIn(CHECK_MARKER, (self.state / "check.json").read_text(encoding="utf-8"))
        return proc

    def test_events_carry_no_scrub_key_and_no_planted_marker(self):
        proc = self.run_planted()
        logs = self.pipe("logs")
        self.assertEqual(logs.returncode, 0)
        self.assertEqual(logs.stdout, proc.stdout)
        status = self.pipe("status").stdout
        cleanup = self.pipe("cleanup").stdout
        for text in (proc.stdout, logs.stdout, status, cleanup):
            for event in self.json_lines(text):
                self.assertIsInstance(event, dict)
                self.assertFalse(SCRUB_KEYS & set(event), event)
            for marker in (ARGUMENT_MARKER, ANSWER_MARKER, MARKER, CHECK_MARKER):
                self.assertNotIn(marker, text)

    def test_the_verdict_carries_no_scrub_key_and_no_planted_marker(self):
        self.run_planted()
        text = (self.state / "verdict.json").read_text(encoding="utf-8")
        for marker in (ARGUMENT_MARKER, ANSWER_MARKER, MARKER, CHECK_MARKER):
            self.assertNotIn(marker, text)
        self.assertFalse(SCRUB_KEYS & set(json.loads(text)))

    def test_cleanup_is_idempotent_and_touches_only_its_own_state(self):
        sentinel = self.root / "target" / "agent-run" / "status.json"
        sentinel.parent.mkdir(parents=True)
        sentinel.write_text("{}", encoding="utf-8")
        self.run_scenario()
        for _ in range(2):
            proc = self.pipe("cleanup")
            self.assertEqual(proc.returncode, 0)
            self.assertEqual(self.json_lines(proc.stdout), [{"event": "cleanup", "outcome": "done"}])
        self.assertFalse(self.state.exists())
        self.assertTrue(sentinel.is_file())
        self.assertTrue(self.server.is_file())


if __name__ == "__main__":
    unittest.main()
