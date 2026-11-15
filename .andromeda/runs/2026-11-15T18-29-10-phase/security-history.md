@history_item n=1 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Dependency Security: 3 rows) `~ decision/no-git-dependency` and `~ decision/no-git-dependencies` gained the except «The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs», and `~ decision/upstream-sync-dependency-set` now admits git-pinned content only by a lock-only, semver-compatible update of a pinned registry crate. This is the exception this chunk exists to use: any second git source (a Taffy dependency from git, a `[patch]`) falls outside it, and the lock, audit and MSRV legs must still pass with git Taffy.

@history_item n=2 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the 28-name falsy predicate; no git dependency; the sync's dependency delta) Pinning moved taffy from git rev `7d33901c` (0.14.0) to crates.io 0.15.0 with its eight features byte for byte, leaving no git dependency in the root manifest, the founder's rule. This is the precedent being reversed: the move back to git must keep Taffy's feature set and record the lock delta the same way (no other third-party package entering, leaving or moving, the audit `advisories ok` with `deny.toml` unchanged).

@history_item n=3 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — (the 2026-10-18 sync record) Upstream's four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`, git Taffy `ea07c4c3`) were not taken, because of the no-git rule; the partial take was PROVISIONAL. That is the origin of the chain this chunk now takes, including `1a11761bc`'s `inline.rs` line.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) It ratified the cursor fix's «F1 answered B», the partial take that left out git Taffy. The prior Taffy refusal therefore stood on the founder's own word until the 2026-11-15 exception.

@history_item n=5 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — (the per-sync lock record of the second partial take) Taffy stayed the registry "0.15", with no git dependency, and upstream's Taffy-main commits (git Taffy `ea07c4c3`) were again not taken. This is the per-sync lock-record form this chunk's Pinning record follows.

@history_item n=6 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — (the `select` row, the labelable label, the PROVISIONAL discharge) It ratified the 2026-10-19 take that left git Taffy out. It also created the single-select row behind `decision/single-select-selectedness` and `decision/select-typeahead-private` (the 1 s typeahead, compared and never kept past 1 s or carried out), both picked rows that hold on every document.

@history_item n=7 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (the per-sync lock record of the ten-commit take, and tokio past RUSTSEC-2026-0386) Body Pinning now reads «or, by cherry-pick, entered no package — one of them moving lock edges alone»; a lock-only, semver-compatible tokio update fixed an advisory; the six Taffy-main commits were not taken. This is the precedent for reading lock edges name by name and for a lock-only update as the audit's remedy.

@history_item n=8 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`multi_doc`'s inputs, the attribute runner's unwrap, the traversal lock and the moved `inline` path) The inline-box height cite moved to `layout/inline/layout.rs`, and the aspect-ratio guard (clamped at 0) was restated. That explains `decision/inline-box-heights-finite`, whose code sits beside the `inline.rs` `place_floated_box` call this chunk edits.

@history_item n=9 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the per-sync lock record of the eleven-commit take) The pins stayed: Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency, every standing ignore encountered. These are the pins `decision/engine-dependency-pins` states, and this chunk moves the Taffy one.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (this take's lock record) The same pins were restated, Taffy the registry "0.15" and no git dependency, with the audit leg reading `advisories ok` and `licenses ok`. This is the latest restatement of `decision/engine-dependency-pins` that the Taffy move makes stale.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (dependency security after the sync: Parley's pin form, the entering crates, an ungated licence table, a second `unsafe`) Pinning named taffy as «the one git dependency, pinned by commit rev», and each entering crate was reviewed by hand for licence, repository, build script and proc-macro. Unsafe code: stylo_taffy holds two `unsafe` blocks, one in `resolve_calc_value`. Precedents: git Taffy pinned by rev, a hand review of each package the lock gains, and the stylo_taffy bridge that a Taffy API move touches.

