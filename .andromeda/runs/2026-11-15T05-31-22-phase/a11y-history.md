@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created the CI job "Accessibility (a11y) tests" running `bash .github/scripts/ci-leg.sh a11y` in the slow tier with no required status check (`decision/a11y-ci-gate`, `decision/a11y-ci-leg`). This is the a11y leg the sync must keep green; widen nothing and add no required check.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (both entries: merged-file citations, and the root-manifest line-61 re-point) — the first upstream merge re-pointed 16 `file:line` citations by the merge's measured line map and fixed root `Cargo.toml` citations the insert had left one line low. Precedent and trap: citations into merged upstream files and the root manifest move with every take and have to be re-pointed against the cited text.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — measured the obs log format for escher's sink: the allowlist prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` for `blitz*` and `accesskit_xplat` targets. This is the §3 note behind the chunk's sink-allowlist re-check: a new upstream `tracing`/`log` site under a `blitz*` target reaches the sink through that allowlist.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — restated the sink as a target allowlist: it redacts `text`/`value`/`html`/`attrs` for `escher_*` and drops whole any record from another target, at every level. The re-check of the taken delta's new call sites and targets reads against this rule.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — §3 note restated as two record classes, printed events and closed spans of an admitted target (ratified 2026-10-09). A new upstream span under an admitted target writes a closed-span line, so the allowlist re-check covers spans as well as events.

@history_item n=6 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the workspace takes the Dioxus crates with `default-features = false`, so a crate gets accessibility only by naming it (`decision/dioxus-accessibility-feature-by-name`). The shell builds and refreshes the tree through `Document::accessibility_tree`, so a wrapper override reaches it (`decision/accessibility-tree-through-document-method`, `decision/a11y-tree-through-document-trait`). It also set `author_id` = stable id and the aria-label / label name sources. A merge must not regress the feature wiring or the trait path.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility`, so both seven_guis binaries compile the tree override but neither builds a platform adapter (`accesskit_xplat`/`accesskit_winit` in neither graph). This is the basis of `decision/stand-builds-no-platform-adapter` and `absence/assistive-technology-reach`, which every take re-reads.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — sync precedent: upstream's engine change (`autofocus` read `!= "false"`) was recorded as an engine change for every document, and the stand's graph was re-read on the merged lock (the accesskit family moved, still no adapter). Template for listing this take's behaviour changes and re-reading the adapter probe.

@history_item n=9 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — sync precedent: the bridge took upstream's private 28-name `is_bool_attr` and retired its own list, so a falsy `inert` is removed. `mutation_writer.rs` is byte-identical to upstream's, so later takes merge it cleanly. Keep it so.

@history_item n=10 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — sync that took upstream's six builder reads as written (`aria-labelledby` outranking, `aria-describedby`/`aria-description` to the adapter, `aria-expanded`, `aria-disabled`, `aria-level`) while keeping the three closed-set state reads and the trimmed `aria-label` read, one read per attribute. Source of `decision/aria-labelledby-outranks-label`, `decision/aria-description-to-adapter`, `decision/one-read-per-aria-attribute`, `contract/aria-property-reads/*`. Trap: a merge of the accessibility builder must not reintroduce upstream's boolean-parse `aria-selected` or unfiltered `aria-label`.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states entry) — the builder writes selected/toggled/invalid from ARIA attributes on every element, each value trimmed and matched in lower case (`decision/aria-state-lower-case-closed-values`, `decision/aria-states-on-every-element`), and the snapshot takes them from the tree. These are among the "accessibility state reads" the scope says every resolution keeps.

@history_item n=12 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (toggle per role; `inert` and `disabled`; the ratification batch) — `aria-checked` on the five checkable roles and `aria-pressed` elsewhere, `role="switch"` → Switch, `disabled` by presence on form controls, `inert` taking focus, hits and the tree, all ratified 2026-10-18. Rows `decision/toggle-read-by-role`, `contract/aria-role-mapping/role-switch`, `decision/disabled-by-presence`, `decision/inert-subtree-excluded`, `contract/hit-testing/inert`. The take's conflict resolutions in the builder, hit test and focus code must keep these.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (native checked; focus fixup; multiple select; optgroup; non-invertible hit; stand graph re-read; leg tally) — engine models the merge must keep. A native checkbox/radio writes toggled. The focus fixup reads `is_inert()` in `DocumentMutator::flush`. `multiple` and `optgroup` are modelled. A non-invertible transform answers `None` from `Node::hit_inner`, the hunk the scope's carried merge surface names as a textual conflict with upstream `0980cce5f`. The adapter probe was re-read on a lock move. The a11y leg reads 14 files.

@history_item n=14 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model: the changed set marked on a changed choice, option `selected` from the chosen state, the `display: none` exception for options, the keys, typeahead on the document's clock, `input` then `change`. Rows `contract/accessibility-tree-states/select-option`, `contract/single-select/*`, `contract/select-closed-list/*`. This is "the select model" every resolution keeps.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (select list entity; changed set where a list stands; `expanded`; keys/clicks; colours; no motion; `:open`/clips/`opacity`) — the select list is an engine overlay on every document, answered first by `hit_with_scrollbar` and painted by `paint_scene` after the root. It added the `reconcile_select_list` / `close_select_list` changed-set marks, single-select `expanded` from engine state, and engine-constant colours. Rows `decision/single-select-engine-overlay-list`, `decision/changed-set-marks`, `decision/single-select-expanded-from-list`, `decision/select-list-colours-engine-constants`, `decision/select-list-no-motion`, `decision/open-select-list-not-open-state`, `contract/driver-click-refusals/covered`. Upstream's SVG paint clipping and isolated-layer commits (`d176ac4e6`, `beaa796f4`) touch paint near `paint_scene`. The list must still paint last, past clips and `opacity`.

