@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI through one leg script) — every linux CI job runs `bash .github/scripts/ci-leg.sh {leg}`, cargo legs `--locked`, `fast` the local pre-push gate (`decision/local-pre-push-gate`, `decision/locked-cargo-legs`); the chunk's exit ("fast", "doc", "a11y", "audit" and fork CI green) is read through exactly this runner, and a take that edits `ci.yml` keeps its legs and the leg/ci contract shape.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (upstream-only signing, WPT and dispatch) — every job reaching the Signed Builds/WPT environments, Pages or dispatch carries `github.repository == 'DioxusLabs/blitz'` (`decision/upstream-ci-resources-upstream-only`, `decision/publish-workflow-upstream-only`); upstream edits to `wpt.yml`/`publish-browser.yml` (`f2065fd48`, `946f7c461`) must keep these guards, and "removing the workflows was rejected".

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — introduced the `audit` and `a11y` legs and pinned every `ci.yml` `uses:` to a 40-hex SHA with a trailing ref comment, each `dtolnay/rust-toolchain` naming its toolchain (`decision/actions-pinned-to-commit-sha`, `decision/pinned-actions`); the folded freight widens this pinning to `wpt.yml`, `publish-browser.yml` and `wpt-post-results.yml` and the guard past `ci.yml`, after the take's workflow lines stand. Trap recorded: a pinned toolchain action loses the toolchain `@stable` selected.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first sync: upstream merged whole (11 commits, conflict-free, additive), the `Upstream sync:` line created as the next sync's merge base, and every `file:line` citation into merged files re-pointed by a measured line map; the precedent for the sync line (`decision/upstream-sync-merge-base`) this chunk moves at its wrap and for the citation sweep a take owes.

@history_item n=5 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — the root `README.md` is escher's own page and at an upstream sync an upstream change to it resolves to escher's version (`decision/readme-is-escher-own`, `decision/readme-is-escher-page`); also records `stylo_taffy`'s MPL 2.0 licence exception (`decision/workspace-licence`) — a take touching `README.md` or `stylo_taffy` meets these.

@history_item n=6 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — `Document::accessibility_tree()` defaults to the inner tree and `DioxusDocument` overrides it to set `author_id` to the stable id; blitz-shell's `update_tree` takes `&dyn Document` (`contract/document-trait/accessibility-tree`, `contract/accessibility-tree/author-id`). Upstream's AccessKit refresh `be60597f6` lands on this crossing; trap: a feature-gated item can pass `--workspace` and be compiled out under `-p blitz-tests`.

@history_item n=7 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the stable-id grammar as built against dioxus-core 0.7.10 (`VComponent.name`, framework scopes adding no segment) (`contract/stable-element-id/*`, `contract/dioxus-bridge/stable-id`); a Dioxus 0.8 shape would re-read every assumption this grammar measured on 0.7.10, and the scope names the stable ids among the intents a resolution must keep.

@history_item n=8 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the changed set: `has_changes` / `take_changed_nodes`, the write sites, node creation marking nothing, the shell's refresh on change (`contract/base-document/has-changes`, `contract/base-document/take-changed-nodes`, `decision/changed-set-write-sites`, `decision/a11y-rebuild-on-nonempty-changed-set`); the "changed set's contract" a take must keep, and the freight's qualifier amends `decision/changed-set-write-sites` at this chunk's wrap.

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits only `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES`, dropping every other target whole at every level (`decision/telemetry-target-prefix-drop`, `contract/telemetry/target-filter`); the scope's "sink allowlist re-checked" reads the taken delta's new `tracing`/`log` targets against these two sets.

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the merge base, Parley's pin form and the merged dependency set) — the second sync took upstream's dependency set whole with no `cargo update`, Parley moved to the registry; the coupling rule (parley, skrifa, vello move together) stands (`decision/font-deps-match-renderers`, `decision/parley-text-layout`) — the rule a take of Parley 0.13 / vello 0.12 must honour as one set.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy's tree traits on `LayoutPassState`; the bounds reader renamed) — upstream moved the Taffy traits off `BaseDocument` and removed `unrounded_absolute_position`; escher's one edit moved `visible_region` onto the new reader (`contract/layout/taffy-traits`, `contract/layout/stylo-taffy-exports`) — precedent for an upstream API move forcing a minimal non-additive edit, and the surface git Taffy at `a3997d55…` would move again.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — upstream's `autotests = false` single `all` target is taken and held at `test = false`, pinned by `test_blitz_tests_targets.py` (`decision/blitz-tests-target-per-file`, `decision/blitz-tests-single-test-target`); the scope's rule for upstream's new test files, and a take that drops `test = false` or takes `autotests = false` turns the pin red.

