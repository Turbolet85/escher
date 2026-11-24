@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs; WPT and publish telemetry upstream-only) — added the per-leg `ci-log-{job id}` failure artifact confined to `target/ci-logs/` and marked the WPT report, `wptscores.json` and the publish-build trace logging upstream `DioxusLabs/blitz` only, their jobs repository-guarded. Origin of `decision/ci-log-artifact-on-failure`, `decision/fork-ci-two-artifacts` and `decision/wpt-publishing-upstream-only`: any upstream workflow change the merge carries must keep the `github.repository` guard and add no fork artifact.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (file:line citations re-pointed after the CI files moved) — re-pointed 8 citations into `Cargo.toml`, `wpt.yml` and `publish-browser.yml` with no claim change. Precedent: the take moves root `Cargo.toml` (Taffy 0.16, dev features `net`/`html`) and possibly upstream workflow files, so cited lines must be re-pointed by the merge's line map.

@history_item n=3 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `coverage-report` artifact (success only, 7 days), the one fork-CI artifact outside `target/ci-logs/`. Origin of `decision/coverage-artifact-on-success`; the merged tree's coverage leg must still produce it.

@history_item n=4 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set up escher-telemetry (stderr-only, `RUST_LOG` default `warn`, `LogTracer` bridge), the engine-prefix allowlist with its seven safe fields and the content-named redaction set, and the rule that tracing in engine/upstream crates compiles only with the `tracing` feature while escher-telemetry is ungated. Origin of `decision/engine-tracing-feature-gated`, `decision/telemetry-via-tracing`, `decision/engine-target-safe-fields` and the `contract/telemetry-sink/*` rows. Any new upstream log site (blitz-shell `dnd.rs`, blitz-dom drag events) must be feature-gated and land under an engine prefix.

@history_item n=5 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — named `seven_guis_native` as the sink installer and recorded that the headless stand installs no subscriber. Origin of `decision/headless-stand-no-sink`, which upstream's new test targets (`flex_br`, `drag_and_drop`) must keep.

@history_item n=6 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added the agent-run harness log and its unscrubbed `run.log` under `target/agent-run/`, uploaded by no CI leg. Origin of `decision/agent-run-raw-log-unscrubbed`. Relevant to the directive: no raw log carrying a temp or home path goes into evidence.

@history_item n=7 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — recorded that `transcript.jsonl` is raw by design and the one committed copy is host-path-masked, with `gate.py hygiene` refusing an unmasked host path in committed evidence. Origin of `decision/committed-transcript-copies-masked`; the same hygiene binds this chunk's evidence files.

@history_item n=8 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (file:line citations re-pointed after the upstream merge) — re-pointed 20 citations into merged upstream files by the merge's measured line map and recorded that the merge added no log, print or tracing site. This is the first upstream-sync precedent for the same pass on `29a439e05`.

@history_item n=9 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (root-manifest citations re-pointed after the line-61 insert) — fixed 6 root `Cargo.toml` citations that an earlier wrap had left one line low. Trap: a manifest edit (here the Taffy move and the dev-feature change) shifts every root `Cargo.toml` citation past it, so re-point them in the same pass and do not carry them.

@history_item n=10 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the founder ruled no OTel export (no OTel crate, no egress, no credential path) and ratified the coverage-report upload. Origin of `decision/no-otel-export`; the merge's new `url` dependency and drop code must add no export or egress.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — added `escher-session` as the second sink installer and scoped "no escher sink" to in-process boots. Trap: a zero read from a host that installs no sink says nothing about one that does. Behind `decision/scrub-sink-reach` and `decision/sink-host-read-at-trace`; the post-merge re-check must read the sink-installing host (`host_log`, `mcp_log`).

@history_item n=12 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — introduced the three-outcome allowlist: engine prefixes scrubbed to safe fields, `escher_` with content-named fields redacted, every other target dropped whole at every level. Rule for later chunks: a target is admitted only together with a scrub rule for it. Origin of `decision/sink-target-allowlist`; the sync's re-check reads every new target in the delta against it.

