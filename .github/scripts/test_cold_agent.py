#!/usr/bin/env python3
"""Contract of scripts/cold_agent_stub.py (over a pipe) and scripts/cold-agent.sh (under a claude shim, no live
model). Run with `python3 -m unittest discover -s .github/scripts`."""

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
STUB = ROOT / "scripts" / "cold_agent_stub.py"

SCRUB_KEYS = {"url", "href", "src", "html", "text", "value", "attrs", "path", "request", "error", "panic.payload"}
MARKER = "cold-agent-transcript-marker"
STUB_TOOLS = ["mcp__stub__list", "mcp__stub__read", "mcp__stub__press"]
INC = {"tool": "mcp__stub__press", "arguments": {"id": "inc"}}

# The shim stands in for `claude -p`: it records its argv and cwd, drives the configured stub as a minimal MCP
# client through the scenario's calls, and prints a stream-json transcript built from what the stub returned.
SHIM = """\
#!/usr/bin/env python3
import json, os, subprocess, sys

args = sys.argv[1:]
with open(__RECORD__, "w", encoding="utf-8") as f:
    json.dump({"argv": args, "cwd": os.getcwd()}, f)
with open(__SCENARIO__, encoding="utf-8") as f:
    scenario = json.load(f)
with open(args[args.index("--mcp-config") + 1], encoding="utf-8") as f:
    server = json.load(f)["mcpServers"]["stub"]
stub = subprocess.Popen([server["command"], *server["args"]], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)


def send(frame):
    stub.stdin.write(json.dumps(frame) + "\\n")
    stub.stdin.flush()


def rpc(n, method, params):
    send({"jsonrpc": "2.0", "id": n, "method": method, "params": params})
    return json.loads(stub.stdout.readline())["result"]


rpc(1, "initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "shim"}})
send({"jsonrpc": "2.0", "method": "notifications/initialized"})
tools = ["mcp__stub__" + t["name"] for t in rpc(2, "tools/list", {})["tools"]]
cwd = os.getcwd()
init = {"type": "system", "subtype": "init", "cwd": cwd, "tools": tools + scenario.get("extra_tools", []),
        "mcp_servers": [{"name": "stub", "status": "connected"}], "model": "shim-model", "slash_commands": [],
        "skills": [], "apiKeySource": "none", "claude_code_version": "0.0.0-shim",
        "memory_paths": {"auto": "/home/shim/.claude/projects/" + cwd.replace("/", "-") + "/memory/"}}
init.update(scenario.get("init", {}))
out = [init]
for n, call in enumerate(scenario.get("calls", [])):
    use = "toolu_%d" % n
    name, arguments = call["tool"], call.get("arguments", {})
    out.append({"type": "assistant", "message": {"content": [
        {"type": "tool_use", "id": use, "name": name, "input": arguments}]}})
    if name.startswith("mcp__stub__"):
        result = rpc(10 + n, "tools/call", {"name": name[len("mcp__stub__"):], "arguments": arguments})
        content, is_error = result["content"], result["isError"]
    else:
        content, is_error = "No such tool available: " + name, True
    out.append({"type": "user", "message": {"content": [
        {"type": "tool_result", "tool_use_id": use, "content": content, "is_error": is_error}]}})
stub.stdin.close()
stub.wait()
subtype = scenario.get("subtype", "success")
out.append({"type": "assistant", "message": {"content": [{"type": "text", "text": "final reply " + __MARKER__}]}})
out.append({"type": "result", "subtype": subtype, "is_error": subtype != "success", "num_turns": 4,
            "duration_ms": 1234, "total_cost_usd": 0.0123, "result": "final reply " + __MARKER__,
            "usage": {"input_tokens": 100, "output_tokens": 20, "cache_read_input_tokens": 300,
                      "cache_creation_input_tokens": 40},
            "permission_denials": scenario.get("denials", [])})
for record in out:
    print(json.dumps(record))
sys.exit(scenario.get("exit", 0))
"""