@history_item n=13 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled-box bounds and the hit stop at a clipping padding box, direct engine changes flagged upstreamable (`decision/client-rect-own-box`, `decision/hit-stops-at-paint-clip`, `contract/scroll-hit/hit-clip`); two of the "engine surfaces escher changed" a take reaching `hit_inner` or `physical_unrounded_geometry` must keep.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the accessibility builder reads three state attributes) — the builder's state reads, a direct engine change, trimmed lower-case exact match, closed values only to the adapter (`decision/a11y-aria-states`, `decision/aria-state-values-exact-lowercase`, `decision/adapter-crossing-carries-states`); "the accessibility state reads" a resolution keeps, and the rule that a later attribute read keeps a closed set with no text carried out.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot, the password mask) — the password mask in every document, merge surface `render.rs` and `document.rs`; the harness composites over white through `anyrender_vello_cpu` (`decision/password-input-paints-masked`, `contract/paint/password-masked`, `decision/headless-screenshot-cpu-renderer`) — a vello 0.12 / AnyRender move reaches both, and shot pixels may move and must be re-measured, not loosened.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry, the beta.3 pins) — taffy became the registry "0.15" and no git dependency is left (`decision/no-git-dependencies`, `decision/taffy-layout`); trap: taffy now moves with the registry like parley — the binding rule upstream's git Taffy/AnyRender at `563d3db73` collides with.

@history_item n=17 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node; the 28-name falsy predicate) — `mutation_writer.rs` made byte-identical to upstream's, a removed node dropped (`decision/bridge-removal-drops-subtree`, `contract/dioxus-bridge/boolean-attributes`), the default sheet arriving through `DocumentConfig::default()` (`decision/dioxus-no-own-stylesheet`) — keeps the bridge diff small for the next take.

@history_item n=18 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range model, a direct engine change with a merge surface of seven upstream files plus `node/range.rs` (`decision/range-current-value`, `decision/range-value-write-reseeds`, `decision/range-input-value-kept-on-reconstruct`); "the range and keyboard models" a resolution keeps.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation and a click's focus, direct engine changes, merge surface `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs` (`decision/keyboard-activation-enter`, `decision/keyboard-activation-space`, `decision/click-focuses-button`) — upstream's `CDP:` and input commits reaching these files are resolved against them.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `audit` widened to `--workspace --all-features check advisories licenses`, the clippy leg's second line (upstream's blitz-dom feature-subset clippy), `stand-repeat` at zero failures, the coverage floors (`decision/blitz-dom-feature-subset-clippy`, `contract/ci-leg-legs/audit`, `decision/stand-repeat-zero-failure-budget`, `decision/per-crate-coverage-floors`); the clippy red (rustc 1.100 `needless_bool`) and the RUSTSEC-2026-0459 audit red this chunk folds land on these legs.

@history_item n=21 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and button arms as private named functions) — the arms split out while the parents were left "at their upstream floor so the next upstream sync's diff stays out of arms escher does not own"; the additivity aim the scope's resolution rule continues.

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the sync at `23b9b26a9`) — the last whole merge, today's merge base; keyboard-types 0.8 beside Dioxus 0.7.3's own 0.7, so the workspace crate is named `::keyboard_types` (`decision/keyboard-types-named-by-absolute-path`, `decision/workspace-keyboard-types-path`) — a Dioxus 0.8 shape would collapse the two versions.

