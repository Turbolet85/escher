@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing and PR-token jobs repository-guarded; CI legs locked) — put `github.repository == 'DioxusLabs/blitz'` guards on publish `release-cli`, `post-results` and the WPT jobs, and `--locked` on every ci.yml cargo leg. This is where `decision/publish-job-repository-guard`, `decision/publish-secrets-upstream-main-only`, `decision/wpt-post-results-trusted-scripts` and `decision/locked-builds` come from. A take that edits `wpt.yml` (`f2065fd48`, `946f7c461`) or `publish-browser.yml` must keep those guards byte for byte. A ref-only `if:` would re-arm signing on the fork.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — brought in the cargo-deny audit leg, SHA-pinned every ci.yml `uses:` and made the ci.yml token `contents: read` (`decision/ci-actions-pinned-to-sha`, `decision/ci-token-read-only`, `decision/cargo-deny-audit-tool`). Its Kept clause left the guarded upstream-only workflows at `@latest` on purpose. The freight `cry-zexngiys` now reverses that by pinning wpt.yml, publish-browser.yml and wpt-post-results.yml and widening `.github/scripts/test_ci_workflows.py` past ci.yml. Pin only after the take's workflow lines stand.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — built escher's stderr sink scrub: the engine-target safe-field allowlist and the eleven content-named fields of `escher_` targets (`decision/engine-target-safe-fields`, `decision/escher-target-content-scrub`, `decision/stderr-only-telemetry-sink`). The upstream `fmt::init()` subscribers and the WPT `env_logger` were left unscrubbed. Background for the scope's "sink allowlist re-checked": new upstream `tracing`/`log` targets from the take, such as devtools/CDP, fall under these rules.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (unknown alignment flags; merge citation re-point) — the first upstream merge. It re-pointed 40 citations by the merge's line map and changed a no-panic fallback to upstream's Taffy behaviour. It also set the precedent that script accessors arriving by merge are a realization on the registered script crossing, not a boundary widening. Apply the same reading to any script API this take adds (`decision/css-property-named-accessors`, `decision/url-search-params-bootstrap`).

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — put the stable id on the platform adapter as AccessKit `author_id` (`decision/author-id-on-accessibility-nodes`). Standing rule: any further consumer of the id re-opens the row. Upstream's AccessKit refresh (`be60597f6`) must be read against it: `author_id` only on element nodes, never a NodeId.

@history_item n=6 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the origin of `decision/accessibility-tree-refresh-on-change`: `View::poll` rebuilds the platform tree when the changed set is non-empty. Ratified as the same crossing, more often. The take's AccessKit refresh (`be60597f6`) and its mutation changes touch exactly this path and the changed set's contract that the scope lists as a merge surface.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — measured third-party targets printing ids and names at `debug`/`trace`. Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. The scope's sink re-check must read a sink-installing host, like `host_log`/`mcp_log`, not a bare one. The same entry family registered `decision/driver-session-unix-socket-only`: a new socket or port reopens the crossing question, which bears on DevTools/CDP.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — dropped every record from a target outside the allowlist, zero bytes at every level (`decision/outside-target-dropped`, `decision/other-log-targets-dropped`, `decision/telemetry-sink-target-allowlist`). The accepted cost is that a third-party WARN/ERROR no longer prints. A new upstream crate's target is dropped whole unless it starts with an allowlisted prefix. A new `blitz*` target's fields are redacted to the seven safe ones. Re-read both against the take's new call sites.

@history_item n=9 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's second record class (one line per closed span) and that only a diff's three sizes may reach a log (`decision/closed-span-lines`, `decision/diff-counts-only-in-logs`). These are the limits the take's new spans or fields are checked against.

@history_item n=10 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (hint-less `@font-face` source, PROVISIONAL) — precedent: a boundary widening that arrives by an upstream merge still needs the founder's own word and is never settled silently. It stood PROVISIONAL until he ruled. This matches the scope's directive to list each widening the take carries (CDP, lossy decoding, new attribute reads, text-shadow/mask paint, AccessKit refresh, env reads) for the operator at P4.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (input rows re-read on the merged engine) — the sync's practice of re-reading input rows on the merged engine. The `file:` read became a narrowing, and new `autofocus` and Selection API rows were added as inventory, not widenings. It also recorded a compile-time env read. The model for this take's re-read of new attribute reads (font `face`/`size`, `valign`, alt as decorative) and env reads.

