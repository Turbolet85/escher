# security-plan — amendments

One entry per amendment to `security-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-fork-ci-reached — signing and PR-token jobs repository-guarded; CI legs locked
**Section:** §Threat Model Summary (CI workflow triggered by pull requests) · §Authentication & Authorization (RBAC — the publish job row) · §Dependency Security (Pinning · Supply chain integrity — Signed artifacts, Lockfile verification) · §Logging & Monitoring (CI publish)
**Change:** was the publish job using "Signed Builds" only on main or `ci-test` branches — a ref-only gate — and post-results running for successful `pull_request` WPT runs; now `release-cli` and `post-results` (and the WPT jobs) carry `github.repository == 'DioxusLabs/blitz'`, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment, its secrets, a signed artifact or the PR-writing token; publish-build trace logging is upstream-only too. `--locked` now covers every ci.yml cargo leg (the leg script and the matrix) beside the flake and `dx bundle`, `examples/wasm_hello` excepted (no `Cargo.lock`). ci.yml references no `secrets.`, pinned by a unit test.
**Why:** the fork holds no secrets or environments, and a ref-only `if:` re-arms signing on the fork's own `main` and its existing `ci-test/sign-android-builds` branch; a repository guard keeps upstream byte-equal.
**Kept:** the write / `always()`-remove pairs for the signing key and keystore are unchanged.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 23 citations re-pointed — `publish-browser.yml` from line 37 on +1 and `wpt.yml` from line 26 on +1 (the repository guards), `ci.yml` by a range map over the rewritten file; no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — cargo-deny audit leg, SHA-pinned ci.yml, a read-only ci.yml token
**Section:** §Authentication & Authorization (RBAC — the ci.yml row) · §Dependency Security (Audit tool · Pinning · Update policy · CI integration · NOT YET MEASURED) · §Supply chain integrity (Lockfile verification) · §Bootstrap phases (`dep-audit-tooling-install` · `dep-security-ci-gate`)
**Change:**
- was "a workflow-level permissions block in ci.yml is observed absent"; now ci.yml declares `permissions: contents: read`, no job-level grant — every ci.yml `GITHUB_TOKEN` read-only, pinned by a unit test;
- was "Audit tool: observed absent" / "CI integration: no audit step"; now cargo-deny 0.20.2 runs `cargo deny --locked check advisories` as ci.yml's `audit` job on every push, slow tier, uncached, configured by the root `deny.toml`: `[graph]` the six ci.yml platforms with `all-features = true`; per-ID ignores only with a written reason, no blanket allow, `unmaintained`/`unsound` at their defaults;
- one ignore: RUSTSEC-2026-0192 (ttf-parser 0.25.1, unmaintained, no patched release), reached only through the exact winit beta pin — a bounded deferral the audit leg re-reads every push; RUSTSEC-2026-0285 fixed by the lockfile update rustls 0.23.43 → 0.23.45 (no build graph reaches rustls);
- the audit's reach is cargo-deny's resolved graph, not the lockfile: 0.20.2 prunes `http-cache` (blitz-net's `cache` feature) and `ravif`, so RUSTSEC-2024-0436 (paste 1.0.15) and RUSTSEC-2026-0186 (memmap2 0.5.10) never reach the gate;
- Pinning: was `awalsh128/cache-apt-pkgs-action` at `@latest` in ci.yml; now every ci.yml `uses:` pinned to a 40-hex SHA; the upstream-only publish-browser and wpt workflows keep `@latest`; ci.yml installs cargo-deny 0.20.2 and cargo-llvm-cov 0.9.1 through `taiki-e/install-action`;
- `--locked` covers the new audit, a11y and coverage legs; the NOT YET MEASURED note narrows to the critical-CVE response SLA; the Update-policy search is stated inline; both bootstrap keys read discharged.
**Why:** the CI gate legs chunk (cargo-deny chosen over cargo-audit at its P4 fork — cargo-audit fails on vulnerabilities only, leaving unmaintained and unsound findings unowned).
**Kept:** the guarded workflows' `permissions` rows and their `@latest` references are unchanged (upstream-only; a ref-only edit would re-arm them on the fork).
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — escher's log sink scrubs; logging-redaction-wire discharged for it; RUST_LOG read
**Section:** Data Protection (At rest) · Bootstrap phases · Secret Management (Environment values read) · Logging & Monitoring (Log format and backends) · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- `logging-redaction-wire`: was recorded absent; now discharged for escher's own sink (`escher_telemetry::init`'s allowlist scrub in `seven_guis_native`), still open for the upstream `fmt::init()` stdout subscribers and the WPT runner's `env_logger`.
- Logging & Monitoring gains escher's sink: stderr only; engine targets (`blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console`) print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted at any target; its reach is that sink only — the std panic hook's raw message and `log.file` host paths stay as-is; the examples' `println!`-only bullet names `seven_guis_native`'s sink.
- Data Protection: the s05 redaction-absent search stands, qualified with escher's sink.
- Environment values read gains `RUST_LOG` (not secret).
- 3 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk wired the scrub at escher's subscriber; the opt-in OTel export (egress + the `OTEL_EXPORTER_OTLP_HEADERS` credential path) was deferred at P4 by the overseer delegate under the founder's standing delegation of technical forks, provisional on the founder's word, and adds no surface here.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — `disabled` row: parsed for focus, presence for state and clicks
**Section:** §Input Validation → Markup attributes (`disabled`)
**Change:** was "Parsed as a boolean value; elements with it ignore pointer selection and click default actions"; now parsed as a bool for focusability only, while its presence alone — `disabled="false"` included — sets the DISABLED element state and makes the element ignore pointer selection and click default actions.
**Why:** the headless stand chunk measured `disabled="false"` matching `:disabled`; the row conflated the two readers.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — the agent-run argv boundary and harness log
**Section:** §Input Validation · §Logging & Monitoring
**Change:**
- §Input Validation: a `CLI arguments (agent-run.sh)` row — a verb outside `boot run status cleanup logs`, a wrong argument count, or a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem prints usage and exits 2 before any cargo call.
- §Logging & Monitoring: the agent-run contract logs harness metadata only — no content-named field, no captured test output or panic text in its events; `target/agent-run/run.log` holds raw cargo/libtest output, unscrubbed like `target/ci-logs/`, never printed, gitignored.
**Why:** the stand test contract chunk added an agent-invoked script whose argument is the one new input it takes and whose events are a new log.
**Kept:** no boundary widened — a selection reaches cargo only as an existing test-file stem; no port, socket or env var.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent login, input rows and log; PROVISIONAL
**Section:** §Input Validation · §Secret Management (Storage · What counts as secret · NOT YET MEASURED) · §Logging & Monitoring
**Change:**
- §Input Validation: a `CLI arguments (cold-agent.sh)` row — a verb outside `run status cleanup logs`, a wrong argument count, or a task outside the allowlist `counter` prints usage and exits 2 before any precondition or `claude` call; the task reaches the session only as a fixed prompt naming no element id. A `Cold-agent MCP stub` row — −32700 / −32601; refusals `not-found` · `disabled` · `not-pressable` · `malformed`, `isError` with no state change; no argument value logged; argv only.
- §Secret Management: a Development storage bullet — the cold-agent live run reaches the model provider through the operator's own Claude Code claude.ai login held by the `claude` CLI; `apiKeySource` none, no env read, no CI secret, evidence census 0. "What counts as secret" names that login. The development-secret-storage NOT YET MEASURED narrows: the login's holder recorded, its at-rest location unmeasured.
- §Logging & Monitoring: the cold-agent pipe logs counts and identities only; its transcript is raw by design, never printed, gitignored; the one committed copy host-path-masked.
**Why:** a credential path and a new IPC input surface are a security-plan amendment first. Ratified at this wrap by the overseer under the founder's standing delegation as a Boundary widening, PROVISIONAL in the body until the founder's own word.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/

## 2026-10-06-upstream-sync-element-identity — unknown alignment flags; merge citation re-point
**Section:** §Error Handling · every section citing a merged upstream file's lines
**Change:**
- Error Handling: was "unknown alignment flags are mapped to none rather than panicking"; now an unknown alignment flag is mapped rather than panicking — to Taffy's `AlignContent::NORMAL` for content alignment, to none for item alignment.
- 40 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no other claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; upstream's switch to Taffy's first-class `normal` keyword changed content alignment's fallback. The no-panic guarantee holds.
**Kept:** the three script-reachable accessors the merge brought (`document.children`, `textContent`, `CSSStyleSheet.disabled`) are realizations on the already registered script-to-DOM binding crossing, validated per §Input Validation's JS API rows — not a boundary widening.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-upstream-sync-element-identity — root-manifest citations re-pointed after the line-61 insert
**Section:** §Dependency Security (Pinning)
**Change:** 2 root `Cargo.toml` citations (the taffy and parley rev pins) re-pointed +1 — they read one line low since the `seven_guis` path entry was inserted at `Cargo.toml:61`; verified against the cited text. No claim text changed.
**Why:** the 2026-10-06-headless-stand wrap did not re-point the root-manifest citations past its insert; the operator chose at this wrap's escalation (2026-10-06) to fix them in this pass rather than carry them.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — the HTML `id` gains the stable-element-id reader
**Section:** §Input Validation (Markup attributes)
**Change:** a new `Markup attributes | id` row: besides CSS matching and `getElementById`, a Dioxus document's `element_id` / `element_ids` read the HTML `id` as an author key only when non-empty, `/`-free and the first in document pre-order; an empty, `/`-bearing or later-duplicate value gives no key and the element reads its `/`-bearing path, so no `id` makes two ids equal; a non-element, stale or detached node reads `None`; no path panics; the id is computed on demand, written nowhere, and carries no engine id or pointer.
**Why:** the chunk added an in-process reader of an already-admitted attribute; no new input class, crossing or write — not a boundary widening. The rule is the reader's validation.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-id-persistence — the CRUD row's id key is a model-assigned u64
**Section:** §Input Validation → Markup attributes | `id` (stable element id)
**Change:** the `id` row adds two facts. A keyed list row's segment carries its Dioxus key (`{tag}[{key}]`). The 7GUIs CRUD row's key is its person's model-assigned `u64` (fixture people 0–2, Create from 3) — never a list index, pointer, hash, clock or process-local value — so a row reads the same id across a re-render, a remount and a second process. A dropped pre-remount `NodeId` reads `None` with no panic. The rest of the row stands.
**Why:** v010-02 proves the id persists. A key from a pointer, hash or process counter would break cross-process equality and the row's no-pointer clause. Not a boundary widening: no new input class crosses the `id` surface — the key component was already in the grammar, and only the value the app feeds it changed.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-accessibility-tree-identity — the stable id reaches the platform accessibility API; accessible-name attributes row
**Section:** §Input Validation → Markup attributes `id` row · a new Markup attributes row (`aria-label` · `<label for>`) · citations re-pointed
**Change:**
- `id` row: was "the id is computed on demand, written nowhere"; now computed on demand and written to no log, DOM or vdom. Under `accessibility`, `DioxusDocument::accessibility_tree` carries it as each element node's AccessKit `author_id` (never on a `TextRun`, the document root or `Window`, never by indexing the slab with a tree id), so it leaves the process only through the platform accessibility adapter when an assistive technology is active — the crossing text runs and labels already take.
- New row: `aria-label` and `<label for>` are read from parsed content (the browser's remote HTML included) into the accessibility tree by an in-process reader. The parser already admits both, so no new input class enters. A whitespace-only `aria-label` names nothing; a label associates only when both it and its bound `<input>` have built nodes; no lookup panics; the names are logged nowhere and reach the adapter like any text run.
- 11 citations into the changed files re-pointed by the measured line map.
**Why:** the operator ratified the id's new crossing at this wrap (their own word, 2026-10-06, P2 escalation) — an existing crossing gaining a new author-derived data class, for AccessKit's test-automation purpose. The ratification set a CARRY on "Stand a11y assertions": assert that `author_id` never carries a `NodeId`, `ElementId` or pointer form on the platform tree. The name-source read was judged not a boundary widening at phase P4 (playbook read recorded in the plan). Standing rule: any further consumer of the id beyond the platform accessibility API re-opens this row.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-founder-rulings — cold-agent crossings ratified; OTel export out of escher 0.1.0
**Section:** Input Validation → cold-agent MCP stub row · Secret Management → Development (operator host)
**Change:**
- Cold-agent pipe: the stub's input row and the operator-login clause were "PROVISIONAL, overseer under the founder's standing delegation, pending the founder's word"; now "ratified by the founder, 2026-10-06". The credential path (the operator's own Claude Code login, `apiKeySource` none, no API key, no env read, no CI secret) and the stdio IPC surface are unchanged.
- Opt-in OTel export: was deferred, provisional on the founder's word, per the 2026-10-06-telemetry-bootstrap entry; now ruled out of escher 0.1.0 — no egress and no `OTEL_EXPORTER_OTLP_HEADERS` credential path ship, and neither the transport nor the credential path is decided. No body text changes: the body never registered either surface.
**Why:** the founder's own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional answers by rule. The OTel decision is held as a cross-version residual (`.andromeda/residuals.md`); a later version that takes it up registers the egress and the credential path here first.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/

## 2026-10-06-snapshot-model — the snapshot is an in-process reader of ids and names
**Section:** §Input Validation → `id` (stable element id) row · → `aria-label` · `<label for>` (accessible names) row
**Change:**
- `id` row: adds `DioxusDocument::snapshot` as a second, in-process reader of the `author_id` — no argument, the id copied verbatim into `SnapshotNode.id`, each tree id resolved through `BaseDocument::get_node` (never an index), an unresolved node omitted. It adds no crossing: no wire form, no log, event, socket or file, no driver, CLI or MCP command yet. The platform accessibility adapter stays the id's only exit; that clause and its ratification are unchanged.
- Accessible-names row: adds the snapshot as an in-process reader of the same names into `SnapshotNode.name` (the `label`, else the `labelled_by` targets' names, cycle-guarded), with no new input class and no crossing.
**Why:** a new consumer of already-admitted data is recorded where the row enumerates who reads it. Not a boundary widening: nothing new crosses and nothing new is admitted, the reading the operator approved in the plan at phase P5. Trap for later chunks: the first wire form of the snapshot (its serialization, then the driver) is a crossing for ids, names and a text control's current text, and must be escalated then.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/

## 2026-10-06-id-stability-across-code-edits — the `id` row states four tiers and a third in-process reader
**Section:** §Input Validation → `id` (stable element id) row
**Change:**
- Was "an unusable value gives no key and the element reads its component or document path"; now an unusable value gives no key and anchors nothing, and an unkeyed element reads one of three paths — anchored `{key}//{segment}`, component, document — with the anchor acting inside one component only. The uniqueness argument is three-way: a key holds no `/`, an anchored path is the only id holding `//`, a component or document path has no empty segment.
- A CRUD row's id was its Dioxus-key segment; now its author key `crud-person-{person.id}`, built from the same model-assigned `u64`. Home's seven cards read the fixed-slug keys `task-card-{slug}`.
- Adds `DioxusDocument::unkeyed_actionable` as a third in-process reader of the id: no argument, no log, ids from `element_ids()`, roles from `accessibility_tree`, an unresolved node skipped; an entry holds id, tag and role, never text, a name or an attribute value. It adds no crossing; the platform accessibility adapter stays the id's only exit.
- "Carries no `NodeId`, `ElementId`, `ScopeId` or pointer" is scoped to the id and the snapshot: an `UnkeyedActionable` holds a process-local `NodeId` in `node`.
- Five line citations re-pointed; citations to the anchor tests, `app.rs`, `actionable.rs` and `stand_actionable_keys.rs` added.
**Why:** the same admitted input read under the same author-key predicate, and a new consumer of already-admitted data — not a boundary widening. The anchored tier is ratified by the founder (the founder, 2026-10-06). Trap for later chunks: a wire form of the check is a crossing for ids and must leave `node` behind; escalate it then.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/