@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads; descriptions on the adapter crossing) — upstream's ARIA reads merged with each attribute read once, upstream's own bool `aria-selected` and unfiltered `aria-label` dropped (`decision/each-aria-attribute-read-once`, `decision/adapter-crossing-carries-descriptions`); the precedent for a widening arriving by upstream merge, ratified as upstream's behaviour — the class the scope lists for the operator at P4.

@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the keyboard-types 0.8 surfaces, `keyboard_compat`, the coverage exclusion) — upstream's `keyboard_compat` bridge in dioxus-native-dom and its coverage exclusion by filename (`decision/keyboard-compat-coverage-exclusion`); it stays until escher takes upstream's retirement in `286874cc1` (the Dioxus 0.8 move).

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes; the geometry readers; the toggle per role) — `inert`, `disabled` on form controls, the toggle per role, the `contain: paint` hit stop, `absolute_position` less ancestors' scroll, the client rect under transforms, the password caret by bullets (`decision/inert-honoured`, `decision/disabled-read-on-form-control-only`, `decision/toggle-read-per-role`, `contract/scroll-hit/absolute-position`, `contract/geometry/get-client-bounding-rect`); their merge surface is now carried to this chunk.

@history_item n=26 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take: `fb8d6c15e` alone by a no-commit cherry-pick, the merge base unmoved, the Taffy-main commits waiting because they put a git Taffy in the root manifest (`decision/upstream-sync-merge-base`, `decision/upstream-taken-by-cherry-pick`); the origin of the cherry-pick shape this chunk's P4 fork weighs again.

@history_item n=27 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core: the document's clock) — every engine time read goes through `DocumentConfig.clock`, no `Instant::now()` / `SystemTime::now()` in blitz-dom outside `clock.rs`, merge surface `config`, `clock`, `document`, `resolve`, `iframe`, `scrolling`, `events/pointer.rs` (`decision/document-clock`, `decision/no-wall-time-outside-clock`); "the clock invariant" — any taken upstream time read (e.g. DevTools, animations) must route through the clock.

@history_item n=28 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the second partial take (`38ca4e777`, `17de049b1`) by the operator as delegate, PROVISIONAL under "an engine behaviour change for every document" and discharged at the epoch boundary; precedent that a delegate's take is provisional while the founder's word ratifies it.

@history_item n=29 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — `BaseDocument::text_extent`, a read-only reader on every document, `Session::lint` its caller (`contract/basedocument-readers/text-extent`); `text_extent` is among the engine surfaces the scope names, and a Parley 0.13 move can shift its readings and the lint's pinned findings.

@history_item n=30 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model, a direct engine change for every document, merge surface by file carried (`decision/select-model-engine-change`, `decision/select-keyboard-model`, `decision/select-choice-single-path`); "the select model" a resolution keeps.

@history_item n=31 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the upstream take read at `f9cbb2756`) — «Десять коммитов (Рекомендую)», pins unchanged; git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version (`decision/deferred-upstream-dependency-bumps`); the flake pin brought to 1.91.0 (`decision/flake-rust-pin-matches-rust-version`); the precedent the scope cites for the take shape.