@history_item n=12 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (dependency security after the sync) — the dependency-delta record of a whole-set sync. The entering crates were each reviewed by hand (licence, repository, build script, proc-macro). Upstream's `licenses` CI job was taken and reverted. This is the review this take's entering crates owe (Stylo 0.24, Parley 0.13, vello 0.12, `parley_code_editor`, devtools), together with `decision/upstream-sync-dependency-set` and the `licence-gate` exception.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source ratified) — the founder ratified the merge-borne widening as upstream's behaviour with a check owed («Утвердить + проверка»). This is the shape a ratification of this take's widenings is likely to take.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings; the closed-field rule) — set the rule behind `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`. A closed-value field inside a ratified carrier is no widening. Authored text, an open value set, a new carrier or a new exit still is. Use it to sort which upstream additions go to P4 as widenings. Example: alt as decorative is a closed read, while text-shadow paint of page text is pixels.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the engine's password mask, `png`) — the password is painted masked in every document (`decision/password-painted-masked`), and no base64 crate is taken (`decision/no-base64-crate`). The take's paint features (text-shadow `795cddd5d`, CSS masks `a946a0a7b`) and the Skia default renderer (`74861e747`) must not paint a password's real glyphs (for example as a text shadow). Any renderer that does not go through blitz-paint is outside the guard.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (28-name falsy predicate; no git dependency; the sync's dependency delta) — moved Taffy from git to crates.io 0.15 and recorded that no git dependency is left (`decision/no-git-dependencies`, `decision/no-git-dependency`). The founder's rule this take's whole-merge shape would reverse, since upstream now pins git Taffy and git AnyRender. It also took upstream's `is_bool_attr` byte-identical (`decision/falsy-boolean-attribute-removed`).

@history_item n=17 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the whole-workspace `cargo deny --locked --workspace --all-features check advisories licenses` and the licence gate (`decision/dependency-audit-cargo-deny`, `decision/licence-gate`, `decision/dependency-audit-ci-gate`). It also recorded memmap2 kept out only by the `unsound` default. The audit leg this chunk must turn green, and the licence check any entering crate passes.

@history_item n=18 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (zip 2.4.2's three ignores) — precedent for external decay. An advisory that turned the audit red over an unchanged lock, with no reachable fix, was ignored by ID with a written reason and a residual owner on the founder's word (`decision/advisory-ignores-by-id`, `decision/audit-ignores-per-id-with-reason`). Directly relevant to the RUSTSEC-2026-0459 libsqlite3-sys red: here a fix exists (rusqlite 0.41) but is semver-incompatible, which makes it P4's fork.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (merged ARIA reads) — `aria-labelledby` names, descriptions to the adapter and three closed values were ratified as upstream's behaviour by the founder's word at P4 (`decision/aria-labelledby-outranks-label`, `decision/descriptions-adapter-only`, `decision/merged-aria-states-closed-parse`, `decision/aria-label-trimmed-once`). Precedent for reading the take's AccessKit refresh (`be60597f6`) and alt-as-decorative (`e65ec76e1`) as possible adapter-crossing widenings.

@history_item n=20 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (eight advisory ignores: the zbus family; the sync record) — a second external-decay ignore set and a whole-set sync record. It also left a criterion «with `deny.toml` unchanged» UNMET as written, on the operator's word. Precedent if this take's audit fix moves `deny.toml`. The zbus ignores are dropped when accesskit_unix moves to zbus 6, which the take's AccessKit refresh may do.

@history_item n=21 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — the first partial take. One commit was cherry-picked, the four Taffy-main commits (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) were left out because they name git Taffy, and the lock was byte-identical. It was PROVISIONAL until the founder ratified it. Those four commits are still in the left-out delta `cry-mjwr45xr` this take re-reads.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the `inert` read and the toggle per role) — origin of `decision/inert-subtree` and `decision/toggle-read-per-role`, engine surfaces escher changed. The scope names `inert` and the accessibility state reads as merge surfaces whose intent must survive conflict resolution.

@history_item n=23 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the clock invariant: `advance` moves the virtual clock exactly, and the scrollbar fade reads the document's clock (`decision/advance-moves-virtual-clock-exactly`, `decision/scrollbar-fade-on-document-clock`, `decision/time-moves-only-on-advance`). It also removed the epoch `unwrap` time reads. The scope lists the clock invariant as a merge surface. Upstream time reads in the take (DevTools, animations) must go through the document's `Clock`.

@history_item n=24 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the second partial take's lock record: two cherry-picks, no dependency moved, the Taffy-main commits again not taken, PROVISIONAL. A sibling record this take's lock record follows.

@history_item n=25 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — besides the single-select row (`decision/single-select-selectedness`, `decision/select-choice-exits`), it records the founder dropping the PROVISIONAL mark on the 2026-10-19 partial take («Утвердить (Рекомендую)»). Precedent: a take shape the delegate answered stays PROVISIONAL until his word.

