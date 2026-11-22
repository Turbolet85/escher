@history_item n=1 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading beside the mutation tier) — landed decision/cognitive-complexity-ceiling's ground: `rust-code-analysis-cli` 0.0.25 on the dev host alone, never in CI, ceiling 15, and on the operator's Q2 answer explicitly no project-wide convention, so 85 functions over it stood untouched. This chunk's three re-measured scores (19 · 46 · 167 against upstream at `547d7514b`) use that tool, and the standing rule its wrap lands as a decision row sits beside that row, the first convention-like rule hung on the ceiling.

@history_item n=2 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and button arms as private named functions) — the direct precedent for part 2: `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` kept their signatures while escher's range and button arm bodies moved to private functions (`press_range_input_key`, `press_range_input`, `range_track_fraction`, `click_button`), with no behaviour changed, "the parents left above the ceiling at their upstream floor so the next upstream sync's diff stays out of arms escher does not own". Gates, event order, counts and the range-focus block inside `handle_click`'s walk stayed in place. This chunk extends the same pattern to one call per upstream body.

@history_item n=3 marker=2026-10-16-file-splits
**2026-10-16-file-splits** (five out-of-line test modules; mcp's and schema's child modules) — the precedent for splitting an escher-driver module along its seams with no public item, crate module, test path or behaviour changed. `mcp` got three private children widened to `pub(super)` and no further, `schema` moved its shape types into a private child and re-exported them, and tests went to `#[path]` `cfg(test)` children with their paths unchanged. The crate's private-module count stayed the same. The `command.rs` split should follow this shape (decision/split-test-module-as-child, decision/driver-unit-tests-in-path-children, decision/restricted-crate-internals).

@history_item n=4 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (eight `#[path]` test children) — moved escher-driver's `cli::tests` and `recording::tests` out whole into `#[path]` children, test paths unchanged, with no module added to the public tree and the scope row unchanged. This is a further precedent for where `command.rs`'s tests go if the split moves them.

@history_item n=5 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Conventions: ~ decision/unit-tests-at-file-end) — the exception's fixed count of `#[path]` test modules was dropped for an uncounted form. A new test child from this split therefore needs no row amendment.

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — introduced the private `command` module beside `refusal` and `schema`, under no `cfg` gate. It holds `validate(call) -> Result<Command, Refusal>`, which receives no `Session` and answers in a fixed order (verb, then the call's arguments as passed, then the verb's arguments in schema order), and `Command`, `Key`, `Call`, `ArgValue`. Trap: `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`, so nothing may field them. The crate root's `Command` is the schema's while `start` takes a `std::process::Command`, which matters for re-export paths after the split.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — set the rule (decision/validated-commands-only) that nothing public runs a `Command` that did not come from `validate`, and nothing resolved outlives a call. A split of `command.rs` must keep construction of a `Command` reachable only through validation, with no item gaining visibility outside the crate.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (Driver session contract: nine verbs in two levels, the written forms, `command_line`) — `command` gained `SessionCommand` and `validate_session`. A session-level verb is refused `unknown-verb` by `validate` before its arguments are read, and `validate_session` checks the session rows by the same rules. These are re-exported names the split must keep resolving at `escher_driver::…`. The same entry set "one verb table: no second list exists".

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — `SessionCommand` gained `Help`: `validate_session` admits it and refuses an argument `malformed`, while `validate` refuses it `unknown-verb`. This is part of the validation behaviour the driver's schema and CLI checks pin and that the split must not move.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors, the recording module) — the crate-private `call_of` moved into `command.rs` (cited `command.rs:32-60`, was `mcp/result.rs`), so command holds MCP argument reading as well as validation. Citations into `command.rs` exist in the masters and will move with the split.