@history_item n=32 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` traversals serialised; the dropped-node hit skip; the recorded style facts) — upstream engine changes ratified with the take: `PARALLEL_TRAVERSAL_LOCK`, `hit_inner` skipping dropped nodes, `Node::out_of_flow`, `ElementData::classes` (`decision/parallel-traversals-serialised`, `decision/hit-and-paint-skip-dropped-nodes`, `decision/node-holds-structure`, `decision/class-atoms-synced-on-write`) — Stylo 0.24 or a later Taffy reaches each.

@history_item n=33 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`multi_doc`, the merge surfaces and their owners, the coverage exclusion's clause) — the range and keyboard merge surfaces read at the pin, the one conflict resolved, the engine-debts surfaces re-owned forward; and the coverage exclusion "until escher takes upstream's retirement" of `keyboard_compat.rs` (`286874cc1`).

@history_item n=34 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen named targets on one `cargo test --workspace --locked` line, pinned by `test_ci_workflows.py` (`contract/ci-leg-legs/a11y`, `decision/a11y-leg-widens-without-new-job`); this chunk's exit "a11y leg green" and the a11y target re-proof run through it.

@history_item n=35 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the upstream take read at `ebacd68bd`; six upstream behaviour changes ratified for this take) — «Одиннадцать коммитов (Рекомендую)», pins unchanged, manifests/lock/`deny.toml` byte-identical; upstream's behaviour changes are ratified per take and not counted among escher's direct engine changes (`decision/upstream-behaviour-changes-ratified-per-take`) — the pattern this take's commits fall under.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the merge surfaces read at `ebacd68bd`) — the range model's one conflict in `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm as a second `if`; keyboard activation untouched — the resolution record pattern each surface this chunk re-reads follows.

@history_item n=37 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`; CSS property accessors; `:open`; IME; redraw route; citations re-pointed) — upstream changes recorded per kind: a narrowing, a realization on the registered script crossing, new public symbols registered, citations into taken files re-pointed (`decision/no-internal-fetch`, `contract/css-style-declaration/property-accessors`, `decision/shell-redraw-through-event-loop`, `decision/ime-enabled-at-editor-construction`) — the classification this take's script and paint additions get.

@history_item n=38 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the publish workflow's dioxus-cli) — `publish-browser.yml` installs dioxus-cli `0.8.0-alpha.3` or a git install at a pinned rev, upstream-only and guarded; the file whose actions the pinning freight now pins (`publish-browser.yml:92-142`).

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — `visible_region` through transforms, a non-invertible transform taking no hit, the focus fixup, the tree's focus fallback, native `toggled`, `multiple`, `optgroup`, `change` on checkbox/radio; upstream `0980cce5f`'s `hit_inner` hunk is a textual conflict with the non-invertible answer (`decision/hit-focus-form-engine-changes`, `decision/non-invertible-transform-takes-no-hit`, `contract/scroll-hit/hit-degenerate-transform`) — a named conflict for this take.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (pulldown-cmark resolved at 0.13.5) — an advisory fixed by a semver-compatible `Cargo.lock` move, no coupled pin, the operator's call; the precedent beside the audit fork this chunk carries (rusqlite 0.41 is not semver-compatible).

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded; the list in the changed set) — the select list, a direct engine change, merge surface sixteen files; the changed set's fifth site and the never-list holding only with no list open (`decision/select-list-engine-change`, `decision/select-list-changed-set`, `decision/changed-set-write-sites`) — the qualifier the freight lands at the wrap.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — a local gate read green over a macOS-only defect, so the embolden witness was owed (`decision/font-embolden-local-witness`); a Parley/vello move must pass the `embolden` step the scope lists among the proofs.

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream line at `e42ee1f25`; the twelve commits ratified for their take) — 36 of 57 taken, 21 left out each with its measured reason, Stylo 0.22.0 / Taffy 0.15 / Dioxus 0.7.3 kept; the last take's pin `e42ee1f25` and the left-out list this chunk re-reads against `563d3db73`.

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`; a failed load ending the wait) — standing trap: a trait default method can hide a take's fix (the do-nothing default compiled through `CountedHandler`), so a later take sweeps the implementers of every trait it extends.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the queued `mounted` event's guard) — upstream's `element_removed_before_its_mounted_event_is_flushed` left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0 (`decision/stale-mounted-event-skipped`); the freight returns it only with the Dioxus 0.8 move.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip; blending painted) — `paints_own_box` and `mix-blend-mode`/`isolation` painted with no frame witness, each gap a CARRY on this chunk (`decision/node-holds-structure`); the two headless frame tests in both modes the freight requires whatever the shape.