@history_item n=26 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the ten-commit take; tokio past RUSTSEC-2026-0386) — the founder's «Десять коммитов (Рекомендую)» take. Git Stylo, Taffy-main and Dioxus 0.8 were not taken. A semver-compatible lock-only `tokio` update fixed an advisory with no ignore, and `mimalloc` was reviewed by hand. This is the remedy pattern the security rule prefers. Precedent for P4's shape fork and for the RUSTSEC-2026-0459 choice.

@history_item n=27 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`multi_doc`'s inputs, the traversal lock) — env reads that arrived by take (`MULTI_DOC_*`) were registered under Environment values read. The new upstream example input was validated in the examples' shape. Model for the scope's "new env reads" check on the 107 new commits.

@history_item n=28 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script runtime's module split) — 17 citations re-pointed by hand after upstream moved blitz-vibey-script's runtime into modules. Expect the same citation work where this take moves cited files.

@history_item n=29 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the eleven-commit take's lock record) — the founder's take, sixteen commits left out with reasons. Several of those reasons (git Stylo) may no longer hold now that upstream's Stylo resolves from the registry (0.24). That re-read is the freight `cry-mjwr45xr`.

@history_item n=30 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script fetch's `file:` refusal by document) — `cee6b8731` was recorded as a narrowing, not a widening (`decision/file-url-only-for-file-documents`, `decision/file-urls-only-from-file-documents`, `decision/default-script-fetcher-local-only`). Precedent for classifying take behaviour: a refusal added on an existing surface is no widening. The guard `remote_document_cannot_load_file_urls` must still pass.

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`open` by presence; CSS property accessors) — engine changes taken from upstream for every document, ratified for that take (`decision/open-pseudo-class-by-open-attribute`, `decision/css-property-named-accessors`). Their behaviour is to be kept through this take's conflicts.

@history_item n=32 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (dioxus-cli in the publish workflow; `TMPDIR` at a second test-only site) — publish-browser.yml installs dioxus-cli `0.8.0-alpha.3` behind the repository guard. Upstream test helpers add env reads that must be registered. Both are surfaces the take's workflow and test commits can move again, and both bear on the action pins written after the take.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `Node::hit_inner` answers `None` under a singular transform (`decision/non-invertible-transform-no-hit`). `0980cce5f`'s `hit_inner` hunk stays a textual conflict with this answer (freight `cry-lurql7hp`). The tip `563d3db73` also resolves transforms through anonymous blocks. Keep the guard `hit_non_invertible_transform`.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms; inert focus; native checkable; `change` event) — escher's engine changes for every document (`decision/visible-region-through-transforms`, `decision/inert-focus-clear-silent`, `decision/accessibility-focus-built-nodes-only`, `decision/engine-change-event-checkable`). Their guards (`visible_region_transformed`, `inert_subtree`, `checkable_change_event`) must survive the take's transform and AccessKit commits.

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (RUSTSEC-2026-0414, the pulldown-cmark lock update; the free-disk action) — a semver-compatible lock-only advisory fix, the operator's call. Also the `jlumbroso/free-disk-space` pinned action on `Stand checks repeated` (`decision/free-disk-only-on-stand-repeat`). Both are precedents for the audit fork and for the action-pinning freight.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key-press panic after a hidden ancestor) — a remote-reachable panic, owned by a CARRY on the next upstream sync on the founder's Q2. Its fix (`deallocate_anonymous_block`) is the merge surface the take re-reads against `563d3db73` "Resolve transforms through anonymous block boxes".

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list: shot pixels, crossing, gates, hostile markup, readers, paint, log silence) — the select list rows are an engine change for every document and a boundary widening, ratified (`decision/open-list-shot-pixels`, `decision/select-list-crossing`, `decision/select-list-open-gate`, `decision/select-list-hostile-markup-finite`, `decision/select-list-painted-over-page`, `decision/select-list-label-never-logged`, `decision/select-list-closes-when-select-unfit`). The scope names the select model and its list as a merge surface. The take's paint changes (masks, text-shadow, Skia) and pointer/scroll commits must keep these guards.

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds) — the opening API row became "no served TCP or UDP port", with two `blitz-net` loopback test binds registered as test-only (`absence/served-network-api`, `decision/session-socket-sole-listener`). DevTools/CDP from this take may open a served socket or port. That is a security-plan amendment first and goes to P4, not into the test-only class.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches the handler) — trap for later takes: a trait's do-nothing default (`NetHandler::error`) can hide a fix through a wrapper no conflict or gate touches. Sweep the implementers of every trait this take extends.