@history_item n=12 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (unknown alignment flags; merge citation re-point) Error Handling: an unknown alignment flag maps to Taffy's `AlignContent::NORMAL` for content alignment, after upstream adopted Taffy's first-class `normal` keyword. This is the precedent that a Taffy move can change a security-plan error-handling claim; the no-panic guarantee must hold after this move too.

@history_item n=13 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (root-manifest citations re-pointed after the line-61 insert) The root `Cargo.toml` citations of the taffy and parley pins were re-pointed. This is a trap: Pinning cites the taffy line of `Cargo.toml` by number, so editing that line means re-pointing the citation.

@history_item n=14 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (signing and PR-token jobs repository-guarded; CI legs locked) `--locked` now covers every ci.yml cargo leg, the flake and `dx bundle`; `examples/wasm_hello` is excepted because it has no `Cargo.lock`. This is the origin of `decision/locked-builds` and its except: the scope's wasm_hello taffy resolution is read outside `--locked`.

@history_item n=15 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (cargo-deny audit leg, SHA-pinned ci.yml, a read-only ci.yml token) It set up cargo-deny 0.20.2 as the `audit` job over the root `deny.toml` (`[graph]` six platforms, all features), with per-ID ignores only, each with a written reason, and `unmaintained`/`unsound` at their defaults. This is the origin of `decision/cargo-deny-audit-tool`, `decision/advisory-ignores-by-id` and `decision/dependency-audit-ci-gate`, which a git Taffy must pass.

@history_item n=16 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (the whole-workspace audit, the licence gate, the owed checks discharged) The audit became `cargo deny --locked --workspace --all-features check advisories licenses`, the fork gates on licences, and memmap2 is kept out by the `unsound` default. This is the origin of `decision/dependency-audit-cargo-deny` and `decision/licence-gate`: a git-sourced Taffy must still pass the licence check.