@history_item n=11 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Command::Snapshot { part: Option<u32> }`, with a `part` admitted in `1..=MAX_PART` through `u32::try_from`, any other number `OutOfBound` and a text or flag `WrongKind`. This is validation logic in `command.rs` that contract/validation-order/first-failure and the refusal-fault rows pin.

@history_item n=12 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (fourteen verbs, actions the tenth instance verb; the shared per-verb target check) — `Command` gained `Actions`, validated from `("actions", [])`. The actions read validates each call `call_naming` spells, then the crate-private `check_target`, the one per-verb target check. This is part of what grew the command module; the actions read is a registered contract (contract/actions-verb/acceptance) that the split must keep calling the same validation.

@history_item n=13 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (graph `part`; `press --ctrl`) — `GRAPH.args` gained an optional `part` (ArgKind::Part 1..1000) and `PRESS.args` gained the `ctrl` flag. These are the 72 lines that took `command.rs` from 776 to 848 code lines and over the 800 bar (scope CARRY cry-p3m7lb6q). The rows contract/graph-verb/arguments, contract/driver-verbs/press and contract/command-line/verb-flags pin this validation.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader `handler_coverage`) — the crate re-exports 62 names (was 59), and escher-driver holds nineteen private modules. This is the latest measured baseline of `escher_driver`'s public paths, which part 1 must keep resolving unchanged.

@history_item n=15 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — `run`'s former body became the private `Session::execute`, with the `command` span entered before `validate` and held until `run` returns. That span was built by hand (decision/driver-hand-built-spans) and fields only the eight closed values, never a `Command`, `Call` or `ArgValue`. Moving validation code must not add a span or a log site; the chunk forbids a new span or log site.

@history_item n=16 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the first escher change to `build_accessibility_tree` and its node-naming. It added a trimmed `aria-label` and a bound `<label>` beside the direct text children, and the `Document::accessibility_tree` override. This is the start of escher's share of the builder's text, which part 2 must tell apart from upstream's.

@history_item n=17 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the accessibility builder reads three state attributes; a third direct engine change) — escher's `aria-selected`, `aria-pressed` and `aria-invalid` reads went into the builder, flagged upstreamable (`accessibility.rs` +57 −1 against upstream). This is a measured instance of escher's share in the file. Rule set: a later attribute read keeps a closed set with no text carried out.

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads) — the merge took upstream's `aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled` and `aria-level` reads. It dropped upstream's own bool-parsed `aria-selected` and unfiltered `aria-label` reads so each attribute is read once (decision/each-aria-attribute-read-once). Trap for part 2: escher's builder also deletes upstream lines, so "upstream's text plus one call" may not be reachable for `build_accessibility_node` without restoring upstream's dropped reads, which would change behaviour. This is the scope's premise fct-n7aqpp47.

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the toggle per role in the builder) — the builder reads `aria-pressed` on every role but the five checkable ones, `aria-checked` on those five alone and neither on a native checkbox or radio, with `"switch"` mapped to `Role::Switch`. These are escher additions inside `build_accessibility_node`, pinned by `accessibility_states` and `accessibility_merged_states` (decision/toggle-read-per-role). The same chunk's `ElementData::is_disabled` is the one disabled reading that Enter/Space activation in `handle_key_or_input_event` reads (decision/one-disabled-reading).

@history_item n=20 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select model) — added three escher pieces this chunk touches. In the builder: a select option's `label` as a fifth name source, and `aria-selected` read on every element but a select's option. In the keyboard path: a focused select's arrows, Home/End and typeahead through `apply_select_choice`. It also added the `Change` event. All are direct engine changes, flagged upstreamable, so the builder and keyboard bodies hold more escher text than the 2026-10-16 split left.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native control's toggled; optgroup; a multiple select's keys; the tree's focus fallback) — `build_accessibility_node` writes `toggled` from `checkbox_input_checked()` under a checkable role and gives an `optgroup` `Role::Group` named by its label. The keyboard path gained `press_multiple_select_key`, and the tree falls back to the window for an unbuilt focus. These are escher additions in two of the three functions, and the 2026-10-31 eight engine changes are pinned by `accessibility_states`, `select_interaction` and `stand_act_select`.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's default actions; a single select's `expanded`; `size` on a multiple select) — keyboard additions: Tab runs `choose_current_and_close`, `press_select_key` opens on Enter or a Space with no typed text, and `press_open_list_key` is the open list's closed key match. In the builder, a single select reads no `aria-expanded` and writes `set_expanded(open)`. A multiple select's `size` hints a height through `size_rows`. These are escher additions inside all three functions, and the select-list merge surface is sixteen files.

@history_item n=23 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (contract/accessibility-tree/active-descendant) — the builder names an open single select's current option as active descendant and never reads a page's `aria-activedescendant`. This is a further escher addition in `build_accessibility_node` that the a11y checks pin.

@history_item n=24 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — the most recent escher additions to all three functions. Keyboard: a sized single select takes its own list-box keys while a drop-down keeps the key model. Builder: `expanded` is written on a drop-down alone. Presentational hints: a single list box's chosen row takes the `#c8c8c8`/`#000000` pair as presentational hints, an escher addition inside `compute_presentational_hints`. Restyle: a `size`/`multiple` write restyles the select's descendants because both decide the rows' hints. The guard is `the_display_size_decides_list_box_or_drop_down` (decision/single-select-list-box, decision/multiple-select-chosen-row-colours).