@history_item n=40 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (this take's lock record) — the last take: twelve commits, twenty-one left out with measured reasons, manifests and lock byte-identical, pins unchanged. These are the baseline counts and the left-out list this take resumes from. Its record stands beside the new one as history.

@history_item n=41 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` leg; the `matrix_test` free-disk step retired) — `decision/embolden-locked-fast-leg` and `decision/free-disk-only-on-stand-repeat` (guard `test_only_stand_repeat_frees_disk`). The `embolden` witness is one of this chunk's proofs. Any workflow edit by the take must keep the free-disk step on `Stand checks repeated` alone.

@history_item n=42 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — escher's own fix in `BaseDocument::deallocate_anonymous_block`, a direct engine change for every document, guard `key_press_after_container_lost_its_box` (`decision/dropped-anonymous-block-clears-parent`). It was flagged upstreamable, and the take's anonymous-block commit `563d3db73` must be read against it (freight `cry-lurql7hp`).

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the mounted event of a removed element) — `decision/skip-mounted-event-of-removed-element`. Upstream's test `element_removed_before_its_mounted_event_is_flushed` was left out because it reads 1 on Dioxus 0.7. It returns only if the shape takes Dioxus 0.8 (freight `cry-jlxpxul2`).

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a stylesheet that fails blocks no render) — `decision/unresolvable-stylesheet-href-no-load`, an engine change for every document taken from upstream. Keep guard `invalid_stylesheet_url_does_not_block_rendering`. `resolve_url` still panics and stays under Panic paths.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (page CSS paints blending and isolation; culling and the inline box's paint skip) — no frame test. The operator placed the frame gaps as a CARRY on this entry. They are now freight `cry-jbkftesv`: two headless frame tests in both layout modes, owed whatever shape is taken.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a custom widget's attributes replayed at set-up) — `decision/custom-widget-initial-attribute-replay`, an engine change behind the feature, ratified. A precedent for taken upstream engine changes that are ratified per take.

@history_item n=47 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams` on the script crossing) — `decision/url-search-params-bootstrap`, `decision/search-params-native-parser-hidden` and `decision/bootstrap-native-helper-removed`. A realization on the script crossing, no widening, with the native helper deleted before page scripts run. Any new bootstrap script or native helper the take adds is read the same way.

@history_item n=48 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops) — the sink redacts the `Resource load failed` message and the `url`/`error` fields, and `NetHandler::error` holds no `tracing` site. Part of the sink re-check over the take's new net or lossy-decoding call sites (`9b9471f2f`).

@history_item n=49 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the zip ignores gain RUSTSEC-2026-0429) — a further external-decay ignore by ID on the operator's own fix directive under the security rule, with the `deny.toml` citations shifted. Precedent for answering RUSTSEC-2026-0459 by an ignore with a written reason if the rusqlite move is refused.

@history_item n=50 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the audit's residual owners as store rows) — every standing ignore (zip `res-h7acz56c`, paste, zbus, memmap2) is owned by a store `residual` row. A take that moves accesskit (zbus 6) or anyrender (zip) may discharge one, and the row's owner must be updated.

@history_item n=51 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (pulldown-cmark 0.13.6 fixes RUSTSEC-2026-0433) — a lock-only semver-compatible advisory fix on the operator's directive. This is the remedy the security rule prefers when one exists.

@history_item n=52 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories published after a green audit) — origin of `decision/local-audit-before-push`. A CI audit can go red over an unchanged lock, so run `bash .github/scripts/ci-leg.sh audit` locally before a push. This is the mechanism of the RUSTSEC-2026-0459 red folded here.

@history_item n=53 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the spawned-task MountedData panic, removed by the buffered render) — `decision/dioxus-render-into-buffer`, guard the `dioxus_runtime_probe` `mounted_data_in_a_task_a_key_handler_spawned_acts` tests. The merge surface `cry-246ouqff`: upstream's Dioxus 0.8 `mutation_buffer.rs` replaces escher's whole if the 0.8 move is taken. The guard must still hold.

@history_item n=54 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Dioxus bridge's dispatch and the signal registry) — escher's edits to `DioxusEventHandler::handle_event` (`decision/dispatch-recorded-only-with-signal-registry`), in dioxus-native-dom, the most-modified crate. A take touching the bridge or Dioxus must keep the registry-gated dispatch and the buffered render untouched.

@history_item n=55 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record and handler_coverage as readers of the stable id) — the reached record written in `handle_event` with no log site and no NodeId (`decision/reached-handler-record-without-log`, `decision/reached-record-deduplicated-survives-removal`). This is another escher edit in the bridge dispatch that the take's conflict resolution must keep.

@history_item n=56 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the security plan became rows, verified against the code at edd0cfaea7f0. The picked rows' handles and their guards come from this conversion. Any row the take makes false is amended by handle at this chunk's wrap.
