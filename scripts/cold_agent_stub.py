#!/usr/bin/env python3
"""The cold-agent pipe's stub tool: a stdlib-only MCP server over stdio (newline-delimited JSON-RPC 2.0).

It models a toy counter (a `count` display, `inc` · `dec` · `reset` buttons; `dec` is disabled at 0) behind three
tools — `list`, `read {id}`, `press {id}`. A wrong call is refused as a tool result with `isError: true` whose
first word names its cause (`not-found` · `disabled` · `not-pressable` · `malformed`) and changes nothing. Every
`tools/call` appends one JSON line to the `--calls` file (seq · tool · outcome · cause, never an argument value)
and rewrites the `--state` file with the count. Only protocol frames go to stdout.
"""

import argparse
import json
import sys

SERVER_NAME = "stub"
ID_SCHEMA = {
    "type": "object",
    "properties": {"id": {"type": "string", "description": "an element id, as `list` reports it"}},
    "required": ["id"],
    "additionalProperties": False,
}
TOOLS = [
    {
        "name": "list",
        "description": "List every element of the counter: id, role and whether it is enabled.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
    },
    {
        "name": "read",
        "description": "Read one element: the display's number, or a button's label and enabled state.",
        "inputSchema": ID_SCHEMA,
    },
    {
        "name": "press",
        "description": "Press one enabled button.",
        "inputSchema": ID_SCHEMA,
    },
]
LABELS = {"inc": "Increment", "dec": "Decrement", "reset": "Reset"}


class Refusal(Exception):
    def __init__(self, cause, detail):
        super().__init__(f"{cause}: {detail}")
        self.cause = cause


class Counter:
    def __init__(self):
        self.count = 0

    def enabled(self, button):
        return not (button == "dec" and self.count == 0)

    def elements(self):
        return [{"id": "count", "role": "display", "enabled": True}] + [
            {"id": b, "role": "button", "enabled": self.enabled(b)} for b in LABELS
        ]

    def list(self, args):
        if args:
            raise Refusal("malformed", "list takes no arguments")
        return self.elements()

    def element_id(self, tool, args):
        if set(args) != {"id"} or not isinstance(args["id"], str):
            raise Refusal("malformed", f"{tool} takes exactly one string argument, id")
        element = args["id"]
        if element != "count" and element not in LABELS:
            raise Refusal("not-found", f"no element {element!r}")
        return element

    def read(self, args):
        element = self.element_id("read", args)
        if element == "count":
            return {"id": "count", "reading": self.count}
        return {"id": element, "label": LABELS[element], "enabled": self.enabled(element)}

    def press(self, args):
        element = self.element_id("press", args)
        if element == "count":
            raise Refusal("not-pressable", "count is a display, not a button")
        if not self.enabled(element):
            raise Refusal("disabled", f"{element} is disabled")
        self.count = {"inc": self.count + 1, "dec": self.count - 1, "reset": 0}[element]
        return {"id": "count", "reading": self.count}


class Server:
    def __init__(self, state_file, calls_file):
        self.counter = Counter()
        self.state_file = state_file
        self.calls_file = calls_file
        self.seq = 0

    def call(self, params):
        name = params.get("name")
        args = params.get("arguments") or {}
        tools = {"list": self.counter.list, "read": self.counter.read, "press": self.counter.press}
        tool = tools.get(name) if isinstance(name, str) else None
        try:
            if tool is None:
                raise Refusal("not-found", "no such tool")
            if not isinstance(args, dict):
                raise Refusal("malformed", "arguments must be an object")
            body, error, cause = json.dumps(tool(args)), False, None
        except Refusal as refusal:
            body, error, cause = str(refusal), True, refusal.cause
        self.seq += 1
        self.log(name if tool else None, cause)
        return {"content": [{"type": "text", "text": body}], "isError": error}

    def log(self, tool, cause):
        record = {"seq": self.seq, "tool": tool, "outcome": "refused" if cause else "ok", "cause": cause}
        with open(self.calls_file, "a", encoding="utf-8", newline="\n") as f:
            f.write(json.dumps(record) + "\n")
        with open(self.state_file, "w", encoding="utf-8", newline="\n") as f:
            json.dump({"count": self.counter.count}, f)

    def handle(self, message):
        method, params = message.get("method"), message.get("params") or {}
        if method == "initialize":
            return {
                "protocolVersion": params.get("protocolVersion", "2025-06-18"),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": SERVER_NAME, "version": "0.1.0"},
            }
        if method == "tools/list":
            return {"tools": TOOLS}
        if method == "tools/call":
            return self.call(params)
        raise LookupError(method)


def reply(frame):
    sys.stdout.write(json.dumps(frame) + "\n")
    sys.stdout.flush()


def serve(server):
    while True:
        line = sys.stdin.readline()
        if not line:
            return
        if not line.strip():
            continue
        try:
            message = json.loads(line)
        except ValueError:
            reply({"jsonrpc": "2.0", "id": None, "error": {"code": -32700, "message": "parse error"}})
            continue
        if not isinstance(message, dict) or "method" not in message:
            continue
        if "id" not in message:
            continue
        try:
            reply({"jsonrpc": "2.0", "id": message["id"], "result": server.handle(message)})
        except LookupError:
            reply({"jsonrpc": "2.0", "id": message["id"], "error": {"code": -32601, "message": "method not found"}})


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--state", required=True)
    parser.add_argument("--calls", required=True)
    opts = parser.parse_args()
    serve(Server(opts.state, opts.calls))


if __name__ == "__main__":
    main()