## 2026-10-06-snapshot-state-fidelity — a password's snapshot value is masked; the falsy clear and the `disabled` reader recorded
**Section:** §Input Validation → Markup attributes (`disabled` · `id` · accessible names · password `input` value) · → Dioxus mutations · §Error Handling → Graceful degradation · → Panic paths
**Change:**
- New row, password `input` value: the snapshot reads `NodeState::value` of a password input as the public constant `MASKED_VALUE` (eight U+2022, one fixed marker) where it holds a non-empty text and `""` where it holds none — never the text, never its length; the typed text is in no id, name or value of any snapshot node and in no label or value of any accessibility-tree node. Password-only and snapshot-only; no input class and no crossing added.
- `disabled` row: the snapshot's `enabled` follows the presence reading, never the focusability parse — `disabled="false"` and a bare `disabled` read `Some(false)` while both stay focusable.
- New Dioxus-mutations row: a falsy value of one of the 27 `BOOLEAN_ATTRIBUTES` names is removed, not written, on both attribute paths (the clear, of `checked` and `disabled` only before this chunk, was not recorded in this plan); a truthy value of a listed name and an unlisted attribute holding `false` are still written.
- 7 line citations re-pointed (`mutation_writer.rs` ×4, `snapshot.rs` ×3); the `dangerous_inner_html` citation, one line early before, now lands on the branch and its `set_inner_html` call.
**Why:** the chunk measured a password input's value reading its typed text in the clear and masked it before any wire form exists; the mask form is the operator's answer at phase (the operator, 2026-10-06). Neither change is a boundary widening: the mask narrows what the snapshot carries, the clear removes attributes on an existing in-process path. The 27-name clear is PROVISIONAL on the direction given at phase's P5 review (2026-10-06), pending the founder's own word at the Epoch 3 boundary. Trap for later chunks: the mask covers the snapshot only — by phase's code read, not measured, the engine still paints a password's characters and a file input's `value` (a host path under `file-input`) reads unmasked through the same reader; the first wire form re-reads both.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/

