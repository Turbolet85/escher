@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (nine verbs, the written forms; scope rows) — created the answer writer: `Outcome::to_json`, `Refusal::to_json`, `SessionError::to_json`, one line of JSON each, keyed by the schema's own words, hand-written and output only. The `json` module ("the written forms") joined escher-driver's private modules. This is the origin of `decision/hand-written-answers` and `decision/answers-written-by-own-json-writer`, which the split must keep: no serialization crate, output only, and the node, refusal and error shapes byte-equal.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` a reader only) — `serde_json` was taken as a reader of MCP requests only, so the written forms stay hand-written. `json`'s `string` / `Object` were widened to `pub(crate)` for `mcp`. This sets `decision/serde-json-reader-only`, and it shows that `json::string` / `json::Object` are cross-module callers whose `crate::json::…` paths must keep resolving after the split, by re-export where an item moves.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — the written forms gained `remedy` in the error object and `differs` on a changed node. `json::list` became `pub(crate)`. This is another cross-module primitive (`list`) the split must keep reachable at its old path, and it shows the snapshot/diff family's node writer carries `differs` after `bounds`, whose bytes must not move.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb; the scopes) — the shot's line `{"width","height","png"}` was added to the written forms. Its base64 is hand-written in the separate `image` module, with no base64 crate. This sets `decision/no-base64-crate` and `decision/png-encodes-shots`. The shot family (`shot`, `shot_without_image`) is one of the unnamed forms whose home P4 must settle, and the base64 stays in `image`, not `json`.

@history_item n=5 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the founder's word respelled) — this restated the `json` module's Existing Scopes wording as "the hand-written forms". It is the scope row that names `json`, and it follows the files when the writer is split.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the first unit-test module moved to a file of its own: `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;`, a child of the module under test, moved whole with its test paths unchanged. This is the root of `decision/split-test-module-as-child` and `decision/moved-unit-tests-keep-paths`, which `json.rs`'s ~1060-line `mod tests` must follow.

@history_item n=7 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — the direct precedent for this chunk's shape. `mcp` got three private children (`framing`, `listing`, `result`), each widened to `pub(super)` and no further, with its tests in `mcp_tests.rs`. `schema` got its child `spec`, with its shape types re-exported and its tests in `schema_tests.rs`. The crate's private-module count stayed fourteen, and no public item, crate module, test path or behaviour changed. Citations into the moved spans followed them to their new files. Children at `pub(super)`, the parent re-exporting, the module count unchanged and the citations following the files are the moves this chunk repeats.

@history_item n=8 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moved three more escher-driver test modules out (`cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs`), each `#[path]`-declared and moved whole with paths unchanged; the latter two are cfg-gated per platform. It also states that escher-driver's modules hold their tests in `#[path]` children "as `schema` and `mcp` do", with no module added to the public tree. That is `decision/driver-unit-tests-in-path-children`, the rule a `json_tests.rs` must satisfy.