@history_item n=17 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — (five advisory ignores: zip 2.4.2's three) Ignores are by ID with a written reason and a named owner, re-read on every push. This is the audit policy behind `decision/audit-ignores-per-id-with-reason`; the Taffy move may add no blanket allow.

@history_item n=18 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (eight advisory ignores: the zbus family's three; the 2026-10-17 sync record) It added the zbus-family ignores and the sync record «Taffy still the registry "0.15" with no git dependency». Its (security) criterion «with `deny.toml` unchanged» stayed UNMET after external decay. Trap: an audit can go red over an unchanged tree.

@history_item n=19 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the zip ignores gain RUSTSEC-2026-0429) A second external decay turned the audit red over an unchanged lock, fixed by an ID ignore with a reason. Precedent for meeting an advisory on this chunk's lock move.

@history_item n=20 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (pulldown-cmark 0.13.6 fixes RUSTSEC-2026-0433) A `Cargo.lock`-only, semver-compatible update with no coupled pin fixed an advisory. This is the lock-only-update shape the audit rule names as the remedy.

@history_item n=21 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (advisories published after a green audit) The local `fast` leg runs no audit, so `bash .github/scripts/ci-leg.sh audit` is run locally before a push. This is the origin of `decision/local-audit-before-push`, which this chunk's measurement and push must follow.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (RUSTSEC-2026-0414, the pulldown-cmark lock update) A lock-only update fixed an advisory with `deny.toml` unmoved. Another precedent for keeping the lock move confined.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` leg, locked) `cargo test -p blitz-tests --locked … --features blitz-paint/font-embolden --test select_popup_painted` is a locked leg last in `FAST_LEGS`. This is the origin of `decision/embolden-locked-fast-leg`, a fast leg the Taffy move must keep green (it reads a painted select list's layout).

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`visible_region` through transforms, and `off-screen`'s meaning) `visible_region` reads each clipping box through its transforms, exact under translate and scale, a rotation reading the bounding box, and a non-finite mapped edge answering `None`. Pins: `visible_region_transformed`, `stand_act_transformed`, `transformed_client_rect`. This is the origin of `decision/visible-region-through-transforms`, whose guards read layout geometry a Taffy move can shift.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a non-invertible transform takes no hit) `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, pinned by `hit_non_invertible_transform`. This is the origin of `decision/non-invertible-transform-no-hit`.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `select` row's rules, limits and pins) A select's options include its `optgroup` children's in tree order, and the `select_interaction` and `stand_act_select` pin counts grew. This explains `decision/optgroup-options-and-label`, `decision/single-select-selectedness` and their guards, layout-reading tests the move must keep green.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a `multiple` select's keys, clicks and chosen set) It added `press_multiple_select_key` and `click_select_row`, and every choice passes `apply_select_choice`; Ctrl+A is not modelled. This is the origin of `decision/multiple-select-keys-and-clicks` (an engine change for every document and a boundary widening, ratified).

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`optgroup` and its `label`) A group takes `Role::Group`, and its non-empty `label` is its name in the tree. Part of `decision/optgroup-options-and-label`.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`optgroup`'s `label` painted as a header) A group's label is now also painted as a header in an open list, so it leaves as pixels in a shot, pinned by `select_popup` and `select_popup_painted`. Completes `decision/optgroup-options-and-label`; the painted list's layout is what the embolden leg reads.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (which select opens a list, what a row takes, and hostile markup) A hostile `size`, 2,000 options and a 5,500-character label keep every box finite and panic nowhere, asserted by `hostile_markup_keeps_every_box_finite_and_panics_nowhere`; at most `MAX_ROWS` 20 rows show. This is the origin of `decision/select-list-hostile-markup-finite`, a finiteness guarantee a new Taffy layout must keep.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`size` on a `multiple` select) `size_rows` is capped at 65534 and hints a height of `size` × 1.2em, pinned by the hostile-markup test. This is the layout input behind `decision/select-list-hostile-markup-finite`.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list's state, keys, pointer and close) It added the one closed key match `press_open_list_key` and the one write path `apply_select_choice`, and a box move closes the list through `reconcile_select_list`. This explains `decision/select-keys-closed-match` and `decision/select-choice-one-write-path`; their guards (`select_popup`) are sensitive to box geometry.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the `select` row: Enter, Space and a click open the list) Enter, and a Space with no typed text standing, open a single select's list, through the closed key match `press_select_key`. Part of `decision/select-keys-closed-match`.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list's visible region, paint and scroll) `visible_region` answers an open list row's scrollport, a non-finite box is left unpainted, and the list's offset is held within its rows (`Geometry::clamped`). This is geometry beside `decision/visible-region-through-transforms` and the hostile-markup finiteness rule.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (culling on the scaled size, the inline box's paint skip) Geometry keeps its cull of non-finite or beyond-f32::MAX bounds; a box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` paints no box of its own. This is the finiteness backstop for layout values a Taffy move produces.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the key-press panic retired) `deallocate_anonymous_block` clears `layout_parent` on its children so the id does not go stale, witnessed by `key_press_after_container_lost_its_box` and `select_popup`. This is a stale layout-id precedent behind `decision/stale-node-ids-never-panic`, in the layout-tree code a Taffy API change can touch.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the key-press panic after a hidden ancestor) A remote page could reach a panic in `position_in_containing_block` through a dropped anonymous block, a layout-tree stale reference (fixed 2026-11-01). Trap for a layout-engine move: stale layout links are a remote-reachable panic class.

@history_item n=38 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the in-process lint as an id reader, and the text extent reader) It added the Layout values row: `BaseDocument::text_extent` is read-only, and an unresolved id or a node with no inline layout data reads `None`. This is the origin of `decision/text-extent-read-only-none-when-unresolved`, an inline-layout reader beside the `inline.rs` code this chunk edits.

