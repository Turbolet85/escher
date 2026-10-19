# layouts extract

## Relevance
partial — the chunk creates and modifies no screen, region or focus stop. It touches two recorded CLI screens (`scripts/agent-run.sh` and `escher-session --record`) and the stand's headless mount, which every replay boots.

## Constraints
- layout-templates §Surface: cli → Primary screens requires `scripts/agent-run.sh` to keep its usage `agent-run.sh <verb> [selection]`, with the verbs `boot · run · status · cleanup · logs` and the run selections `stand · all · <blitz-tests file name>`. Every verb's stdout must be JSON lines only, and the raw cargo output stays in `target/agent-run/run.log`. A replay target reached through `run stand` must fit that screen. Whether the `stand_*` glob already picks up a new target is research's question.
- layout-templates §Surface: cli → Primary screens requires `escher-session`'s `--record <dir>` to be taken at most once, and only on `start`, the host role `serve` and the `mcp` role. Any other shape is a usage ending (exit 2). The plan also requires the following:
  - each recorded session is kept as one `check-{n}.json` (`0600`, never replacing a file) in a `0700` directory whose parent must exist;
  - a directory open to its group or others ends as `record-dir-not-private` (exit 3), and `start` spawns nothing;
  - nothing the caller typed is written back, the path included.
  The recordings this chunk commits must come through that unchanged screen.
- layout-templates §Surface: cli → Primary screens requires `help` and `tools/list` to read the same with and without `--record`. The `mcp` role lists the eleven verbs in order, with `help` last. Adding an `answer` key to the kept check therefore must not move the self-description or the tool listing.
- layout-templates §Surface: desktop-native → Primary screens requires the host role and the `mcp` role to boot one lean task through `seven_guis::stand` only. That is the `task_in_shell` TaskShell mount at the pinned 800 × 600 viewport, scale 1.0, Light, with the bundled DejaVu Sans. Each consumer adds no markup, id, class, style, wrapper or order, and a held instance reads as a fresh boot. A replay on a freshly booted session must take that same mount.
- layout-templates §Surface: desktop-native → Primary screens records each lean task's author `id`s as its stable element ids (for example `counter-value` · `counter-increment`, the `flight-*`, `timer-*` and `crud-*` sets, and `crud-person-{person.id}`). It records them as adding no class, style, wrapper or order. A recorded check addresses these ids, so a witness that changes the app's behaviour must not rename or reorder them unless it moves this plan's record.

## Patterns to follow
- An in-file fixture rather than stand markup: per layout-templates §Surface: desktop-native → IA notes, the pinned viewport is scrolled only on an in-file fixture taller than it. Following that, the behaviour-change witness can use a fixture variant beside the stand, so the stand facts in §Primary screens stay unchanged.
- Boot through the one stand mount: per layout-templates §Surface: desktop-native → Primary screens, the CLI host, the `mcp` role and the session checks are consumers of the same `seven_guis::stand` boot. The replay runner should be one more consumer of it, not a second mount.
- Stdout is the answer, never a log: per layout-templates §Surface: cli → Primary screens, every `escher-session` ending writes one uncoloured JSON line on stdout (or nothing, for a usage ending), and `agent-run.sh` writes JSON lines only. A replay's red verdict, naming the step that changed, should ride the existing JSON-line event stream.

## Anti-patterns to avoid
- Editing a stand task's markup, id, class, style, wrapper or order to make a behaviour change for the red witness. layout-templates §Surface: desktop-native → Primary screens records every consumer of the mount as adding none of these.
- Adding a CLI shape, flag position or ending outside the recorded grammar, such as a replay verb on `escher-session` or a `--record` outside its three roles. Echoing a typed path or text back on any ending is also out. Both breach layout-templates §Surface: cli → Primary screens.

## Contract bindings
- layouts ↔ tests: layout-templates §Surface: cli → Primary screens defers `agent-run.sh`'s exit grammar and event schema, and the session contract that `escher-session` serves, to test-plan §3 (→ Session lifecycle). The replay target's events and endings bind there.
- layouts ↔ a11y: the stand mount and author ids in layout-templates §Surface: desktop-native → Primary screens are what `stand_keyboard`'s Tab tables read. A stand-task witness change would bind to the a11y record, which is a further reason to prefer a fixture variant.

## Acceptance criteria contributions
- (layouts) `bash scripts/agent-run.sh run stand` (or `run <replay target>`) writes JSON lines only on stdout, with the replay's results among them, and the raw cargo output stays in `target/agent-run/run.log` (per layout-templates §Surface: cli → Primary screens).
- (layouts) The `escher-session help` line and the `mcp` role's `tools/list` read byte-identical before and after the `answer` key lands, with and without `--record` (per layout-templates §Surface: cli → Primary screens).
- (layouts) Every replay boots its session through `seven_guis::stand` at the pinned 800 × 600 TaskShell mount. `git diff` of `examples/seven_guis/src/tasks/` and `examples/seven_guis/src/app.rs` adds no markup, id, class, style, wrapper or order, and the behaviour-change witness lives in a fixture variant (per layout-templates §Surface: desktop-native → Primary screens).
