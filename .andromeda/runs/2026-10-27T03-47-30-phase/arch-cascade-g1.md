# arch · cascade read · group 1 of 6

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:59) The rewritten "What Escher is" and "Try it" must describe the stand, its checks and the driver as run headlessly through the harness, with no window, GPU or compositor, and claim no more.
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks; the reference browser keeps its own state best-effort.
  >   - The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19)
  (.andromeda/architecture.md:13)
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks
- (.andromeda/architecture-detail/stack-and-technologies.md:73) The capabilities statement's restated cold-agent paragraph must describe the 2026-10-25 run as this records it: an operator-host live run of the `claude` client against the workspace's own `escher-session mcp`, never a CI run.
  > the cold-agent pipe's agent client, the Claude Code CLI `claude` — a host tool on the operator host only, a `run` precondition (exit 3) and no CI requirement (CI runs the pipe's shim tests only), read at 2.1.308 on the dev host, the live run's `client_version` the same (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md) — its MCP server, since 2026-10-15-cold-agent-test, the workspace's own `escher-session mcp` (a `run` precondition built by the operator, never by the pipe; the stub retired with that chunk)
  (.andromeda/architecture.md:241)
  > scripts/               agent-run.{sh,ps1} (test contract) · cold-agent.{sh,ps1} (cold-agent pipe) · code-graph tooling
- (.andromeda/architecture-detail/established-decisions.md:39) The moved accent and success rows must land stating their colours as painted on the headless stand, whose documents render in the Light scheme.
  > headless documents and the WPT viewport use `ColorScheme::Light` (examples/screenshot.rs:76-81; examples/paint_bench.rs:94-99; wpt/runner/src/main.rs:552-557)
  (.andromeda/architecture.md:46)
  > headless documents and the WPT viewport in `ColorScheme::Light`.
- (.andromeda/architecture-detail/established-decisions.md:112) The README's "a `select` driven by id" must stay inside the single-choice model the engine holds, naming no `multiple` select, `optgroup` option or popup.
  > A `select`'s choice is held as its options' `CHECKED` element state, so `:checked` matches the chosen option; an option created with a `selected` attribute is seeded `CHECKED` (packages/blitz-dom/src/node/element.rs:472-478). The rules, in the crate-private `node/select.rs` cited to the WHATWG `select` element: a select's options are its `option` children in tree order (an `optgroup`'s are not listed);
  (.andromeda/architecture.md:62)
  > The select model: every choice that changes the chosen option goes through the one path `apply_select_choice`, and a choice of the chosen option fires nothing; no popup is painted, an option is not clicked, and a `multiple` select or an `optgroup`'s options are not modelled.
- (.andromeda/architecture-detail/established-decisions.md:168) The README's driver paragraph and "Try it" must describe the session's crossings as they stand — a local Unix-domain socket and the `mcp` role's stdio, no TCP or UDP port, unix only across processes — and claim no other carrier.
  > **[Driver session]** A session crosses processes on a `std`-only Unix-domain socket in an owner-only state directory — no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port (the founder's choice, 2026-10-07, relayed verbatim);
  (.andromeda/architecture-detail/established-decisions.md:170)
  > since 2026-10-10-mcp-surface a session's calls also cross from an MCP client over the `escher-session mcp` role's stdio — a child of that client, local to the invoking user, holding its session in its own process with no listener
  (.andromeda/architecture.md:94)
  > **[Driver session]** a session crosses processes on a `std`-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio — no third-party IPC crate and no TCP or UDP port; ids, names, masked values, snapshot text, diffs, shots and lint findings cross, never a `NodeId`;
- (.andromeda/architecture-detail/established-decisions.md:178) Where the README names a session recorded as a replayable check, it must state it as owner-only, kept at rest and replayed through `Session::run`, the two committed stand checks the one exception.
  > A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check — `std` only, no dependency added, unix only like the socket: the calls as they crossed (target ids, a `press` key, an `advance`'s ms, text typed into an unmasked control), a masked or unresolved `type` as one fixed stand-in, each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels — so a kept check holds the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest, never logged and never named by a tool call; a kept check replays through `Session::run` alone (`Check::replay`), and the repository's own two scripted stand checks are committed in git, the one exception to owner-only.
  (.andromeda/architecture.md:96)
  > A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call; the repository's two committed stand checks are the one exception to owner-only.
  (.andromeda/architecture.md:238)
  > tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture.md:242) The capabilities statement is edited in place under `docs/`, beside upstream's three files, and the chunk adds no file there.
  > docs/                  capabilities.md (escher's capability statement) beside upstream's three files — an upstream change under docs/ never conflicts with it
- (.andromeda/architecture.md:246) The stand's DejaVu Sans item, moved out of the design system, lands as this line states the font: seven_guis' crate-root `DEJAVU_SANS`, the one font the headless stand registers.
  > bundled DejaVuSans.woff2 — seven_guis' bytes are the crate-root `DEJAVU_SANS`, compiled on native too, where the headless stand registers the same font (examples/seven_guis/src/lib.rs:7-24; examples/todomvc/src/wasm.rs:1-15; examples/wasm_hello/src/lib.rs:21-23; examples/wasm_hello/src/lib.rs:102-106)

## Bindings
- **established-decisions ↔ the body's Infrastructure Patterns**: the moved stand rows land together — the accent and success colours in the Light scheme headless documents render in (.andromeda/architecture-detail/established-decisions.md:39) and the DejaVu Sans item as the `DEJAVU_SANS` bytes the headless stand registers (.andromeda/architecture.md:246) — so the wrap amendment states the stand's colours and its one font in one consistent place.
- **stack-and-technologies ↔ established-decisions**: the 2026-10-25 run the statement describes is the operator-host `claude` client driving `escher-session mcp` (.andromeda/architecture-detail/stack-and-technologies.md:73), whose stdio is the ratified crossing the driver session states (.andromeda/architecture-detail/established-decisions.md:170); the statement names no carrier beyond it.
- **established-decisions ↔ the body's Infrastructure Patterns**: the recorded check's one exception to owner-only (.andromeda/architecture-detail/established-decisions.md:178) is the committed checks directory of the tree (.andromeda/architecture.md:238); a README line naming the replayable check names that exception and no other committed check.
- **design-philosophy ↔ established-decisions**: the README's headless claim (.andromeda/architecture-detail/design-philosophy.md:59) and its driver paragraph (.andromeda/architecture-detail/established-decisions.md:168) describe one driver that runs with no window over a local socket and stdio, and the README states neither beyond what the records prove.