## 2026-10-06-compact-snapshot-serialization — the snapshot has one text form, returned to its caller only; a file input's value is masked
**Section:** §Input Validation → Markup attributes (`id` · accessible names · password and file `input` value · `disabled`) · §Data Protection → Local user data handled
**Change:**
- The `id`, accessible-names and masked-value rows each said the snapshot "has no wire form"; now each says it has one text form, `Snapshot::to_text` — an id written as a quoted `id=` field, a name as each line's quoted string, a masked value as the quoted `MASKED_VALUE`, all in `str`'s `Debug` form so a field never holds a line break or a bare `"` — returned to its caller only: no log, event, socket, file or command carries the snapshot or its text, `to_text`'s callers are tests, and the platform adapter stays the id's only exit. There is no JSON form, no `Display` impl and no parser of the text.
- The masked-value row was "Password-only"; now it covers an `input` whose `type` is `password` or `file` (each compared ASCII-case-insensitively): a non-empty value reads `MASKED_VALUE`, an empty one `""`, a file input with no `value` attribute `None`. The typed text and the path occur in no id, name or value of any snapshot node. The DOM is untouched: a file input's `value` attribute still holds the chosen file's host path and form submission still carries it.
- §Data Protection's file-path bullet adds that the attribute and form submission keep the path while the snapshot and its text read the mask — the reader's half measured on an authored `value`, the dialog's write of the path read from the code, not run.
- 2 line citations re-pointed (`snapshot.rs`) by the chunk's measured line map; the text form's citations added to the three rows.
**Why:** the chunk built the snapshot's first serialized form. A boundary-widening subject, so it was escalated at this wrap: the returned-value-only answer is PROVISIONAL — the operator's answer at the chunk's plan (2026-10-06), recorded as provisional on the operator's answer at this wrap's escalation (2026-10-07), pending the founder's word at the Epoch 3 boundary. The file mask is the operator's answer at the plan's question round (2026-10-06) and is not provisional. Standing rule: the first command that exposes the snapshot or its text is a new crossing for ids, names and values, asked again at that chunk.
**Kept:** the `id` row's second "no wire form", about `DioxusDocument::unkeyed_actionable`, stands — that check still has no serialized form.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/