class StubProcess:
    def __init__(self, workdir):
        self.state = workdir / "stub-state.json"
        self.calls = workdir / "calls.jsonl"
        self.proc = subprocess.Popen(
            ["python3", str(STUB), "--state", str(self.state), "--calls", str(self.calls)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
        )
        self.next_id = 0

    def rpc(self, method, params=None):
        self.next_id += 1
        frame = {"jsonrpc": "2.0", "id": self.next_id, "method": method}
        if params is not None:
            frame["params"] = params
        self.proc.stdin.write(json.dumps(frame) + "\n")
        self.proc.stdin.flush()
        return json.loads(self.proc.stdout.readline())

    def notify(self, method):
        self.proc.stdin.write(json.dumps({"jsonrpc": "2.0", "method": method}) + "\n")
        self.proc.stdin.flush()

    def call(self, tool, arguments=None):
        params = {"name": tool} if arguments is None else {"name": tool, "arguments": arguments}
        return self.rpc("tools/call", params)["result"]

    def count(self):
        return json.loads(self.call("read", {"id": "count"})["content"][0]["text"])["reading"]

    def close(self):
        self.proc.stdin.close()
        rest = self.proc.stdout.read()
        self.proc.stdout.close()
        self.proc.wait()
        return rest


class ColdAgentStubTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.stub = StubProcess(Path(self._tmp.name))
        self.stub.rpc("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}})
        self.stub.notify("notifications/initialized")

    def tearDown(self):
        if self.stub.proc.poll() is None:
            self.stub.close()
        self._tmp.cleanup()

    def call_log(self):
        return [json.loads(line) for line in self.stub.calls.read_text(encoding="utf-8").splitlines()]

    def test_initialize_then_tools_list_offers_exactly_list_read_press(self):
        tools = self.stub.rpc("tools/list")["result"]["tools"]
        self.assertEqual(sorted(t["name"] for t in tools), ["list", "press", "read"])
        for tool in tools:
            self.assertEqual(tool["inputSchema"]["type"], "object")

    def test_three_presses_of_inc_reach_three(self):
        for _ in range(3):
            self.assertFalse(self.stub.call("press", {"id": "inc"})["isError"])
        self.assertEqual(self.stub.count(), 3)
        self.assertEqual(json.loads(self.stub.state.read_text(encoding="utf-8")), {"count": 3})

    def test_dec_is_disabled_at_zero_and_enabled_above_it(self):
        listed = json.loads(self.stub.call("list")["content"][0]["text"])
        self.assertIn({"id": "dec", "role": "button", "enabled": False}, listed)
        self.stub.call("press", {"id": "inc"})
        self.assertFalse(self.stub.call("press", {"id": "dec"})["isError"])
        self.assertEqual(self.stub.count(), 0)

    def test_each_refusal_names_its_cause_and_changes_nothing(self):
        self.assertEqual(self.stub.count(), 0)
        cases = [
            ("not-found", "read", {"id": "nope"}),
            ("not-found", "no_such_tool", {}),
            ("disabled", "press", {"id": "dec"}),
            ("not-pressable", "press", {"id": "count"}),
            ("malformed", "press", {}),
            ("malformed", "press", None),
            ("malformed", "read", {"id": "count", "extra": "x"}),
            ("malformed", "press", {"id": 7}),
            ("malformed", "list", {"id": "inc"}),
        ]
        for cause, tool, arguments in cases:
            with self.subTest(cause=cause, tool=tool, arguments=arguments):
                result = self.stub.call(tool, arguments)
                self.assertTrue(result["isError"])
                self.assertEqual(result["content"][0]["text"].split(":")[0], cause)
                self.assertEqual(self.stub.count(), 0)
        refused = [c for c in self.call_log() if c["outcome"] == "refused"]
        self.assertEqual([c["cause"] for c in refused], [c[0] for c in cases])

    def test_the_call_log_holds_one_line_per_call_and_no_argument_value(self):
        self.stub.call("read", {"id": MARKER})
        self.stub.call("press", {"id": "inc"})
        self.stub.call("list")
        log = self.call_log()
        self.assertEqual(
            [(c["seq"], c["tool"], c["outcome"], c["cause"]) for c in log],
            [(1, "read", "refused", "not-found"), (2, "press", "ok", None), (3, "list", "ok", None)],
        )
        self.assertNotIn(MARKER, self.stub.calls.read_text(encoding="utf-8"))

    def test_stdout_carries_only_json_rpc_frames(self):
        self.assertEqual(self.stub.rpc("resources/list")["error"]["code"], -32601)
        self.stub.call("press", {"id": "count"})
        self.stub.proc.stdin.write("not json\n")
        self.stub.proc.stdin.flush()
        rest = self.stub.close()
        frames = [json.loads(line) for line in rest.splitlines()]
        self.assertEqual([f["error"]["code"] for f in frames], [-32700])
        for frame in frames:
            self.assertEqual(frame["jsonrpc"], "2.0")


class ColdAgentPipeTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        (self.root / "scripts").mkdir()
        shutil.copy(PIPE, self.root / "scripts" / PIPE.name)
        shutil.copy(STUB, self.root / "scripts" / STUB.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.record = self.root / "claude-record.json"
        self.scenario_file = self.root / "scenario.json"
        self.state = self.root / "target" / "cold-agent"
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

    def run_scenario(self, calls, **knobs):
        self.scenario_file.write_text(json.dumps(dict(knobs, calls=calls)), encoding="utf-8")
        return self.pipe("run", "counter")

    def json_lines(self, text):
        return [json.loads(line) for line in text.splitlines()]

    def verdict(self):
        return json.loads((self.state / "verdict.json").read_text(encoding="utf-8"))

    def client_record(self):
        return json.loads(self.record.read_text(encoding="utf-8"))

    def assert_failed_for(self, proc, reason):
        self.assertEqual(proc.returncode, 1, proc.stderr)
        verdict = self.verdict()
        self.assertEqual(verdict["outcome"], "failed")
        self.assertIn(reason, verdict["reasons"])
        return verdict

    def test_three_presses_pass_with_no_wrong_call(self):
        proc = self.run_scenario([{"tool": "mcp__stub__list"}, INC, INC, INC])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        start, end = self.json_lines(proc.stdout)
        self.assertEqual((start["event"], start["task"]), ("run.start", "counter"))
        self.assertEqual(end["event"], "run.end")
        verdict = self.verdict()
        self.assertEqual(
            {k: verdict[k] for k in ("outcome", "reasons", "isolated", "counts_agree", "tool_calls", "wrong_calls")},
            {"outcome": "passed", "reasons": [], "isolated": True, "counts_agree": True, "tool_calls": 4,
             "wrong_calls": 0},
        )
        self.assertEqual(verdict["final_count"], 3)
        self.assertEqual(sorted(verdict["session_tools"]), sorted(STUB_TOOLS))
        self.assertEqual((verdict["client_version"], verdict["model"]), ("0.0.0-shim", "shim-model"))
        self.assertEqual(
            [verdict[k] for k in ("input_tokens", "output_tokens", "cache_read_input_tokens",
                                  "cache_creation_input_tokens", "cost_usd", "num_turns", "duration_ms")],
            [100, 20, 300, 40, 0.0123, 4, 1234],
        )
        self.assertEqual(verdict["transcript_file"], "target/cold-agent/transcript.jsonl")
        self.assertEqual({k: v for k, v in end.items() if k != "event"}, verdict)
        for name in ("transcript.jsonl", "calls.jsonl", "stub-state.json", "client.log", "events.jsonl"):
            self.assertTrue((self.state / name).is_file(), name)

    def test_wrong_calls_are_counted_from_the_stub_log_and_agree_with_the_transcript(self):
        wrong = [
            {"tool": "mcp__stub__read", "arguments": {"id": "nope"}},
            {"tool": "mcp__stub__press", "arguments": {"id": "dec"}},
            {"tool": "mcp__stub__press", "arguments": {}},
        ]
        proc = self.run_scenario(wrong + [INC, INC, INC])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        verdict = self.verdict()
        self.assertEqual(
            (verdict["outcome"], verdict["wrong_calls"], verdict["transcript_errors"], verdict["counts_agree"]),
            ("passed", 3, 3, True),
        )

    def test_a_tool_outside_the_stub_is_a_wrong_call(self):
        proc = self.run_scenario([{"tool": "Bash", "arguments": {"command": "ls"}}, INC, INC, INC])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        verdict = self.verdict()
        self.assertEqual((verdict["wrong_calls"], verdict["transcript_errors"], verdict["counts_agree"]), (1, 1, True))

    def test_a_permission_denial_is_a_wrong_call(self):
        proc = self.run_scenario([INC, INC, INC], denials=[{"tool_name": "mcp__stub__press"}])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(self.verdict()["wrong_calls"], 1)

    def test_an_unmet_task_fails(self):
        verdict = self.assert_failed_for(self.run_scenario([INC, INC]), "task-unmet")
        self.assertEqual(verdict["reasons"], ["task-unmet"])
        self.assertEqual(verdict["final_count"], 2)

    def test_an_empty_session_is_never_a_pass(self):
        verdict = self.assert_failed_for(self.run_scenario([]), "no-tool-call")
        self.assertEqual(verdict["tool_calls"], 0)

    def test_a_session_cwd_naming_the_repo_root_is_not_isolated(self):
        cwd = f"/work/{self.root.name.upper()}/session"
        verdict = self.assert_failed_for(self.run_scenario([INC, INC, INC], init={"cwd": cwd}), "not-isolated")
        self.assertEqual(verdict["reasons"], ["not-isolated"])

    def test_a_tool_beside_the_stubs_is_not_isolated(self):
        verdict = self.assert_failed_for(self.run_scenario([INC, INC, INC], extra_tools=["Bash"]), "not-isolated")
        self.assertEqual(verdict["reasons"], ["not-isolated"])

    def test_an_api_key_source_is_not_isolated(self):
        self.assert_failed_for(self.run_scenario([INC, INC, INC], init={"apiKeySource": "ANTHROPIC_API_KEY"}),
                               "not-isolated")

    def test_a_failed_client_fails_the_run(self):
        verdict = self.assert_failed_for(self.run_scenario([INC, INC, INC], exit=1), "client-exit")
        self.assertEqual((verdict["reasons"], verdict["client_exit"]), (["client-exit"], 1))

    def test_an_unsuccessful_result_fails_the_run(self):
        self.assert_failed_for(self.run_scenario([INC, INC, INC], subtype="error_max_budget_usd"), "no-result")

    def test_a_missing_transcript_fails_the_run(self):
        (self.bin / "claude").write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        verdict = self.assert_failed_for(self.pipe("run", "counter"), "no-result")
        self.assertFalse(verdict["isolated"])

    def test_usage_errors_exit_2_with_empty_stdout(self):
        for args in ((), ("no-such-verb",), ("run",), ("run", "nonesuch"), ("run", "counter", "extra"), ("status", "x")):
            with self.subTest(args=args):
                proc = self.pipe(*args)
                self.assertEqual(proc.returncode, 2)
                self.assertEqual(proc.stdout, "")
                self.assertIn("usage:", proc.stderr)
        self.assertFalse(self.record.exists())

    def test_a_missing_client_exits_3_before_any_state(self):
        dirs = [d for d in os.environ["PATH"].split(os.pathsep) if d and not (Path(d) / "claude").exists()]
        proc = self.pipe("run", "counter", path=os.pathsep.join(dirs))
        self.assertEqual(proc.returncode, 3)
        self.assertEqual(proc.stdout, "")
        self.assertFalse(self.state.exists())

    def test_status_and_logs_before_any_run_exit_3(self):
        status = self.pipe("status")
        self.assertEqual(status.returncode, 3)
        self.assertEqual(self.json_lines(status.stdout), [{"event": "status", "outcome": "none"}])
        logs = self.pipe("logs")
        self.assertEqual((logs.returncode, logs.stdout), (3, ""))

    def test_status_reports_the_last_verdict(self):
        self.run_scenario([INC, INC])
        proc = self.pipe("status")
        self.assertEqual(proc.returncode, 0)
        (event,) = self.json_lines(proc.stdout)
        self.assertEqual({k: v for k, v in event.items() if k != "event"}, self.verdict())

    def test_the_client_runs_isolated_with_only_the_stub(self):
        self.run_scenario([INC, INC, INC])
        argv = self.client_record()["argv"]
        for flag in ("--strict-mcp-config", "--disable-slash-commands", "--no-session-persistence", "-p"):
            self.assertIn(flag, argv)
        for flag, value in (("--tools", ""), ("--setting-sources", ""), ("--allowedTools", "mcp__stub"),
                            ("--output-format", "stream-json"), ("--max-budget-usd", "1")):
            self.assertEqual(argv[argv.index(flag) + 1], value, flag)
        config = argv[argv.index("--mcp-config") + 1]
        self.assertTrue(Path(config).is_absolute())

    def test_the_session_runs_outside_the_root_in_a_directory_removed_after(self):
        self.run_scenario([INC, INC, INC])
        cwd = Path(self.client_record()["cwd"])
        self.assertNotIn(self.root.resolve(), [cwd, *cwd.parents])
        self.assertNotIn(self.root.name.casefold(), str(cwd).casefold())
        self.assertFalse(cwd.exists())

    def test_events_are_json_carry_no_scrub_key_and_no_transcript_content(self):
        proc = self.run_scenario([{"tool": "mcp__stub__read", "arguments": {"id": MARKER}}, INC, INC, INC])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        transcript = (self.state / "transcript.jsonl").read_text(encoding="utf-8")
        self.assertIn(MARKER, transcript)
        logs = self.pipe("logs")
        self.assertEqual(logs.returncode, 0)
        self.assertEqual(logs.stdout, proc.stdout)
        status = self.pipe("status").stdout
        cleanup = self.pipe("cleanup").stdout
        for text in (proc.stdout, logs.stdout, status, cleanup):
            for event in self.json_lines(text):
                self.assertIsInstance(event, dict)
                self.assertFalse(SCRUB_KEYS & set(event), event)
            self.assertNotIn(MARKER, text)

    def test_the_verdict_carries_no_scrub_key_and_no_transcript_content(self):
        self.run_scenario([{"tool": "mcp__stub__read", "arguments": {"id": MARKER}}, INC, INC, INC])
        text = (self.state / "verdict.json").read_text(encoding="utf-8")
        self.assertNotIn(MARKER, text)
        self.assertFalse(SCRUB_KEYS & set(json.loads(text)))

    def test_cleanup_is_idempotent_and_touches_only_its_own_state(self):
        sentinel = self.root / "target" / "agent-run" / "status.json"
        sentinel.parent.mkdir(parents=True)
        sentinel.write_text("{}", encoding="utf-8")
        self.run_scenario([INC, INC, INC])
        for _ in range(2):
            proc = self.pipe("cleanup")
            self.assertEqual(proc.returncode, 0)
            self.assertEqual(self.json_lines(proc.stdout), [{"event": "cleanup", "outcome": "done"}])
        self.assertFalse(self.state.exists())
        self.assertTrue(sentinel.is_file())


if __name__ == "__main__":
    unittest.main()