@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that `Command`/`Call`/`ArgValue` derive `Debug` holding ids and text, which nothing may print, log or field. Origin of `decision/debug-types-never-logged`, which conflict resolution in the driver/dioxus-native-dom must keep.

@history_item n=14 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built; `tracing` ungated in escher-driver) — made escher-driver take `tracing` ungated and recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child at `info` receives engine events. Origin of `decision/escher-crates-always-trace` and `decision/engine-tracing-by-invocation`. Both builds must be re-read after the merge.

@history_item n=15 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed-span line as a second record class, judged pair by pair under the same scrub. Origin of `decision/closed-span-lines`, `decision/closed-span-lines-judged-alike` and `contract/span-line/*`.

@history_item n=16 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — recorded that the package install's output stays in the step log. Kept: a new artifact out of fork CI is a boundary widening that waits for the founder. This binds any CI workflow edit the merge brings.

@history_item n=17 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — measured the workspace-built host writing one engine `blitz_dom::document` INFO line and the host built alone writing none, and made stdout answers never log lines. Baseline for `decision/engine-tracing-by-invocation` and `contract/escher-session-stdout/command-answer`.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log) — gave the host's by-level readings under both builds, with 0 id, name and sentinel needles. These are the numbers a post-merge `host_log`/`mcp_log` re-read compares against.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the `tool_call` span with exactly two closed fields. Origin of `decision/tool-call-span-two-closed-fields`, which the merge must leave unchanged.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — made a shot's image leave only in its answer, with 0 image bytes on stderr under both builds. Origin of `decision/shot-image-leaves-only-in-answer` and `contract/command-answer/shot`. Relevant because the merge touches paint (`f65baeca5` one-axis clip), so the screenshot readings are re-measured.

@history_item n=21 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — added the merge's five upstream integration targets to the no-subscriber census, with `host_log`/`mcp_log` passing under both builds and the fixed-sha census reading 0. Direct precedent: upstream's new `flex_br` and `drag_and_drop` targets join the census the same way.

@history_item n=22 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — re-read the workspace host at 6 lines, with the `focussed node` line printing twice. Dated readings stand and re-reads are appended. This is the precedent for recording a moved by-build reading after the merge without overwriting it.

@history_item n=23 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — measured a client's stderr and the workspace-built windowed `seven_guis_native` by build and level, 28 runs with 0 needles. This bears on `decision/engine-tracing-by-invocation` because the merge's winit drop source runs in windowed binaries.

@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — excluded `keyboard_compat.rs` from the coverage report's lcov. The scope's CARRY `cry-twrvkwp6` re-reads this exclusion at the sync together with the Dioxus 0.7.3 hold.

@history_item n=25 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — recorded upstream's focus checks silent and the `Focussed node` INFO line standing, with `host_log`/`mcp_log` passing under both builds. The engine take was PROVISIONAL on a delegate answer. This is a precedent for the take's census and for a take's ratification status.

@history_item n=26 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — read the ten-commit take silent, recorded the sink allowlist re-check holding, and recorded that the new root example `multi_doc` prints timings via `println!` outside the sink. Rule: each upstream sync is read for log sites and the sink re-checked. The new root examples `examples/dnd.rs` and `examples/file_drop.rs` need the same reading.

@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed into the code the take moved) — re-pointed the script-error-surface and engine citations, by hand where a citation's code left its file. This touches the `contract/window-error-event/*`, `contract/window-onerror/arguments` and `contract/script-error-source-labels/*` rows. Precedent for the citation pass after the merge.

