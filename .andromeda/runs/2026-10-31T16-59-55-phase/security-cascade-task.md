# Task — security · 2026-10-31-select-popup

## The chunk
A closed single `select` opens a painted list of its options, in every document and renderer, as a new engine overlay in paint, hit testing and keys.
A person or an agent chooses an option by pointer and by key through the one write path `apply_select_choice`, an option can be acted on through the driver, and the list closes without a choice.
An open list's options read real boxes, so the snapshot, the lint's `overlaps` exclusion and the `covered` refusal's closed-option reading and schema text are restated.
A group's label and `size` are painted or read here, and the focus and key contract, reduced motion and engine colours go to the founder as P4 forks.
`.gitignore` gains `escher-0.4.0-incubator/`, and the wrap halts once on the Playbook Boundary widening as the founder's answer card.

## Reached files
- packages/blitz-paint
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/events/keyboard.rs · edits Enter, Space, arrows and typeahead on a single select
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/assets/default.css
- packages/dioxus-native-dom/src/snapshot.rs · edits `snapshot`'s option `bounds`
- packages/escher-driver/src · edits `execute`, `lint`, the `covered` refusal's reading and schema help
- .gitignore · edits its incubator lines

## Needs
- The remote-content surface: the overlay shows and takes input over page-authored options in every document, remote pages included, so a validated surface admits new input and pixels; the wrap's Boundary widening halts on this.
- The select control's rules under Input Validation's markup attributes: a choice from the list reaches the select only through `apply_select_choice`, by pointer, key and driver alike, with no second writer.
- `disabled` and `inert`, read by presence: the chunk's key and pointer code must act on them on the select, its groups and its options when opening the list and when choosing a row.
- Newly read markup: `size`, if this chunk reads it, falls under the markup numeric rule of a parse with a default fallback; a group's label is page text, by a reader with no panic path, leaving escher only by the ratified carriers.
- The list's geometry under the paint, layout and scrolling values: non-finite geometry culled, oversized radii scaled, scroll targets clamped to the range, for a list of any option count or a select near a viewport edge.
- Graceful degradation over the open list: a select or option removed or changed by a page while its list is open must resolve through `get` to nothing and be skipped, never a panic on a stale id.
- The engine panic paths the overlay's code must avoid: `layout_style()` on an unstyled node, `layout_data` on a node kind lacking it, an inline root without inline layout data at paint, and the `expect` invariants in `events/keyboard.rs`, for options that hold no layout while closed.
- The keyboard and pointer code around the list: Enter on buttons and links, Space on buttons, the copy-to-clipboard guard in `events/keyboard.rs`, the file input's path write and the devtools hover mode in `events/pointer.rs` stay as they are beside the new branches; the contract for keys with a list open is the founder's fork.
- A `multiple` select's own rows, keys and clicks stay as built by 2026-10-31-engine-debts, whichever way `size` goes.
- `DioxusDocument::snapshot`'s option `bounds` change from the select's client rect to real boxes while open; the snapshot, its text and its diffs still read a password's or file input's value as `MASKED_VALUE`.
- The open state in the accessibility tree through `aria-expanded`'s `expanded`: ARIA states pass closed parses carrying no attribute text, and the driver socket's widening rule decides whether a new closed-value field needs a boundary ruling.
- The carriers an open list's data travels: the socket's replies, the command line, MCP results, kept checks, screenshots and a lint's findings, with state readings as closed words and booleans, never an attribute's text; any new carrier, exit or authored-text field is named on a fork.
- The stable element id of an option in the open list: acted on by that id alone, no author `id` able to make two equal, no `NodeId` kept between calls or crossing a carrier.
- Driver acting on an option: through `validate` and then `Session::run` alone, over the existing closed table of twelve verbs and seven argument kinds, an id that cannot take the call refused with nothing run; the chunk adds no verb, node key, cause, session error, exit status, carrier, listener, port, env var or workspace crate.
- The `covered` refusal's restated reading and schema text: `Refusal` stays typed, one of the eight causes with a fixed name, meaning and remedy, no `String` field, no path and nothing a call supplied.
- `execute`, `lint` and the schema's code in `escher-driver`: no `unwrap`, `expect` or panic on anything a call supplies, and no clock read by the schema or its validation.
- The lint's restated `overlaps` exclusion (F9) over options with boxes of their own: its findings stay within what the carriers already ratify.
- Kept checks over the changed answers: a step on an open list records and replays through the strict `Check::from_bytes` and `validate`; whether the snapshot's change reaches the digests of the committed `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` is for P3 to read.
- The stand's `select#flight-type` and the stand tables that read it, `stand_contrast`, `stand_keyboard`, `stand_act_select` and `stand_lint`, which move with any new row; `stand_contrast` reads through blitz-test-harness's contrast reader, whose `color` source and no `tracing`, no `style` rule stand.
- Engine colours as a new default in `assets/default.css`, the engine holding no tokens: the declarations pass the CSS parsing rule like any other, and their values are the founder's fork.
- `dioxus-native`'s windowed path shows the same list from the engine; a separate OS window is not assumed, so no new window-creation or shell panic site is taken on.
- `.gitignore`'s edit adds the one incubator line and keeps what it holds, `target/` among it, the home of the unscrubbed `run.log` and transcripts.
- The CI gates the chunk must pass (17 checks at the take-up) and the watch on the free-disk step in `Stand checks repeated`, retiring on a recurrence or 3 green runs.

## Standing conventions
- S1 · A new log line or span in an engine or `escher_` target holds fixed words, bools and counts, never option text or an attribute's text, and passes `escher_telemetry`'s target allowlist · Logging & Monitoring, .andromeda/security-plan-detail/logging-monitoring.md
- S2 · A new test or test file that reads an env var or the temp directory is recorded in the environment values list, as test-only and not secret · Secret Management, .andromeda/security-plan-detail/secret-management.md
- S3 · A new check a test keeps stays owner-only and uncommitted, the two committed stand-Counter checks being the one exception · Data Protection, .andromeda/security-plan-detail/data-protection.md
- S4 · Any dependency a test or fixture would take inherits `workspace = true` and enters no package to `Cargo.lock` without the audit leg's `--locked` read · Dependency Security, .andromeda/security-plan-detail/dependency-security.md

## Not needed
- Outbound requests, TLS, the HTTP cache and cookies: the list fetches nothing.
- The script runtime's fetcher, module imports and JS API argument checks: the list adds no native API and no script path.
- Application authentication: no login, token or peer check enters, and the absence findings do not move.
- CI signing material, the upstream-guarded publish and WPT workflows and their secrets, and the operator's Claude Code login: no job or secret source is touched.
- The browser app's urlbar, sqlite history, favicon, error pages and about pages: none is a part of the select.
- rdme's markdown rendering and the examples' command-line arguments (`screenshot`, `paint_bench`, `preact_script`, `multi_doc`): no entry point of theirs changes.
- The WPT runner's quarantine, net provider and report paths: the chunk does not edit the runner.
- `agent-run.sh`, `cold-agent.sh`, `apt-install.sh` and `coverage_floor.py` and their argument checks: the scope edits none of these scripts.
- The session state directory's modes, the session label rule and the socket path bound: lifecycle is unchanged.
- Fetched images, fonts and SVG, and `file:` and `data:` URL decoding: the list paints options' text from the document already parsed.
- Upstream sync history, its lock reads, the vendored Preact copy and the third-party CDN links in fixtures: upstream sync is the next epoch's.
- Non-adjacent `multiple` sets through the driver: a residual outside this chunk, `press` holding no Ctrl.