@history_item n=25 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range keys were escher's first addition to `handle_key_or_input_event` (arrows step, Home/End go to min/max, other keys ignored, through `apply_range_move`), flagged upstreamable with its merge surface named. Behaviour pinned by `range_input_interaction` and `stand_act_range` (decision/range-keyboard).

@history_item n=26 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation) — Enter's activation Click comes from `handle_key_or_input_event` after the text-input and range arms, through `dispatch_activation_click`, queued so the app's handlers run first. Space goes through `handle_key_up`. The order of arms inside the upstream body is part of the behaviour (decision/keyboard-activation-enter, contract/event-dispatch/enter-click), pinned by `stand_keyboard` and `stand_act_keys`.

@history_item n=27 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — added focus fixups after layout and "Tab skipping what does not show" as direct engine changes for every document (decision/hit-focus-form-engine-changes). A further escher change on the key path that the moved keyboard additions must preserve; it amended contract/event-dispatch/focus-change.

@history_item n=28 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the merge surfaces read at `ebacd68bd`; six upstream behaviour changes ratified for this take) — the merge-surface record for keyboard activation states upstream's taken commits left `events/keyboard.rs` untouched. It also fixed the rule decision/upstream-behaviour-changes-ratified-per-take: an upstream change is not counted among escher's direct engine changes. Part 2's premise is reading upstream's text at `547d7514b` to tell escher's share from upstream's.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner at `e42ee1f25`) — at that sync the twelve taken commits reached the range and keyboard surfaces only in `node/element.rs`, again leaving `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched. Each engine change is flagged upstreamable and nothing is sent. This is the merge-surface bookkeeping that the "one call in upstream's body" form is meant to shrink.

@history_item n=30 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — upstream was taken by a two-parent merge (decision/upstream-sync-by-two-parent-merge replacing the cherry-pick rule) with Stylo 0.24. Events now queue on the document (decision/events-queued-on-document, retiring decision/dispatch-event-callback), so `handle_key_or_input_event`'s upstream shape changed at that merge. Part 2's diff must be read against upstream's own text, since blame across a merge misattributes (scope premise, Tier 3 learning).

@history_item n=31 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `clippy` leg's second line, upstream's `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets -D warnings` (decision/blitz-dom-feature-subset-clippy). It also set the coverage floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 % (decision/per-crate-coverage-floors). Escher's new blitz-dom helper functions must compile clean under that feature subset, for example `accessibility`-gated code. The `command.rs` split must keep escher-driver's 97 % floor.

@history_item n=32 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the keyboard-types 0.8 surfaces) — the driver's key types come through the harness's `Key`, `Modifiers` and `NamedKey` re-exports (decision/key-types-through-harness), and `keyboard_key` maps the named keys. The coverage leg excludes `keyboard_compat.rs`. These constraints hold on any driver code the split moves.

@history_item n=33 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Stack and Technologies: + decision/mutation-tier-excludes-demo-shells) — the precedent for a code-audit rule landed as an architecture decision row, "the code audit reads its extra exclusions only from an architecture decision row". This is the form the standing rule this chunk's wrap lands beside decision/cognitive-complexity-ceiling takes (scope CARRY cry-6gvl75lk; no master row in this chunk's plan).

@history_item n=34 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs named accessibility targets on one `cargo test --workspace --locked` line (contract/ci-leg-legs/a11y; now eighteen per 2026-11-18-a11y-re-read). These are among the existing checks the plan names as proving that `build_accessibility_node`'s moved additions behave the same.

@history_item n=35 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Stack and Technologies: ~ decision/local-pre-push-gate) — the fast leg (fmt, clippy, test, ci-scripts, embolden) is the local pre-push gate, and on a dev host it ends with the target sweep. The chunk's gates run through it once the operator's quiet window allows cargo.

@history_item n=36 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (Infrastructure Patterns: + decision/cargo-under-verified-skia-url) — every skia-building cargo call, the fast leg's sweep included, runs under the verified `SKIA_BINARIES_URL`. This is the gated chunk this one feeds: its run reads v040-08 from this chunk's re-measured readings at its HEAD.
