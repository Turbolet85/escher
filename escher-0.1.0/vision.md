# escher 0.1.0 — vision

*Source: `escher-0.1.0/intent.md` (intent-file path).*

## Problem
escher is a fork of Blitz (adopted at `0f60502e`) being turned into an **agent-first UI framework**. Today an agent
can't see, drive or check a Blitz UI on its own. Nodes have no semantic identity, so it can only aim at coordinates
or selectors. The screen can only be read as an HTML string, a debug print or a screenshot, and no change stream
exists. The headless harness is a test-only Rust API that clicks at box centres, gives no reason when an action
fails, and leaves settling to the caller.

## Who / why
The user is an agent, working with no display and no outside driver. The founder's development style is
agent-driven, so the framework's own driver is meant to become the harness the project is built with. 0.1.0 is the
first wave. No other project consumes escher in this version.

## Principles
- **Infrastructure ahead of the queue.** Each abstraction is built before the capabilities that use it. In order:
  as-built baseline green on this host + fork CI reached → headless harness booting the stand → stable ids →
  snapshot data model + serialization → change tracking → settle detection → one command/refusal schema → CLI, then
  MCP → screenshot → cold-agent test last.
- **The stand is the proof.** Every requirement is verified headless, by an agent, on an internal stand: three or
  four 7GUIs tasks already in the repo (counter, flight booker, timer, CRUD are the lean) that together exercise
  every requirement.
- **The API explains itself.** An agent with no prior context learns the framework from the tool alone.

## What "0.1.0 done" means
On the stand, with no display, an agent can:
- address every element by a stable id
- read the screen as a compact semantic tree
- act by id and get back only the nodes that changed, after the UI settles
- get a refusal that names its cause and remedy
- take a deterministic screenshot

It can do all of this from a CLI and from MCP. A fresh agent given only the tool completes a stand task and writes
a passing check.

## Out of 0.1.0
UI graph (component → handlers → state → readers) · virtual time everywhere · complete accessibility names/states
beyond what the snapshot needs · visual lint · trace-to-test · auto-exploration · handler coverage · component
isolation · upstream sync. These are later versions, chosen by value.