@history_item n=16 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — sync precedent: upstream's dropped-node hit skip was taken as an engine change for every document. The stand's graph was re-read on the take (no package in or out of `Cargo.lock`, no adapter). Same re-read owed now.

@history_item n=17 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files over `cargo test --workspace --locked`, pinned one `assertIn` per target word, eight also in `stand-repeat`. The leg the scope requires green, with its counts as the baseline the per-OS re-read compares against.

@history_item n=18 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — membership of the stand-repeat `--test` list is held jointly by `stand_requirement_sweep`'s leg check and `test_stand_repeat_leg_runs_three_rounds`. A name dropped from one leg alone is invisible to the sweep. This is a trap when upstream's new test files join the blitz-tests target list (`.github/scripts/test_blitz_tests_targets.py`).

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (stand graph re-read at `ebacd68bd`; citations; `:open`; `CSSStyleDeclaration` accessors; IME at editor construction) — the sync before last: manifests and lock unchanged, adapter probe 0 rows (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) `). Moved citations were re-pointed. Upstream behaviour changes were each ratified on the founder's word as engine changes for every document (`decision/ime-enabled-for-prefocused-input`). This is the template for this chunk's PROVISIONAL boundary widenings (SVG presentation attributes, `hr`'s `color`/`noshade`/`size`) with the founder away.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations; adapter re-read at `e42ee1f25`; `select_popup_painted` in `embolden`; the focus limit; key press past a dropped anonymous block; `text_contrast` past a blend; blending paints) — the most recent full sync. It fixed the panic in `BaseDocument::deallocate_anonymous_block`, a merge surface the scope re-reads row by row, leaving the `focus-fixup-reads-inert-alone` limit. `select_popup_painted` joined the `embolden` witness the scope runs. `mix-blend-mode`/`isolation` paint with no frame test, the open list painting last past blend layers. `text_contrast` reads `None` past a blend (`decision/contrast-unread-past-non-colour-paint`). The SVG isolated-layer commit lands on this paint path.

@history_item n=21 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (stand graph re-read on pulldown-cmark 0.13.6) — the adapter probe was re-read on a lock-only move. The adapter and its AT witness are owned by a residual row reading `re-carried`. Re-read again if this take moves the lock.

@history_item n=22 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (hazard closed by the buffered render; same-render limit; residual owner clauses) — `DioxusDocument` renders into `buffered_mutations` and replays it once (`decision/buffered-dioxus-render`). The scope carries this as an escher-only merge surface (`dioxus_document.rs`, `mutation_buffer.rs`, `lib.rs`), replaced whole only by the Dioxus 0.8 move, which this chunk leaves out. `res-llkgqygv` (adapter / AT witness) reads re-carried at 0.3.0.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record is written on `DioxusEventHandler::handle_event`'s path before `runtime.handle_event` (`contract/dioxus-document/reached-handlers`, `contract/dioxus-document/record-reached`), beside the registry arm (`contract/ui-graph-state/*`). These dioxus-native-dom surfaces must keep escher's behaviour through any bridge conflict.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (241 rows), each verified against the code. The picked rows and their guards are the form this chunk's resolutions are checked against. An upstream behaviour the take brings that no row states needs a `+` row.

@history_item n=25 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (`~ decision/changed-set-marks`; `+ decision/empty-alt-image-decorative`; `+ decision/hidden-checkable-label-toggle`) — the previous take. It qualified the changed-set row with a scroll that closes a list. It added two rows for ratified upstream behaviours no row stated, resolved by the operator on the founder's relayed «Утвердить все 19». This is the direct precedent for how this take records new upstream a11y-relevant behaviour (e.g. `hr` presentational hints, SVG attributes) as rows. With the founder away, those rows stay PROVISIONAL.

@history_item n=26 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful: the shell drains it and rebuilds the tree when the set it took is non-empty (`decision/a11y-tree-refresh-on-change`, ratified 2026-10-07 by `2026-10-07-driver-session`). "The changed set's contract" is the first item the scope says every resolution keeps.

@history_item n=27 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a fresh boot leaves it non-empty (`decision/harness-settle-leaves-changed-set`). This is a trap for upstream test files ported under the take that read `has_changes()`.

@history_item n=28 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click is read on the document's clock, the harness's virtual clock on a harness-built document (`decision/double-click-on-document-clock`, `decision/double-click-threshold`). This is the "clock invariant" the scope says every resolution keeps.

@history_item n=29 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter/Space activation, click focuses a button; `disabled` by presence) — the keyboard model: synthetic Click on Enter/Space, app handlers then `handle_click` (`decision/keyboard-activation-enter-space`, `contract/keyboard-activation/*`, `decision/click-focuses-buttons`). `handle_click`'s arms now live as private `click_*` functions, so an upstream arm edit is re-applied by hand (the scope's `cry-lrzp6e2g` carry).

@history_item n=30 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range model: keys, click focus, pointerdown value, one `input` per change (`contract/range-input/*`, `decision/range-input-keys`). These are "the range and keyboard models" the scope says every resolution keeps.