@history_item n=28 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd` silent, and the sink re-check restated) — replaced the census line at Validate rather than adding a sibling, since a body holds current truth and history lives in the sidecar. It noted a test-only `TMPDIR` read and `escher-telemetry`/`escher-driver` unchanged. This is the format for this sync's census and re-check.

@history_item n=29 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the blitz-vibey-script slice's absences after the module split) — kept each slice absence (metrics, scrubbing, spans) standing for code moved into new files, under the slice's own count. Applies to the take's new modules (blitz-traits `DataTransfer`, blitz-shell `dnd.rs`).

@history_item n=30 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the test crate slice's absences hold `pseudo_element_flow_classification`) — named an upstream test target added as its own file inside the test-crate slice's absences. Direct precedent for `flex_br.rs` and `drag_and_drop.rs`.

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — placed upstream's `file:` refusal (typed `FetchError::Forbidden`, no log site) in §7. Origin of `decision/no-file-loads-from-non-file-documents`. The take's dropped-file reads over `std::fs` meet this rule, and the scope requires keeping the `file:` refusal through conflicts.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, no log site added, and the census's carve-out) — held blitz-shell's `src/event.rs` inline `#[cfg(test)]` module out of the added-lines census, so a 0 does not cover it. Trap for the merge's blitz-shell additions (`dnd.rs`, event changes).

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — recorded the engine INFO line once per focus move under the workspace build, holding a node number only. This is the engine-line baseline for `decision/engine-tracing-by-invocation`, which the merge's blitz-dom event-driver changes could move.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — recorded the engine's `change` after `input` (a checkbox always, a radio only when not checked before) and that a page's script hears one `change`. Origin of `contract/page-events/checkbox-change`, `radio-change` and `single-change`. The merge's events-driver change (`22c18e38f`, preventDefault ignored on a non-cancelable event) must keep these.

@history_item n=35 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed past the take's line moves) — a digits-only re-point across §1–§9 after the take. Kept: telemetry-strategy's `ci-leg.sh fast` naming. Precedent for this sync's re-point pass.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — `NetHandler::error` became a public trait method with a do-nothing default, which hid the fix at `CountedHandler`. Standing trap: sweep the implementers of a trait a take extends. This applies directly to the take's new blitz-traits `DataTransfer` and drag-event surfaces.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream take read at `e42ee1f25` is silent) — the most recent take's silence line. Only test-only loopback binds were added, `host_log`/`mcp_log` read 0 needles at `trace` under both builds, no scrub set moved, and the earlier take's lines stand beside it as history. This is the template for this sync's silence and sink re-check.

@history_item n=38 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — converted the master's text into rows (281 rows) verified against the code. Every picked row's handle and wording dates from here, and later amendments name rows by handle.

@history_item n=39 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`, widened to the engine sub-resource refusal (fixed text, no URL, no log site). This is part of the `file:` refusal that the merge's conflicts must keep.

@history_item n=40 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent`, so the re-run child exception covers digests. This is the area-3 silence rule that upstream's new test targets are read against.

@history_item n=41 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `~ decision/no-file-loads-from-non-file-documents` to its current wording (the browser app's favicon and link navigation drop it silently). The OS drop input and its `std::fs` file reads in dioxus-native-dom are a new file-reading path to set against this rule.

@history_item n=42 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (3. Observability Harness Contract: 2 rows) — `~ decision/headless-stand-no-sink` and `~ decision/checks-silent`: `stand_layout` installs its own recording subscriber. The Taffy 0.16 adaptation of `layout_reason.rs` makes `stand_layout` a surface of this take.

@history_item n=43 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (4. Span / Trace Coverage: 1 row) — `+ decision/layout-read-opens-no-span`, which `stand_layout`'s TRACE capture guards. Under the `CalcHandle` adaptation the layout read must still open no span.

@history_item n=44 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (3. Observability Harness Contract: 3 rows) — `~ decision/headless-stand-no-sink`, `~ decision/driver-library-installs-no-subscriber` and `~ contract/telemetry-sink/rust-log`: the scoped capture beside the sink, filtered by `RUST_LOG` only on the sink layer.

@history_item n=45 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (4. Span / Trace Coverage: 1 row) — `~ decision/driver-emits-no-event`: the driver sets the scoped capture around an acting step.

@history_item n=46 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (6. Log Coverage: 2 rows) — `~ contract/escher-sink-env/rust-log` and `+ decision/capture-framework-targets-warn-up`. Any new engine or framework warn site in the take (the drop handling) reaches the capture.

@history_item n=47 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Observability Harness Contract: 4 rows) — `~ decision/headless-stand-no-sink`, `~ decision/checks-silent`, `~ decision/answers-echo-nothing-typed` and `~ decision/driver-library-installs-no-subscriber`: the error-text check installs escher's sink, and the chaining panic hook was added.

