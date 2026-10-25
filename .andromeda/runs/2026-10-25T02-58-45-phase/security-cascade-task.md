# Task — security · 2026-10-25-capability-statement

## The chunk
Writes one capability statement, a document and not code, of what escher can do as of 0.2.0, read from the shipped surfaces: the twelve driver verbs on their carriers (socket `call`, command line, MCP tools, a kept check), stable element ids, the snapshot and its text, parts and diff, the headless screenshot, the visual lint, session recording and replay, the load wait, the select and range models, keyboard activation and the virtual clock.
Gathers in one place the limits the tool and the masters already state (the three measured-off readings, the boolean model's two limits, the select's unmodelled parts, cross-OS PNG bytes, the unbuilt focus fixup, `text_extent`'s LTR reading) and states one new limit: an acting verb's diff and a lint's findings are not paged, held only by the 1,048,576-byte answer bound.
States the a11y reach (what SC 2.1.1 · 1.4.3 · 2.4.3 is proven on, what is unmeasured) and what an MCP client carries onward: the content escher hands it over stdio, and that anything the client forwards, a model provider included, is beyond escher's boundary, the cold-agent pipe the measured instance; these claims are re-read against the security plan's surfaces and the cold-agent crossing.
Corrects README's MCP-role verb count from eleven to twelve; where the statement stands (escher's README, a tracked escher document it points to, or a file in Viola's untracked incubator in another repository) is an open P4 fork; no code, engine, schema, verb, carrier, log field or span changes.

## Reached files
- README.md · edits the MCP-role line's verb count, the 0.2.0 stage list

## Needs
- The MCP stdio boundary (`escher-session mcp`): trust boundary the spawning client, no listener or authentication, and what a result carries (ids, accessible names, control values with a password's and file input's masked, snapshot text and diffs, a shot's PNG with a password painted masked, a lint's findings); the statement restates this and the client-onward clause
- The driver session socket boundary: owner-only `0700` directory and `0600` socket, no handshake or token, the content its call answers carry, and the idle expiry, restated as a carrier in the statement
- The absence of authentication on the socket and the MCP server, which the statement's description of the carriers must not contradict or overstate
- The cold-agent crossing: the stand's screen carried to the model provider on the live run, the agent-written check as untrusted input, replay on fresh MCP children; the measured instance of what a client carries onward
- The driver command schema and its bounds: twelve verbs, seven argument kinds with bounds (`part` 1 to 1000), eight refusal causes with fixed remedies, validation before anything runs; the statement's verb and limit lists are read from it
- The MCP server's bounds: request line 65,536 bytes, result 1,048,576 bytes, the request id echoed in two forms only, fixed protocol errors; the answer bound is the one bound that holds the unpaged diff and findings the new limit states
- The answer bound on the socket (1,048,576 bytes, `oversize` past it, the call not rolled back), which bounds the unpaged diff and findings on that carrier
- Masking: `MASKED_VALUE` for a password's or file input's value in the snapshot, text, diffs and every carrier, the painted bullets in a shot, and `MASKED_STAND_IN` for a recorded masked `type`; the statement must state the masking as it stands
- Kept checks at rest: owner-only `check-{n}.json` in a `0700` directory, the answer kept whole and masked, never a pixel, the strict `Check::from_bytes` reader and replay through `Session::run`; and the two committed Counter checks as the one exception to owner-only
- The visual lint's findings as ids, closed defect words and numbers only, never page text, colour or value, on every ratified carrier
- The stable element id and the snapshot as what leaves by the platform adapter, the driver's answers, MCP results and kept checks, with no `NodeId` crossing
- The widening rule: a new field of a closed value set inside a ratified carrier is no widening, authored text, an open value set, a new carrier or a new exit is; the statement adds no carrier and must describe the crossings as ratified, not widen them
- The markup-attribute reads the statement describes as capabilities: ARIA states by closed parse, `inert` and `disabled` by presence, the range and select models, Enter and Space activation, names and descriptions as page text leaving only by ratified carriers
- The settle and load wait: `LOAD_WAIT` bounding an acting step's wait on a load in flight, `snapshot`, `screenshot` and `lint` waiting on none; a stated capability and limit
- The `screenshot` and `lint` refusal `fonts-unavailable` and a crop of no pixel refused `off-screen`, as stated limits
- Logging: escher's sink drops other targets and redacts content-named fields, the `command` and `tool_call` spans hold fixed words and counts only; the statement may say no content reaches a log and must say only what holds
- The command line's stdout carrying the answer only, never a log line, and nothing the caller typed written back; restated in the carrier description
- Secrets: a committed document holds no credential, host path, e-mail address or login detail; the statement names the cold-agent run's login only as the operator's own and carries none of it
- Indirect, through the README edit: README is a tracked public file, so the same no-secret and no-host-path rule binds the corrected line and any stage-list entry
- Indirect, through the P4 location fork: a write into Viola's incubator would be a file outside this repository; the statement there must carry nothing beyond what escher's own tracked text may

## Not needed
- Dependency security and supply chain (cargo-deny, `deny.toml` ignores, pinning, `--locked`, `serde_json`, `png` and `color` entries, signed artifacts, licence gate): the chunk adds and moves no dependency and touches no lockfile
- CI secrets and the upstream-only publish, WPT and post-results workflows: no workflow is edited and no secret is read
- The browser app's persistence (sqlite history), the HTTP cache, cookies, clipboard and file dialog handling: no part of the browser app is touched or described
- Remote-content parsing rules (fonts, `@font-face` sniffing, SVG, CSS, stylesheets, images, iframes, `file:` and `data:` URLs in the net provider): the chunk changes no fetch or parse path and routes no URL
- Script execution (`blitz-vibey-script`, ScriptFetcher, JS API arguments, script types): no script path is touched
- The WPT runner's local file reads and its logging: not touched
- rdme's markdown rendering with raw HTML: not touched
- The example CLIs (`screenshot`, `paint_bench`, `preact_script`, `multi_doc`) and their argument handling and panics: not touched
- The argument checks of `agent-run.sh`, `apt-install.sh` and `coverage_floor.py`: no script is edited
- The engine's panic paths and graceful degradation (URL resolve, layout, table, DOM internals, shell setup): no code changes
- The driver session state directory's creation and deletion rules and the session label's grammar: no session is started or changed by the chunk
- The environment values read by source (`RUST_LOG`, `CARGO_*`, `TMPDIR`, `WPT_DIR` and the rest): no env read is added or changed
