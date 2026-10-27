# arch · cascade read · group 1 of 7

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:76) The chunk is done only when `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts) and the `doc` leg (rustdoc `-D warnings` over every workspace library crate) pass on the refactored tree.
  > `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, run as the `fmt` and `clippy`
  > legs of `.github/scripts/ci-leg.sh` (.github/scripts/ci-leg.sh:20-25; .github/workflows/ci.yml:155;
  > .github/workflows/ci.yml:177), with `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts,
  > stopping at the first red) the local pre-push gate (.github/scripts/ci-leg.sh:9; .github/scripts/ci-leg.sh:80-85);
  > `RUSTDOCFLAGS: "-D warnings"` (.github/workflows/ci.yml:19-20; .github/workflows/wpt.yml:13-14; exported by the leg
  > script, .github/scripts/ci-leg.sh:87), applied by the docs job's `doc` leg, `cargo doc --workspace --no-deps
  > --locked`, which documents every workspace library crate — the `browser` bin `blitz` is `doc = false`, so the lib
  > `blitz` alone writes `target/doc/blitz/` — and exits 0 with no output-filename collision
  (.andromeda/architecture.md:19)
  > GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates
- (.andromeda/architecture-detail/stack-and-technologies.md:77) The `fast` leg's clippy step runs blitz-dom's second line too, and the refactored requirement sweep must still assert each 0.2.0 requirement's witness inside the `stand-repeat` leg — the stand checks under the zero-failure budget the leg script selects.
  > since the same chunk the `clippy` leg runs a second line only when the workspace line is green, upstream's own
  > `cargo clippy -p blitz-dom --no-default-features --features
  > svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`, and the
  > `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds (.github/scripts/ci-leg.sh:21-25;
  > .github/scripts/ci-leg.sh:48-65)
  (.andromeda/architecture.md:247)
  > stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the
  > test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no
  > job-level permission; .github/workflows/ci.yml:300-323)
- (.andromeda/architecture-detail/stack-and-technologies.md:72) The `ci-scripts` leg inside `fast` runs `test_blitz_tests_targets.py`, the target pin that holds `stand_requirement_sweep`'s name and target, so the sweep stays one blitz-tests target of its own file with its name unchanged.
  > among them upstream's `wpt_area_changes.py` with its test file and escher's `test_blitz_tests_targets.py`, which
  > reads a manifest with the stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later, no package added:
  > 3.14.7 on the dev host, and green on `ubuntu-latest` in CI#38013740580, the runner's Python version not read
  > (.github/scripts/wpt_area_changes.py:1-92; .github/scripts/test_blitz_tests_targets.py:1-30; as measured at
  > escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md)
  (.andromeda/architecture.md:21)
  > the `ci-scripts` leg needs Python 3.11 or later
- (.andromeda/architecture-detail/stack-and-technologies.md:72) The gates are read through the one leg runner the dev host and every linux job run, never a hand-built cargo line, so the chunk's green on the dev host is the same command CI runs.
  > a bash leg runner, `.github/scripts/ci-leg.sh`, that every linux job and the dev host run
  > (.github/scripts/ci-leg.sh:1-6)
- (.andromeda/architecture.md:247) The `doc` leg the chunk must pass is the docs job's `cargo doc --workspace --no-deps --locked` under rustdoc `-D warnings`, run as `bash .github/scripts/ci-leg.sh doc`, so every new helper's doc comment must build clean.
  > each of the thirteen linux jobs (twelve until 2026-10-14-quality-gates) runs its leg as `bash
  > .github/scripts/ci-leg.sh {leg}`, the same command the dev host runs, with `--locked` on every cargo leg but
  > `examples/wasm_hello` (.github/workflows/ci.yml:47; .github/scripts/ci-leg.sh:18-67); the docs job runs `cargo doc
  > --workspace --no-deps --locked` (.github/workflows/ci.yml:214; .github/scripts/ci-leg.sh:37), an effective rustdoc
  > `-D warnings` gate over every workspace library crate