@history_item n=48 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Span / Trace Coverage: 1 row) — `~ decision/driver-emits-no-event` to its current wording (the capture keeps records and panics in process).

@history_item n=49 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (6. Log Coverage: 1 row) — `~ decision/capture-framework-targets-warn-up` to its current wording: engine targets are admitted at warn and above.

@history_item n=50 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (7. Error Capture & Reporting: 2 rows) — `~ contract/telemetry-scrub-set/error` and `~ contract/telemetry-scrub-set/url`: records cross into the caused answer under the field-name scrub. New `url`-bearing fields from the take's drop code (the `url` dependency) meet these.

@history_item n=51 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (8. PII Scrubbing & Compliance: 1 row) — `~ decision/escher-target-content-named-set`, with `password` added to the redacted set.

@history_item n=52 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (8. PII Scrubbing & Compliance: 3 rows) — `− decision/scrub-at-sink-not-call-sites`, `+ decision/scrub-by-field-name` and `+ decision/engine-sites-fixed-messages`. Engine warn/error sites write fixed messages with values in named fields. Any new upstream log site in the delta must follow this, or be adapted to it in the merge.

@history_item n=53 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (7. Error Capture & Reporting: 1 row) — `~ contract/telemetry-scrub-set/error` to its current wording: the capture keeps `error` as written, cut at the text bound.

@history_item n=54 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (8. PII Scrubbing & Compliance: 1 row) — `~ decision/scrub-by-field-name` to its current wording (the capture uses the answer's own hidden set).

@history_item n=55 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (3. Observability Harness Contract: 1 row) — `~ decision/answers-echo-nothing-typed`, with the error field shown as written. This sits beside `decision/debug-types-never-logged` in area 3.

@history_item n=56 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage: 1 row) — `~ decision/layout-read-opens-no-span` to its current wording (it runs inside the layout verb's command span). This is re-read after the Taffy adaptation.

@history_item n=57 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (6. Log Coverage: 2 rows) — `~ contract/command-span/verb` (fifteen words) and `+ contract/command-span/layout`.

@history_item n=58 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage: 2 rows) — `~ contract/tool-call-span/tool` and `~ contract/command-span/settled`, which adds layout to the verb-alone reads.

@history_item n=59 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Observability Harness Contract: 1 row) — `~ decision/sink-host-read-at-trace`, which adds the layout-answer key needles that `host_log`/`mcp_log` read after the merge.

@history_item n=60 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (8. PII Scrubbing & Compliance: 2 rows) — `+ contract/layout-answer/content` and `+ contract/layout-answer/logging`. The layout answer's readings move with Taffy 0.16 and must stay off the log.

@history_item n=61 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (8. PII Scrubbing & Compliance: 1 row) — `~ contract/layout-answer/logging` to its current wording (transcript carrier on a live cold-agent run).

@history_item n=62 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (the delegate confirms of two answer cards ratified) — the founder ratified the error-field and layout-answer rows. Those rows stand ratified and are not provisional.

@history_item n=63 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract: 1 row) — `~ decision/checks-silent` to its current exception list (the order page logs). The silence census over the merge's new targets is read against this rule.

@history_item n=64 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (9. CI Integration: 5 rows) — added the build-directory sweep's stdout line contracts. These bear on the directive that a probe worktree stands outside `target/`, which the sweep manages.