@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the Stylo prefs behind a `Once`; `background-clip: text` left out) — prefs written with Stylo 0.22 names, upstream's 0.23 `set_pref!` names not (`decision/stylo-prefs-set-once`); `ac6f9f582` left out since `BackgroundClip::Text` is gecko-only on 0.22 (`absence/background-clip-text`) — both reasons the freight re-reads now Stylo 0.24 is a registry version.

@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner at `e42ee1f25`; the key-press panic's fix) — every merge surface (seven, eight, clock, select model, select list, range, keyboard) re-owned to «Upstream sync ahead of determinism and exploration»; the `deallocate_anonymous_block` fix, upstreamable, upstream's tree panicking the same way (`decision/anonymous-block-dealloc-detaches`, `decision/anonymous-block-drop-clears-layout-parent`) — read against `563d3db73`'s anonymous-block transform fix; trap: assert a panic probe's fixture first.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`AGENTS.md` stays out) — upstream's `AGENTS.md` from `22f0cd000` is kept out of the tree (`decision/upstream-agents-md-excluded`); one of the left-out 21 re-read here.

@history_item n=50 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`; the test-only binds and no served port) — `fast` ends on `embolden` (`contract/ci-leg-legs/fast`, `contract/ci-leg-legs/embolden`); blitz-net's loopback test binds registered as test-only with no served port (`absence/served-network-port`) — DevTools/CDP opening a port would break this standing absence.

@history_item n=51 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the advisory fix, pulldown-cmark 0.13.6 and the deny.toml ignore) — a semver-compatible lock move plus one per-ID `deny.toml` ignore with a reason (`decision/dependency-and-licence-audit`); the two forms `.claude/rules/security.md` allows that the RUSTSEC-2026-0459 fork chooses between.

@history_item n=52 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured; a node's component through Dioxus public API) — the hazard's fix routed to a port of `286874cc1` onto the 0.7 bridge; owning component and held signals read through Dioxus 0.7.10's public API, hook-local state needing a fork-level change (`decision/owning-component-via-public-api`, `decision/graph-state-by-registration`) — surfaces a Dioxus 0.8 take re-reads.

@history_item n=53 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — sentences once naming `.andromeda/residuals.md` now name residual rows, among them `res-3v42legm` (the merge surfaces) which this chunk owns; read those owners through `rec.py get`.

@history_item n=54 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed; the buffered render's witness) — the buffered render, a port of `286874cc1` onto the 0.7 bridge, flagged not upstreamable; merge surface `dioxus_document.rs`, `mutation_buffer.rs` (replaced whole by upstream's 0.8 file at the 0.8 move), `lib.rs`, its re-read a CARRY on this chunk (`decision/bridge-buffered-render`, `decision/dioxus-bridge-buffered-render`); trap: upstream's ported test is insensitive to the borrow, the probe's two `…_spawned_acts_*` tests are the discriminating witness.

@history_item n=55 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — `DioxusEventHandler::handle_event` sets the registry's dispatch only where a registry stands, `mutation_writer.rs` kept byte-equal to upstream `434e0661` (`decision/dioxus-event-path-records-opt-in`, `contract/dioxus-bridge/registry-dispatch`, `decision/first-graph-state-hook-provides-registry`) — an escher edit on dioxus-native-dom's event path, the scope's most-modified crate.

@history_item n=56 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch) — the ungated `reached` module writes before `runtime.handle_event` where switched on (`decision/reached-record-before-dispatch`, `contract/dioxus-bridge/reached-record`); a second escher hook in `handle_event` a bridge-touching take must keep, `stand_handler_coverage` deciding.

@history_item n=57 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (1530) verified against the code at `edd0cfaea7f0`; amendments now name rows by handle, so this chunk's wrap amends rows (the sync line, the changed-set qualifier, any merge-surface row) by `~`/`+`/`−` handle.

@history_item n=58 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — a row amendment in the new form (`~ decision/actions-keyed-reading`, `~ decision/keyed-wire-form`, state provisional → standing); the form the pinned `decision/changed-set-write-sites` qualifier takes when it lands at this chunk's wrap.