@history_item n=9 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Conventions: 1 row) — `decision/unit-tests-at-file-end`'s except changed from "Eight unit-test modules sit in files of their own…" to an uncounted form, because a new `#[path]` test child broke the count. A new `json_tests.rs` (or family test files) needs no count edit in that row, but the row should still be read against the moved layout.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, the recording module) — `json` gained a signed `integer` writer, and escher-driver's private modules went from fourteen to fifteen. The `integer` primitive belongs with the shared primitives (`string`, `number`, `Object`, `list`) the split keeps in the parent or re-exports. The module count is restated in the escher-driver scope row, which the split will move again.

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest) — a kept check's `answer` is the line as the driver wrote it (crate-private `kept_line`), a shot's through `json::shot_without_image`. `digest` is FNV-1a 64 over that line, and `Check::replay` compares the live line with the kept `answer`. This is why the committed `counter-{cli,mcp}.json` checks replayed by `stand_replay` are the byte-for-byte witnesses of the split, and why `shot_without_image`'s caller in `recording.rs` must keep resolving.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — added the finding's written form `{"findings":[…]}`, each finding keyed `id` · `defect` · `bounds` then its reason's keys. These are the lint family's writers (`finding`, `bounds`, `ratio`) whose home P4 settles, pinned by `contract/answer-json/lint-finding` and `contract/lint-verb/wire-form`.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `to_json` writes a paged answer `{"text":…,"part":n,"parts":m}`, while a whole screen stays `{"text":…}`. This is part of the snapshot family the entry names, pinned by `contract/answer-json/paged-snapshot`, and its bytes must not move.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the `graph` verb; citation ranges re-read) — the graph's hand-written form `{"nodes":[…],"states":[…]}` was added, and escher-driver's scope names "json's node and state forms" (`graph_node`, `graph_state`, `write_readings`) as the verb's home in `json`. That is an unnamed family the split must place. The same chunk re-read every `file:line` range into `json.rs`. Trap: a move of `json.rs` content stales every master citation into it, so the wrap's pointer rows must follow the files.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read) — `Outcome::Actions` is written by `json::action_node`, with `owner` and `cause` left out when absent. escher-driver's private modules went to eighteen and its re-exports to 59. This is the actions family whose home P4 settles. The module count it restates moves again if the split adds child modules, while the re-export list must stay unchanged (`lib.rs`'s `pub use`).

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader; citations re-pinned) — `Coverage::to_json` writes `{"unexercised":[…],"checks":n,"held":m}` by hand (the `impl Coverage` that sits in `json.rs`), with `serde_json` writing nothing. Re-exports went to 62 and private modules to nineteen. The citation pass re-pinned ranges into `json.rs`. This is the coverage family the split must place, pinned by `contract/handler-coverage/written-form` and `contract/coverage-role/answer`.

@history_item n=17 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Standard Contracts: 6 rows) — added `contract/acting-verb-results/caused` and amended `contract/answer-json/outcome` so that `caused` is written last and only when the step caused something. The `caused` family the entry names starts here, and the measure that took `json.rs` from 1244 to 1861 code lines over Epoch 5 includes it.

@history_item n=18 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Standard Contracts, 1 of 2) — `caused` became one item per panic, error or record (message cut at 1024 bytes, location, boundary and chain, fields, first reached handler, 16 kept), and `caused_past` was added after `caused` only when entries pass the 16. These are the rows `contract/acting-verb-results/caused`, `caused-past` and `answer-json/outcome` that bind the `caused` family's writers (`with_caused`, `cut_text`, `caused_item`, `caused_past_item`) byte for byte.

@history_item n=19 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's `error` field now crosses as written, scrubbed only by the answer's own hidden field names (`decision/no-echo-of-typed-input`'s except, `contract/acting-verb-results/caused`). This is the latest edit to the `caused` writer's field handling, and the split must carry it unchanged.

@history_item n=20 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Standard Contracts 3 and 4 of 4) — added the layout answer's written form and its closed words: `contract/answer-json/layout`, `layout-placing`, `layout-shift`, `layout-size-cause`, `layout-length` and `layout-fallback` (a later engine-set member written `unexplained` / `transform` / `layout` / `auto`, never a panic), plus `decision/layout-answer-crossing`. These bind the `layout` family (`placer_word` … `layout_answer`) the entry names, and their fallback arms must move with no body edit.

@history_item n=21 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (CI gate legs, a real rustdoc gate) — the `doc` leg became `cargo doc --workspace --no-deps --locked` under `-D warnings`, the origin of `decision/rustdoc-warnings-are-errors`. The scope gates on the `doc` leg. Trap: an intra-doc link or a `pub(crate)` item named from docs can break when items move between modules.

@history_item n=22 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the per-crate coverage floor (escher-driver 97 %) and the `stand-repeat` leg with its zero-failure budget, the origin of `decision/per-crate-coverage-floors` and `decision/stand-repeat-zero-failure-budget`. The split must keep escher-driver at or above its floor when tests move between files. The stand checks, which hold the written forms, run three green rounds.

@history_item n=23 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading) — recorded the `rust-code-analysis-cli` reading on the dev host alone against a ceiling of 15 (`decision/cognitive-complexity-ceiling`). The ceiling is not a project convention, and functions over it stand untouched. A move-only split edits no function body, so it should change no function's reading, but the code audit's reading of `json.rs` is the measure this chunk answers.