@history_item n=39 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — (a password's snapshot value is masked; the falsy clear and the `disabled` reader recorded) A password input's snapshot value reads `MASKED_VALUE`; the `dangerous_inner_html` citation was re-pointed onto its `set_inner_html` branch. This is the origin of `decision/password-file-value-masked`, and it explains `decision/dangerous-inner-html-parsed-unsanitized`.

@history_item n=40 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — (the snapshot has one text form; a file input's value is masked) The mask extends to `type="file"`; the DOM attribute and form submission still hold the host path. This is the except of `decision/password-file-value-masked`.

@history_item n=41 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the engine's password mask, nine causes, no image in a log, `png`) blitz-paint paints a password's value as bullets laid out by `layout_as_text_input`; two values of one length paint byte-identical frames. This is the origin of `decision/password-painted-masked` (guard `password_painted_masked`), a layout-dependent pixel check the move must keep.

@history_item n=42 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§Input Validation: `disabled` on form controls, the password caret by the bullets) `click_password_input` lays out bullets through `layout_as_text_input`, so no real character width reaches the caret or a shot. This is a layout-built guarantee beside `decision/password-painted-masked`.

@history_item n=43 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the MCP server's input and its crossing, ratified) The masked values gain MCP results as an exit, by construction. This is the start of `decision/mask-holds-on-every-carrier` (guards `stand_act_filled`, `stand_mcp_masked`).

@history_item n=44 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (three state readings inside the ratified carriers; the closed-field rule) It set the founder's rule: a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is. This is the origin of `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`, the test the scope's fork rule applies to whatever the Taffy chain brings.

@history_item n=45 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the kept check at rest, its directory, its reader and `--record`) A kept check is `0600` in a `0700` launcher-named directory, never logged, and removed only when its own recording ends. This is the origin of `decision/kept-check-owner-only`.

@history_item n=46 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (each kept answer whole at rest, the committed checks, the replay) The committed `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` are compared byte for byte with a fresh recording, and each kept answer is held whole. This is the origin of `decision/committed-counter-checks`. Trap: a Taffy layout change that moves any bounds in an answer breaks the committed checks' byte equality.

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the shared target check and the actions read) One crate-private `check_target` serves the four id-taking arms and `Session::actions`; the read runs nothing and changes nothing. This is the origin of `decision/actions-read-shares-target-check` and `decision/actions-read-changes-nothing`; `off-screen` and `covered` causes read layout geometry.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb's validation and its answer lines) The actions answer holds ids, owner ids, verb and cause words and `keyed` booleans, never a value or `NodeId`. This underlies `decision/kept-check-actions-answer`.

@history_item n=49 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the kept actions step's replay now exercised) A kept `actions` step replays, guarded by `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes`. This is the guard of `decision/kept-check-actions-answer`; a layout-moved cause would stop a replay.

@history_item n=50 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — (Data Protection: 1 row) `~ decision/kept-check-actions-answer`, state: provisional → standing, after the founder ratified the `keyed` field.

@history_item n=51 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage line's content and its silence in the logs) The coverage line holds ids, owner ids, event names and counts only, guarded by `stand_handler_coverage`. This is the origin of `decision/coverage-line-no-user-data` and the `contract/coverage-line/*` rows (checks, held, unexercised).

@history_item n=52 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified — the coverage line's content and its silence in the logs) The PROVISIONAL mark on the coverage line entry was replaced by the founder's ratification. The coverage-line rows stand ratified.

@history_item n=53 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — (a sink-installing host logs no id and no name at any level) Engine targets print only safe fields and any other target's record is dropped whole. This is the basis of `decision/no-screen-content-in-logs` (guards `host_log`, `mcp_log`, `stand_act_spans` at `RUST_LOG=trace`). A git Taffy logging under a new target is dropped, not printed.

@history_item n=54 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (escher's log sink scrubs) `stylo_taffy` is among the engine targets that print only the safe fields. This bears on any log site the Taffy chain's blitz/stylo_taffy edits bring.

@history_item n=55 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (Input Validation: 2 rows) `+ decision/one-stamping-request-constructor`: blitz-dom builds a sub-resource request only through the stamping constructor, a clippy disallowed-methods rule upstream lacks, so it is a new upstream merge surface. Trap: the Taffy chain's blitz-dom commits (`35cdfc859` · `930448918` · `b4952e15f`) must not trip that clippy gate.