- (.andromeda/architecture-detail/stack-and-technologies.md:48) The refactored listing test and any helper it gains read the `tools/list` answer through serde_json as a reader alone, adding no dependency or feature to escher-driver.
  > since 2026-10-10-mcp-surface serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by
  > escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares
  > (packages/escher-driver/Cargo.toml:17) —
  (.andromeda/architecture-detail/established-decisions.md:169)
  > `serde_json` (`{ workspace = true }`, no feature and no `[features]` table of its own, a reader of MCP requests, of
  > kept checks and of the live answer line a replay compares, writing nothing
- (.andromeda/architecture-detail/established-decisions.md:172) Every `Finding` the refactored `findings` walk and its helpers or table produce must stay stable ids, closed defect words and numbers, with no page text, colour, control value or `NodeId`, and add no span field or dependency.
  > a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the
  > closed defect words and numbers, no page text, colour, control value or `NodeId` — with no new carrier, exit, path
  > argument, cause, session error, span field or dependency: a ratified boundary widening;
  (.andromeda/architecture.md:94)
  > ids, names, masked values, snapshot text, diffs, shots and lint findings cross, never a `NodeId`; the library
  > depends on no app, emits no event, installs no subscriber and reads no env var;
- (.andromeda/architecture-detail/established-decisions.md:170) The refactor of `lint.rs` must open no span, emit no event, install no subscriber and read no env var, so `lint`'s `command` span still records `verb` and, refused, `cause` alone.
  > it opens one `tracing` span per call handed to `Session::run` and, since 2026-10-10-mcp-surface, one `tool_call`
  > span per MCP `tools/call` request, holding `tool` and `ending` only (the founder, 2026-10-10, «Span с tool и
  > ending», relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's), and
  > emits no event (§Standard Contracts → Driver session), installs no subscriber, reads no env var and has no default
  > state location;
- (.andromeda/architecture-detail/established-decisions.md:171) The listing test must keep asserting each tool's description as the `'static` table text — its help and its result fields — so a table walk or helper reads `VERBS` and never edits the schema's tables.
  > since 2026-10-12-self-description the command line's stdout and the `mcp` role's stdio also carry the program's own
  > fixed text, held as `'static` data of the one table — the `help` self-description, a session error's remedy and
  > each tool's description with its result fields — riding those ratified carriers, while text from a page, a call or
  > a screen, a new carrier or a new exit still widens;
- (.andromeda/architecture-detail/established-decisions.md:178) A kept check holds a `lint` answer whole and by its digest and replays it through `Session::run`, so the refactored walk must leave the `{"findings":[…]}` line byte-identical, in the same order.
  > each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's
  > without its pixels — so a kept check holds the screens' ids, names, values, snapshot text, diffs and a lint's
  > findings at rest, never logged and never named by a tool call; a kept check replays through `Session::run` alone
  > (`Check::replay`)
  (.andromeda/architecture.md:96)
  > A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable
  > check, never logged and never named by a tool call
- (.andromeda/architecture-detail/established-decisions.md:113) `stand_lint`'s re-asserted flight-booker reading — one `overlap`, `flight-type-one-way` × `flight-type-return` — rests on the builder keeping a select's options in the tree, so the refactored walk must still visit those option nodes and pair them in document order.
  > The accessibility builder keeps an option whose parent is a `select` although it computes `display: none` —
  > `hidden`, `inert` and `visibility: hidden` still exclude it —, writes its `selected` from its chosen state with no
  > `aria-selected` read, and takes its non-empty `label` attribute as its `label` where it has no `aria-label`
- (.andromeda/architecture-detail/established-decisions.md:104) The scope keeps `clear_interaction_state_for_removed_node`, the engine function behind this removed-node clearing, exactly as it was — no edit to it or its file's behaviour, its checks still green.
  > interaction state on removed nodes is cleared (tests/blitz-tests/tests/stale_interaction_state.rs:11-14),
  > hover/active retargeting to the nearest surviving ancestor and focus resetting to the body
  > (tests/blitz-tests/tests/interaction_state_teardown.rs:6-14)
  (.andromeda/architecture.md:70)
  > interaction state on removed nodes is cleared or retargeted.

## Bindings
- **stack-and-technologies (serde_json) ↔ established-decisions (driver session)**: the listing test's serde_json read and the driver library's serde_json are one rule — a reader of MCP output, writing nothing, no dependency or feature added (.andromeda/architecture-detail/stack-and-technologies.md:48) (.andromeda/architecture-detail/established-decisions.md:169)
- **established-decisions (lint findings on the carriers) ↔ established-decisions (kept check)**: findings stay closed ids, words and numbers and cross as the `lint` answer, and a kept check keeps that answer whole with its digest, so the walk's output and its order must stay byte-identical for a replay to match (.andromeda/architecture-detail/established-decisions.md:172) (.andromeda/architecture-detail/established-decisions.md:178)
- **stack-and-technologies (code quality) ↔ stack-and-technologies (CI/CD) ↔ the body's Infrastructure Patterns**: the `fast` leg runs fmt → clippy (both lines) → test → ci-scripts, the ci-scripts step holds the sweep's target pin, the `doc` leg is the docs job's rustdoc gate, and the `stand-repeat` leg is the leg script the refactored sweep reads for its witnesses (.andromeda/architecture-detail/stack-and-technologies.md:76) (.andromeda/architecture-detail/stack-and-technologies.md:77) (.andromeda/architecture-detail/stack-and-technologies.md:72) (.andromeda/architecture.md:247)
- **established-decisions (DOM semantics, select) ↔ the test plan's stand_lint pins**: the select-option nodes the builder keeps are the source of the flight booker's one `overlap`, the pinned reading the chunk re-asserts unchanged in both layout modes (.andromeda/architecture-detail/established-decisions.md:113)
- **established-decisions (DOM semantics, removed nodes) ↔ the scope's boundary**: the engine function behind removed-node interaction-state clearing is the one over-ceiling function the founder's word keeps as is (.andromeda/architecture-detail/established-decisions.md:104)
