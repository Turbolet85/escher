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

## 2026-10-07-change-tracking-and-diff — the diff as an in-process reader; the platform tree delivered on change
**Section:** §Input Validation → the `id`, accessible-names and masked-value rows; §Data Protection → Local user data handled; `file:line` citations into `document.rs` and `mutator.rs`
**Change:**
- `id` row: the platform accessibility adapter receives the tree on change as well as on its initial request — `View::poll` takes the document's changed set and, under `accessibility`, rebuilds the platform tree when it was non-empty (was: built once; the poll-time refresh did not run) — the same crossing and content classes (ids as `author_id`, accessible names), more often; no windowed witness on the dev host. `Snapshot::diff` is a fourth in-process reader of the id, through the snapshot model only: ids in `DiffNode.id`, `DiffNode.parent` and `SnapshotDiff.removed`, no `NodeId`, `ElementId`, `ScopeId`, AccessKit id or pointer, no text form, returned to its caller only — the platform adapter still the id's only exit.
- Accessible-names row: names reach the adapter on change as well; `Snapshot::diff` reads the resolved names into `DiffNode.name`, with no new input class and no crossing.
- Masked-value row: a diff takes its values from the snapshots only, so a password's or file input's value in a diff entry is `MASKED_VALUE` — measured for a password input, by the same construction and not separately run for a file input.
- Data Protection: a diff of two snapshots joins the snapshot and its text as not holding a chosen file's host path — by construction, not run.
- 11 of 17 citations into the two blitz-dom files re-pointed by the measured line map.
**Why:** the chunk added the diff and made the shell's poll-time refresh run. The refresh on change is a boundary widening: PROVISIONAL, pending the founder at the Epoch 3 boundary (the operator, 2026-10-07, at the chunk's plan and again at this wrap's escalation: record as provisional). The diff adds no crossing; its returned-value-only exit carries the snapshot text's provisional answer. Standing rule: the first command that returns a diff asks the crossing question again.
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-audit-corrections — the refused Dioxus key and its two witnesses
**Section:** §Input Validation → rows `id` (stable element id) · password and file input value (`file:line` citations)
**Change:**
- `id` row: a keyed list row's segment carries its Dioxus key (`{tag}[{key}]`), and a Dioxus key that is empty or holds `/` is refused — the row reads the positional `{tag}:{n}` segment instead. Each of the two cases is witnessed by a unit test in `dioxus_document_tests.rs` (was: the row said the segment carries the key and did not state the refusal).
- 6 of 15 citations into the edited files re-pointed by the measured line map (five in the `id` row, one in the masked-value row); 9 keep their numbers.
**Why:** the Epoch 2 code audit found the key filter's `&&` could become `||` with no test failing; the chunk added a named witness for each of the filter's two conditions, so the row can now state the refusal on evidence. No input class, crossing or validation mechanism changed.
**Kept:** the full segment grammar (a repeated key falling back to an index, the first-among-siblings rule) stays architecture's to state; this row names only the two refusals the witnesses prove.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/

## 2026-10-07-driver-session — the driver session socket: threat vector, access, input rows
**Section:** §Threat Model Summary → Attack surface (new vector: local IPC) · §Authentication & Authorization (new row) · §Input Validation (four new rows: session socket · state directory · label · `escher-session` argv) · §API Security (lead · new control row) · §Secret Management → Environment values read · §Dependency Security → Pinning (citations)
**Change:**
- API Security lead: was "No served API surface exists … listeners are observed absent"; now no served network API and no TCP or UDP port, with one local listener — the driver session's Unix-domain socket `session.sock`, lifecycle messages only.
- Access: no handshake, token or peer-credential check; the state directory is created `0700`, one open to group or others is refused (`StateDirNotPrivate`), the socket is `0600` — the owning user only.
- Validation: requests parsed against a closed grammar (`hello v1`, `stop v1`), bounded at 64 bytes, a 2 s read and write bound, refusals and broken connections changing nothing; replies carry a pid, the label and a count only; the label is 1-32 bytes of `a-z0-9-`, checked before the boot; the binary's argv is closed (two arguments, four tasks) and exits 2 before anything boots; `stop` removes the socket file and the directory, nothing else.
- Environment: two build-time `env!` values in the session checks; the library and the binary read no env var beyond `RUST_LOG`.
- Citations: root `Cargo.toml:104; :114` → `:106; :116`; the blitz-tests dev-dependency range → `:15-40`.
**Why:** a new listener and a new input surface are a security-plan amendment first. A boundary widening, ratified by the founder's own choice (the founder, 2026-10-07, relayed verbatim). Standing: the wire carries nothing of the screen; a command that carries an id, a name, a value, snapshot text or a diff reopens the crossing question.
**Kept:** no idle expiry and no pid check in `start` — recorded as built; a TCP or UDP port was ruled out (it needs an auth surface the version excludes).
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-driver-session — what a sink-installing host logs: the id and name rows and the scrub's reach scoped to the measurement
**Section:** §Input Validation → Markup attributes (`id` · accessible names) · §Logging & Monitoring → Log format and backends (escher's own sink · Stdout output) · §Bootstrap phases → logging-redaction-wire
**Change:**
- Accessible names: was "the names are logged nowhere"; now escher's own code logs no name, a host that installs the sink writes none at the default level or at `info`, and at `trace` a name prints on its stderr through `dioxus_core::diff::node`.
- `id`: was "computed on demand and written to no log, DOM or vdom"; now escher's own code writes it to none, while an author-key id — the element's HTML `id` — prints on such a host's stderr at `debug` and `trace` through Stylo's records; the three "the id's only exit" clauses now read "the only exit escher's own code gives the id".
- The sink: both seven_guis binaries install it; the past-the-scrub list gains the class "a record from a target outside the engine allowlist prints as written" with the reading on `escher-session` (0 lines at `warn`; 1 line, no id, no name at `info`; ids at `debug`; ids and names at `trace`; stdout empty). Typed text and the windowed stand by level are stated as not measured.
- logging-redaction-wire: was discharged for escher's own sink; now discharged for engine targets and content-named fields, open for third-party targets, owed by the route entry "Sink target allowlist".
**Why:** the chunk measured the session host's stderr by level. The founder ruled to record it honestly and fix it in the chunk right after (the founder, 2026-10-07, relayed verbatim). Trap: a "nothing in the logs" check over a process that installs no sink passes vacuously — `stand_session_quiet`'s host is one.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-driver-session — founder's rulings: the Epoch 3 PROVISIONAL items ratified
**Section:** §Input Validation → Markup attributes (`id` · accessible names · password and file `input` value)
**Change:** eight clauses in the three rows read PROVISIONAL (the operator's answer, pending the founder's word at the Epoch 3 boundary, or "as the `id` row records"); now "ratified by the founder, 2026-10-07" — the shell's refresh of the platform tree on change (its windowed witness still owed, and the row says so), and the snapshot, its text and its diff being returned to their caller only. No other text of those clauses changes.
**Why:** the founder ruled on both items at the Epoch 3 boundary (the founder, 2026-10-07, in the overseer session, relayed verbatim by the overseer), superseding the operator's provisional answers by rule.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-sink-target-allowlist — a sink-installing host logs no id and no name at any level: outside-target records dropped
**Section:** §Input Validation → Markup attributes (`id` · accessible names) · §Logging & Monitoring → Log format and backends (escher's own sink) · §Bootstrap phases → logging-redaction-wire · §Data Protection · §Secret Management → Environment values read
**Change:**
- The sink: was a two-outcome scrub in which "a record from a target outside the engine allowlist prints its message and fields as written" (per "2026-10-07-driver-session — what a sink-installing host logs: the id and name rows and the scrub's reach scoped to the measurement", whose other claims stand); now three outcomes — engine targets print only the safe fields, `escher_` targets print with the eleven content-named fields redacted, any other target's record is dropped whole, zero bytes, at every level, WARN and ERROR included, whatever `RUST_LOG` names, a bridged record judged by the target it was logged under.
- Readings, stderr lines before → after the drop: `escher-session` (CRUD, one `hello`, then `stop`) 0 → 0 at the default level, 1 → 1 at `info`, 1165 → 1 at `debug`, 1501 → 1 at `trace`; the windowed `seven_guis_native` (Home, 10 s) 1 → 0, 3 → 1, 12,413 → 1, 47,482 → 1; the line left is the install line; stdout empty in all 16 readings.
- `id` row: an author-key id prints at no level — id needles found 15 of 15 → 0 (host) and 11 of 11 → 0 (windowed) at `debug` and `trace`; the ×12 / ×20 figures for one id are retired.
- Accessible-names row: no name at any level — name needles at `trace` 6 of 6 → 0 (host), 5 of 5 → 0 (windowed).
- "the windowed stand not measured by level" is retired at its three sites; typed text stays not measured.
- `log.file`: a host path can ride only a bridged record under an engine target — recorded by construction, not measured.
- logging-redaction-wire: discharged for escher's own sink, the third-party-target clause included; still open for the upstream apps' subscribers and the WPT runner's logger.
- Data Protection names the sink as redacting and dropping. Two citations re-pointed; the session checks' build-time env citation names both host-spawning files and their shared module.
**Why:** the chunk delivered the fix the founder ruled should follow the host-log finding (the founder, 2026-10-07, relayed verbatim by the overseer, as the earlier entry records); dropping every level was approved by the operator at the plan review (the operator, 2026-10-07). Cost recorded in the body: a third-party WARN or ERROR no longer prints. Trap: a "nothing in the logs" check still has to run against a host that installs the sink; `host_log` is that check.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/

## 2026-10-07-settle-detection — a settle reports a load in flight and never waits on one
**Section:** §API Security (a new row, "Settle and loads in flight", beside "Request timeout")
**Change:** a harness settle (`Harness::settle`, and the driver session's `act` over it) reports a load in flight at once — `NotSettled` naming the class `Loads` — and never waits on one; it reads no clock, so the absent request timeout cannot hang it, and its pass loop is bounded at 64 passes; the outcome is a returned value naming a class only — no URL, request, node, id, name or value — and the counter it reads stores counts only. The "Request timeout" row is unchanged: a request still has no timeout.
**Why:** the chunk added a wait over a document that can issue requests with no timeout, and the bound and the no-clock rule are what end it. Rule for later chunks: a driver command that would wait on a load with a clock is a new decision for this plan, not an extension of settle; and the session socket's wire is untouched — a settle verb or a busy-source reply on it is a new crossing question.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-command-and-refusal-schema — a driver call is validated before anything runs; a refusal holds nothing of the call
**Section:** §Input Validation (new row: Driver command schema) · §API Security (the "Settle and loads in flight" row) · §Error Handling → Error format (typed errors)
**Change:**
- §Input Validation gains the row "Driver command schema (escher-driver)": `validate` checks a caller-built call against a closed table of five verbs and five argument kinds with bounds (`id` 1 to 1024 bytes, `text` 0 to 4096, `key` one of twelve names, `flag`, `milliseconds` 1 to 60000) and returns a typed `Command` or a `Refusal`, one answer per call in a fixed order; no `unwrap`, `expect`, `panic!`, `unreachable!` or index on what a call supplies; no clock read; no `Session` in its signature, so a refused call changes nothing. Held in process: not yet an external-input surface. `Command`, `Call` and `ArgValue` print an id and typed text under `Debug`; nothing prints, logs or fields them.
- §API Security: the settle row now records that no verb has an argument or a result field of a wait, a not-settled step is a result and not a refusal, time moves only by `advance` (1 to 60000 ms a call), and no driver command waits on a load. Its stack reference reads `blitz-test-harness · escher-driver`.
- §Error Handling gains the `Refusal` bullet: a closed set of eight causes with fixed non-empty texts holding no path, a `Fault` carrying at most the schema's own argument name, no `String` field in `Cause`, `Fault` or `Refusal`.
**Why:** the chunk built the check every later driver surface's input passes through. That no driver command waits on a load is the operator's answer, given at the plan's forks on 2026-10-07. Rules for later chunks: the first surface that reaches `validate` from a socket, a CLI or an MCP tool is its own crossing question; keep every refusal text fixed and free of call content.
**Kept:** the detector graded these `escalate` by its own severity while reporting its invariant holding; each is a change the plan's reviewed list names, and nothing crosses a boundary, so none was escalated.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/

## 2026-10-07-act-by-id — the command schema is executed in process; an id is taken in; typed text still not measured
**Section:** §Input Validation (the Driver command schema, `id`, accessible-names, password-and-file value, session label and CLI arguments rows) · §Threat Model Summary → local IPC entry point · §Secret Management → Environment values read · §Logging & Monitoring → Log format and backends
**Change:**
- Driver command schema: was "held in process only … not yet an external-input surface"; now executed in process by `Session::run`, which validates first — nothing public runs a `Command` that did not come from `validate`; an id that names no element is `not-found` and an `advance` with no time step `time-unavailable`, each with nothing run; the reported time is clamped to the `ms` asked; no panic path on what a call supplies; still reached by no socket, CLI or MCP tool, so still not an external-input surface. `Outcome` joins the `Debug`-deriving types nothing prints, logs or fields.
- `id` row: its first consumer that takes an id in — the driver's `click` and `type`, bounded 1 to 1024 bytes by `validate`, looked up by a linear search of `element_ids()` and read through `get_node`, never by index, anew on every call. The snapshot text and the diff are returned by driver commands in process, to the caller of `run`; `to_text` and `diff` each have one caller outside the tests. Was "no driver, CLI or MCP command exposes it yet, and its callers are tests".
- Accessible-names and password/file rows: was "reaches no log, event, socket, file or command"; now no log, event, socket or file — the driver returns text and diff to its caller in process. A password typed through the driver occurs 0 times in what the call returns and reads the mask.
- Typed text (accessible-names row, the sink bullet): was "no command can type yet"; now a typing command exists and types into an instance held in process, where no sink is installed; none reaches a sink-installing host, so typed text in a host's log stays not measured.
- Five citations into `session.rs`, `session_host.rs` and `session_common/mod.rs` re-pointed.
**Why:** the chunk built the executor. Its eight wording proposals carried the detector's own `escalate` grade while the detector reported its invariant holding; they were applied without a halt under the playbook rule appended at this wrap (the founder, 2026-10-07, relayed verbatim by the overseer). The crossing question was answered before the plan: a call runs in process and nothing of it goes on the socket (the operator, 2026-10-07, at the plan's forks).
**Kept:** the rule that a snapshot's text and a diff are returned to their caller only, with its ratification (the founder, 2026-10-07), word for word; the `unkeyed_actionable` clause; the wire rows; "nothing prints, logs or sends" a `Refusal`.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — six verbs; the detected causes after `validate`; the session holds id text
**Section:** §Input Validation → Driver command schema · Markup attributes `id` · Driver session label (citation) · URL fragment (citation) · §API Security → Settle and loads in flight · §Error Handling → the `Refusal` bullet · one `session_common` citation
**Change:**
- Driver command schema: a closed table of six verbs (was five; `scroll`, one required `id`, through the same `validate`). After `validate` the executor refuses with nothing run: an id the screen does not read as `stale` or `not-found` (was `not-found` alone), then a `click` or a `type` on a target that cannot take it as `disabled`, `off-screen` or `covered`, in that order; the snapshot and both focus readings are equal after each. `scroll` applies none of the three and says `in_view`. All eight causes are returned by code.
- `id` row: `scroll` joins `click` and `type` as taking a caller-supplied id; five acting commands return the diff (was four). The session is a new holder of the id: a record of id text, at most 4096 ids, session-lived, private, never printed or logged, fed from its own snapshots (was "keeps no id"). The bound counts ids, not bytes: an app id longer than 1024 bytes is recorded whole. "No `NodeId` in any field" stands.
- Settle row: six verbs. `Refusal` bullet: two texts of `off-screen` reworded, its remedy naming `scroll`. Twelve citations re-pointed.
**Why:** the chunk built the detection. The sixth verb widens a validated surface: ratified by the founder (2026-10-07), his own choice relayed verbatim by the overseer and confirmed by the operator at this wrap's escalation. The record is the operator's decision at the plan's forks; the remedy's rewording is the founder's ruling, the meaning's the operator's directive. Still in process: no crossing is added, and the first command that carries a call or an outcome out of the process asks the crossing question.
**Kept:** the unbounded id length is stated as built and owned on the route; the plan's "about 4 MiB" is not written — it is no property of the code.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-07-driver-command-spans — what the driver logs: one span, eight fixed-word or count fields
**Section:** §Input Validation (the Driver command schema row; the `id` row, three citations) · §Error Handling (the `Refusal` bullet) · §Secret Management (one citation)
**Change:**
- Driver command schema row: was "nothing in the crate prints, logs or fields any of them"; now the crate prints nothing, fields none of `Command`, `Call`, `ArgValue`, `Outcome`, and fields one `tracing` span per call handed to `Session::run` — target `escher_driver`, name `command`, INFO — with eight fields: `verb` (the table's word, never the caller's text), `cause`, `settled`, `busy`, `passes`, `added` · `removed` · `changed`. Never fielded: any argument, `Refusal` or `Fault`, the screen text or its length, the label, the record of ids, `in_view`, `advanced_ms`. No subscriber installed by the crate; no env var and no clock read.
- `Refusal` bullet: was "nothing prints, logs or sends one yet"; now exactly one thing of a refusal reaches a log — its cause's fixed name, as the span's `cause` field, recorded at one site for every refusal `run` returns. No `Fault`, meaning, remedy or `Display` text is fielded.
- Citations into `execute.rs` and the telemetry `lib.rs` re-pointed.
**Why:** the chunk built the span the route entry owed. A cause name holds nothing a call supplied, so the logged value cannot.
**Kept:** "reads no clock" — the timings on a printed line are the sink's. "Not yet an external-input surface" — no socket, CLI or MCP tool reaches `validate`.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-07-driver-command-spans — the sink's second record class and the diff's lengths, PROVISIONAL; typed text read in process
**Section:** §Logging & Monitoring (escher's own sink) · §Input Validation (the `id` row; the accessible-names row)
**Change:**
- escher's own sink prints a second record class, PROVISIONAL: one line per closed span of an admitted target, nothing before the span closes, every pair — `span` included — judged by the rule that judges an event's field (engine target: the seven safe fields; escher target: all but the eleven content-named fields); an outside target's span writes no byte; a content-named span field's value is never stored, every other span field's value is held in process memory until the span closes.
- A named limit, read from the locked `tracing-subscriber 0.3.23` and not exercised: when a field formatter returns an error at span creation the layer `eprintln!`s the span's attributes with `Debug`, unscrubbed. The sink's field formatter returns no error.
- The `id` row: beside the founder-ratified "a diff is returned to its caller only, no log, event, socket or file carries it", not reworded, the body states, PROVISIONAL, that the three list lengths of a diff the driver returns are recorded on the command span and printed by the sink at `info` or below.
- Typed text: was "not measured … types into an instance held in process, where no sink is installed" (two sites); now measured in process only — a sink capture of the six verbs and a refused call per cause holds 0 occurrences of the three supplied texts, 0 of 36 id needles and 0 of 8 name needles, in both layout modes — and still not measured in a sink-installing host's log. The session host's by-level reading re-measured unchanged.
**Why:** a new record class is a boundary widening on the operator's answer; the diff clause is the founder's ratified sentence. The operator kept both marks at this wrap; they wait for the founder's batch at the Epoch 4 boundary.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-09-founder-rulings — the sink's second record class and the diff's sizes in a log, ratified
**Section:** §Logging & Monitoring (escher's own sink) · §Input Validation (the `id` row)
**Change:**
- escher's own sink: the second record class — one line per closed span of an admitted target — was "PROVISIONAL: a boundary widening on the operator's answer … not yet the founder's word", per the entry `2026-10-07-driver-command-spans — the sink's second record class and the diff's lengths, PROVISIONAL; typed text read in process`; now "a boundary widening, ratified by the founder (2026-10-09) as built". What the line prints, and the named limit of `tracing-subscriber 0.3.23`, are unchanged.
- The `id` row: the clause beside "a diff is returned to its caller only, no log, event, socket or file carries it" was "PROVISIONAL: stated beside the ratified sentence, which is not reworded"; now "ratified by the founder, 2026-10-09": the sentence stands and gains the clarification that a diff's content never reaches a log while its three sizes — the counts of added, removed and changed — may.
**Why:** the founder ruled on both at the Epoch 4 boundary (the founder, 2026-10-09, relayed verbatim by the overseer as his chosen option labels; the English wording of each ruling is the overseer's); a boundary widening is ratified by his own word only, and this is it. Standing rule for later chunks: a count of a diff's nodes may reach a log, no node, id, name or value of it may — a field that holds any of those is a new crossing question.
**Kept:** the same day the founder ruled that an in-process driver call returning the snapshot text and the diff is not a boundary widening — it is the returned value his rule allows. No mark waited on that answer; the clauses that state the in-process return stand as written.
**Ref:** .andromeda/runs/2026-10-09T19-17-00-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — an Input Validation row for the CI install script's arguments
**Section:** §Input Validation
**Change:** a new row, `CLI arguments (apt-install.sh)`, beside the `agent-run.sh` and `cold-agent.sh` rows: package names and option values from a ci.yml `run:` step; a package name must match `^[a-z0-9][a-z0-9+.-]+$` and an option value (`--attempts`, `--bound`, `--pause`) `^(0|[1-9][0-9]{0,5})$` — at most six digits, no leading zero — with `--attempts` and `--bound` at least 1 and at least one package named; anything else (no package, a name outside the form, an unknown option, an option with no value) prints `usage: …` to stderr and exits 2 before `sudo`, `timeout` or `apt-get` runs; the script reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable; ci.yml's nine calls pass package names and no option; pinned by `InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs` over nine bad calls. No existing row changed.
**Why:** the chunk added a script that takes arguments and runs `sudo`, and §Input Validation lists script argument boundaries row by row. The detector's grade was `escalate` by its own severity while it reported the validation present, and the plan's reviewed list named the row, so it was applied without a halt under the playbook's rule for that class. It is a new script the repository's own workflow steps call — no already-hardened boundary gained a crossing or an input class.
**Kept:** §Dependency Security is unchanged: no dependency or action was added, and the apt-cache action keeps its pinned SHA.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — a hint-less `@font-face` source is fetched and sniffed: a boundary widening by upstream merge, PROVISIONAL
**Section:** §Input Validation → the `@font-face` source | URL row
**Change:** the row keeps its claim (a source whose URL cannot be resolved is skipped instead of panicking) and gains a PROVISIONAL clause: a source with no format hint whose URL has no extension — a `data:` URL — was skipped and is now fetched, its format sniffed from its bytes as the Fetched fonts row states; no test of ours and none in the merged delta covers it; escher routes no agent- or user-supplied URL through this path today — no driver verb takes a URL, and the stand boots offline on its bundled font.
**Why:** upstream #1109 arrived with the merge and widens which document-supplied font sources reach the fetch and the font decode. A boundary widening is the playbook's never-routine class whatever brings it: the operator, 2026-10-10, through the question dialog of this wrap, had it recorded as PROVISIONAL — the call is the founder's own word, also when the widening arrives by an upstream merge. The mark is discharged in the batch at the Epoch 5 boundary, on the founder's ruling; until then it stands in the body, in the security rule file and summary, and in the handoff.
**Kept:** the Fetched fonts | Format row is unchanged — sniffing "when no format hint is given" was already its claim. No code was changed for this: the merged behaviour stands as upstream wrote it.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — input rows re-read on the merged engine: the `file:` read, `autofocus`, the script Selection API; one compile-time env read
**Section:** §Input Validation → `file:` URLs (net provider), Markup attributes, JS API arguments · §Secret Management → Environment values read
**Change:**
- `file:` URLs: was "Read with `std::fs::read(request.url.path())`"; now read through `Url::to_file_path` off wasm — a `file:` URL that is no file path is a typed `InvalidInput` error, no panic — and through `request.url.path()` on wasm only. No path restriction, as before.
- Markup attributes, new row `autofocus`: on a focusable element, under blitz-dom's `autofocus` feature, the attribute enables autofocus when present with any value but `"false"` — bare, empty and `"true"` each focus the element.
- JS API arguments, new row Selection: `setBaseAndExtent` checks its argument count and offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection; the `innerText` / `outerText` getters only read; escher uses none.
- Environment values: the `env!("CARGO_MANIFEST_DIR")` bullet also names upstream's `tests/all.rs` of blitz-tests, a target held at `test = false` and never built. No runtime env read was added.
**Why:** each row states a reading of code the merge changed. The `file:` change narrows — an error where there was none — and leaves the standing gap, no path restriction, as it was. The Selection API is a realization on the script-to-DOM crossing scripts already write through, so it is an inventory row, not a widening.
**Kept:** the Dioxus boolean-attributes row stands: the bridge still removes a falsy `autofocus`, so a Dioxus `autofocus: false` never reaches the engine's presence reading.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — dependency security after the sync: Parley's pin form, the entering crates, an ungated licence table, the apt action gone upstream, a second `unsafe`
**Section:** §Dependency Security → Audit tool, Pinning, Unsafe code, Supply chain integrity
**Change:**
- Pinning: was "Git dependencies are pinned by commit rev", citing taffy and parley; now the one git dependency, taffy, is pinned by commit rev and parley is a registry version, "0.12".
- Pinning, new bullet: the sync took upstream's dependency set whole, no `cargo update`; the root manifest gains icu_casemap, icu_locale_core, icu_properties and icu_segmenter "2.3" and writeable "0.6"; six names enter the lock (core_detect, multiversion_no_op, icu_casemap, icu_casemap_data, memchr-n, fearless_simd_macros), each from crates.io with a checksum and reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, its source beyond the manifest not read; jetscii and tinyvec_macros leave; `advisories ok`, no ignore added or dropped.
- CI tooling: was "the upstream-only publish-browser and wpt workflows still reference [the apt-cache action] at `@latest`"; now they no longer use it — each installs with a bare `apt-get` line behind its repository guard.
- Unsafe code: was "one `unsafe` block" in stylo_taffy; now two — the second in its public `resolve_calc_value`, which reads a calc value back through a raw pointer.
- Audit tool: `deny.toml` also holds a `[licenses]` table the audit leg does not read.
- Supply chain integrity: the NOT YET MEASURED note narrows to SBOM generation and base image scanning; a new Licence compliance bullet — upstream's table (13 allowed expressions) byte-identical in `deny.toml`, no CI leg runs it, one reading `licenses ok` over the six-target graph with two unmatched allowances.
**Why:** all of it arrived by the merge. Upstream's `licenses` CI job was taken by the merge and reverted — the operator's decision of 2026-10-10 at the plan: no licence gate yet, one reading into evidence. Whether the fork gates on licences is left open on purpose: the operator, 2026-10-10, as the arguments of this wrap, pinned the question for the founder on the route entry "Quality gates".
**Kept:** `[graph]` and `[advisories]` are the chunk start's; the audit's reach is still cargo-deny's resolved graph.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-driver-cli — the session socket carries calls and answers: the trust boundary, the socket row and the API rows restated
**Section:** §Threat Model Summary → the local IPC vector (Entry point · Trust boundary) · §Input Validation → Driver session socket · Driver session state directory · §API Security (opening paragraph · Local session socket · Settle and loads in flight)
**Change:**
- Trust boundary: was "the wire is the lifecycle only … carries no element id, accessible name, control value, snapshot text or diff"; now it carries calls in and answers out — an id and typed text in; ids, names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out; no `NodeId`. The boundary is still the owning user, by file mode alone.
- Entry point: the host role is `serve <task> --session <dir>`, run by hand or spawned by the binary's own `start` command through `std::env::current_exe()`; the two-argument form is gone.
- Driver session socket: was `hello v1` and `stop v1`, a 64-byte request, a reply carrying a pid, label and count only; now wire `v2` with `call v2 <verb> [<name>=<kind>:<value>]…` decoded strictly, a 16,384-byte request bound, a decoded call run through `Session::run` and nothing else, answers `accepted` · `refused` · `oversize` bounded at 1,048,576 bytes, a 30 s read of an answer. A request outside the grammar or over the bound still runs nothing.
- State directory: removed at `stop` or at the host's idle expiry.
- API Security: the listener serves the lifecycle and driver calls; the Local session socket row states the new bounds and the idle expiry — was "there is no idle expiry", now a host ends as on `stop` after 1,800 s with no answered request, an `attach` or a `status` restarting the count; the Settle row reads over the six instance-level verbs and names the three session-level ones' fields.
**Why:** a second widening of the 2026-10-07 boundary, ratified by the founder (2026-10-10), by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Сокет несёт команды» (the English wording is the overseer's), and confirmed not PROVISIONAL by the operator at the plan's review. The bounds were proposed by the builder and stand unchanged by that answer. Trap for later chunks: a watcher that polls `attach` or `status` keeps a host alive.
**Kept:** no handshake secret, token or peer-credential check; one connection at a time.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — the command line is an external input surface; the command schema is reached from outside the process
**Section:** §Input Validation → CLI arguments (escher-session) · Driver command schema (escher-driver)
**Change:**
- CLI arguments: was a closed argv of exactly two arguments, usage exit 2, an error exit 1, nothing on stdout; now the argv is read by `escher_driver::command_line` — a closed reader with its rules in a fixed order, the call checked by `validate` or `validate_session` before the session address is asked for, `--session <dir>` exactly once, an argument that is not text a usage error, no panic path on argv; four endings with statuses 0 · 1 · 2 · 3 and the host role's quiet end; no ending writes back anything the caller typed; an answer on stdout holds ids, names and values by the ratified crossing; `start` re-runs the binary with its streams closed and neither kills nor reaps the host. The row's two citations of the old `main` are replaced by the reader's.
- Driver command schema: was six verbs and five argument kinds; now nine verbs in two levels — `start`, `status`, `stop` session-level, refused by `validate` as `unknown-verb` and checked by `validate_session` — and six kinds, `name` the sixth (1 to 32 bytes of `a-z`, `0-9`, `-`).
- Driver command schema: was "still no socket, CLI or MCP tool reaches it … not yet an external-input surface"; now two external surfaces reach it, each through `validate` first — the socket's `call` request and the command line — and no MCP tool yet.
- Driver command schema: "the crate prints nothing … reads no env var and no clock" narrowed to the executor, session, schema, validation and JSON writer; the one writer is the command line, the one clock the host's, for the idle expiry. No span field was added.
**Why:** the chunk put the schema behind two inputs that come from outside the process. Both are the founder's ratified answers of 2026-10-10 (what the agent runs, named flags, four exit classes), relayed verbatim by the overseer as his chosen option labels «Бинарник приложения, путь» · «Именованные флаги» · «Четыре класса» (the English wording is the overseer's); the three session-level verbs and the sixth kind widen the validated verb set under the same ratification. Standing rule: every later surface's input still passes `validate` before anything runs, and a `Refusal`, a usage line and a session error hold nothing a caller supplied.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — ids, names and values leave in an answer; typed text measured in a host's log; a refusal has a written form
**Section:** §Input Validation → Markup attributes · `id` · `aria-label` · `<label for>` · password and file `input` value · §Error Handling → Error format (the `Refusal` bullet) · §Logging & Monitoring → Log format and backends (escher's own sink · Stdout output)
**Change:**
- `id`: was "returned to its caller only … nothing of it goes on the socket … the platform adapter stays the only exit", for the snapshot text and for the diff; now a hosted answer carries both over the session socket and onto the client's stdout, the driver writing a diff as JSON, and the id has two exits, the platform adapter and the driver's answers. `unkeyed_actionable` still has no command and adds no crossing of its own.
- `id`, taken in: the id a call names now comes from argv (`--id`) and from the socket's `call`, bounded 1 to 1024 bytes by `validate` on each side.
- `id`, the record: was "one longer than 1024 bytes … is still recorded whole … not measured"; now such an id is not recorded, with a unit test.
- Accessible names: now leave the process in an answer; still reach no log.
- Password and file value: values cross in an answer, a password's or a file input's as `MASKED_VALUE` — measured for a password, 0 occurrences of the typed text; a file input's by construction, not separately run.
- Typed text in a host's log: was "not measured", in three rows; now measured — 0 occurrences of a typed sentinel, 0 id and 0 name needles on the `escher-session` host's stderr at `trace`, under both builds, with one command-span line per hosted call; the by-level reading is stated.
- `Refusal`: was "held in process only — nothing prints or sends one"; now written by `Refusal::to_json` from its cause's fixed strings, sent on the socket and printed on stdout with status 1; a `SessionError` likewise, with a fixed `kind` word; four fixed texts amended, the cause set still eight.
- Stdout output: was "`escher-session` writes nothing to stdout on any path"; now one line of JSON per command, usage and a session error's message on stderr.
**Why:** each sentence said the opposite of the ratified crossing (the founder, 2026-10-10, relayed by the overseer). The typed-text proof was owed since the sink fix, due at the first command that types into a sink-installing host.
**Kept:** no log, event or file carries an id, a name, a value, the snapshot's text or a diff's content.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — the hint-less `@font-face` source: the PROVISIONAL mark comes off, ratified
**Section:** §Input Validation → `@font-face` source
**Change:** was "PROVISIONAL — a boundary widening that arrived with the 2026-10-10 upstream merge (upstream #1109), awaiting the founder's ruling", per the entry "2026-10-10-upstream-sync-agent-surfaces — a hint-less `@font-face` source is fetched and sniffed: a boundary widening by upstream merge, PROVISIONAL"; now the widening is ratified by the founder (2026-10-10) as upstream's behaviour — a source with no format hint whose URL has no extension, a `data:` URL, is fetched and its format sniffed from its bytes. The row keeps what is still owed: no test of ours and none in the merged delta covers the path, and a check of ours is owed before 0.1.0 ships, owned by the route entry "Quality gates". The routing sentence stands: no driver verb takes a URL, and the stand boots offline. Only the mark is retired; every fact of that earlier entry stands.
**Why:** the founder's own ruling, given by question dialog in the overseer session on 2026-10-10 and relayed verbatim by the overseer as his chosen option label «Утвердить + проверка» — ratify, with a check; the English wording is the overseer's — and named by the operator as this wrap's arguments. It was the one standing PROVISIONAL mark, ruled on ahead of the Epoch 5 boundary. No code of this chunk touched the path.
**Kept:** escher still routes no agent- or user-supplied URL through `blitz-net`; that rule is unchanged by the ratification.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — three of the driver's fixed texts restated
**Section:** §Error Handling → Error format (typed errors), the escher-driver command refusal item
**Change:** was "`covered`'s meaning now ends with the sentence that a hit reaches content scrolled out of a scrolling box, and three help texts say the scrolled-box bounds limit and that `type` replaces"; now three of the four texts amended at 2026-10-10-driver-cli are restated — `covered`'s meaning ends with the sentence that a hit reaches content clipped by `contain: paint`, the help of `changed` and of `snapshot`'s `text` read as they did before that chunk and state no limit — and `type`'s help still says that it replaces. The cause set stays eight in its order, and every text is a fixed string holding nothing a call supplies.
**Why:** the two engine limits the texts stated are fixed in the engine. No boundary moves: no input surface, verb, argument, result field or cause was added, and a refusal still holds nothing of the call.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — three state readings inside the ratified carriers; the closed-field rule
**Section:** §Input Validation → Driver session socket row · CLI arguments (escher-session) row · Driver command schema row · §Threat Model Summary → local IPC, the trust boundary · §Error Handling → Error format (the command refusal item)
**Change:**
- Socket row: an answer's two carriers, the snapshot's text and the diff, now also hold three state readings for an element that states one — up to five more bare words of a closed set in the text (`selected` / `unselected`, `pressed` / `unpressed`, `invalid`), up to three more boolean keys in a diff node (`selected` and `pressed` only where present, `invalid` always; twelve node keys) — never an attribute's value as typed. No carrier, verb, argument kind, refusal cause, log field, socket or port added; the wire stays `v2`. Recorded as an inventory fact of the ratified crossing, not a further widening.
- The row also states the rule set with the ruling: a new field of a closed value set inside an already ratified carrier is not a widening and does not come to the founder; a field holding authored text, an open value set, a new carrier or a new exit still does.
- Threat Model and the CLI row: the same content list restated with the three readings.
- Driver command schema row: `Outcome`'s `Debug` also prints each diff node's three state readings; the crate logs and fields none, and the command span keeps its eight fields.
- Error format: was "the help of `changed` and of `snapshot`'s `text` read as they did before 2026-10-10-driver-cli and state no limit"; now both are restated, name the state words and say the two limits of the boolean model, each still a fixed literal holding nothing a call supplies.
**Why:** the founder's answer to the first boundary fork of this chunk (2026-10-10, by question dialog in the overseer session, relayed by the overseer; confirmed by him at this wrap's card, relayed the same way): the three readings are inside the ratified crossing. The plan had recorded them PROVISIONAL on the operator's reading; his word supersedes it and no PROVISIONAL mark was written. The rule is also in the playbook, appended at this wrap.
**Kept:** MCP stays its own crossing question, asked at its entry.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-snapshot-states — a Markup attributes row for the three control states; the adapter crossing's third class
**Section:** §Input Validation → Markup attributes, a new row `aria-selected` · `aria-pressed` · `aria-invalid` (control states) · the `id` (stable element id) row
**Change:**
- New row: the in-process builder reads the three attributes from parsed content, the browser app's remote HTML included, on every element whatever its role. No new input class enters. Each value is trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set: `aria-selected` `true` / `false`; `aria-pressed` `true` · `false` · `mixed`; `aria-invalid` `true` · `grammar` · `spelling`, an absent, empty or `false` value nothing and any other value `Invalid::True`. `TRUE` reads unrecognized — the lower-case match is a choice. `aria-checked` is not read. No log site, no panic path: a value of 5,500 characters builds the tree, and neither an unrecognized word nor the long value is written anywhere as typed.
- The row names two exits, both closed enums or booleans: the platform accessibility adapter, and the snapshot, which takes the three from the tree node — `NodeState.selected`, `.pressed` (only `Toggled::True` reads `Some(true)`) and `.invalid` — and crosses the session socket and the client's stdout inside the ratified carriers. The password and file mask is untouched.
- `id` row: the adapter crossing carried two content classes (ids as `author_id`, accessible names); it now carries a third, control states — `selected`, `toggled`, `invalid`. Its windowed witness is owed like the refresh's: no check reads the three at the adapter on the dev host.
**Why:** ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Ратифицировать»; confirmed by him at this wrap's card, relayed verbatim the same way as his chosen option label «Подтвердить всё»; the English wording is the overseer's): the builder writes the three states on the node — a widening of the adapter crossing by a new author-derived class. The plan had recorded it PROVISIONAL on the operator's reading; his word supersedes it and no PROVISIONAL mark was written.
**Kept:** what an assistive technology reads stays unmeasured until a windowed check; the witness is owned by a route entry.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — the MCP server's input and its crossing, ratified
**Section:** §Threat Model Summary → Attack surface · §Authentication & Authorization (table) · §Input Validation (a new MCP server row · Driver command schema · CLI arguments (escher-session) · the `id`, accessible-names, control-states and password / file rows) · §API Security · §Error Handling → Error format
**Change:**
- A new vector, "local stdio — the driver's MCP server": `serve_mcp` run by `escher-session mcp` as a child an MCP client spawns, the trust boundary the process holding its stdio; its results reached the model provider through the client in the one live trial. A new auth row: no handshake, token or peer check, no file modes — it binds nothing.
- A new Input Validation row: a request line ≤ 65,536 bytes (longer read, dropped, `-32700`), non-JSON `-32700`, a bad envelope `-32600`, the `id` echoed only as an i64 or a string ≤ 128 bytes (else `-32600`, `id: null`), fixed protocol errors with no `data`, four methods answered (else `-32601`), notifications run nothing, a tool argument of the wrong kind refused `malformed` before anything runs, then `validate`; keys in key order, so `Repeated` cannot arise over MCP; a result ≤ 1,048,576 bytes; replies hand-written, `serde_json` reading only.
- The schema row: was "two external surfaces … no MCP tool does yet", now three — the MCP `tools/call` the third; outcomes written in two places; two clocks and two writers; a second span.
- The `id` row: was "two exits … still no MCP command" and "no MCP command exists", now three exits; names, control states and masked values each gain the MCP results as an exit (the mask holds by construction; a password typed over MCP not separately run).
- CLI row: `mcp` alone the role, `mcp x` usage; the role's end; `FnMut` boot. API Security: still one listener. Error format: refusals and session errors as `isError` results.
**Why:** a boundary widening — the MCP crossing with its three bounds and its `id` echo — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Ратифицировать как есть»; his confirm at this wrap's Validate, relayed verbatim by the overseer as his chosen option label «Подтвердить всё», as the operator's answer; the English wording is the overseer's). Recorded ratified, no PROVISIONAL mark; the next carrier is a new question.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-10-mcp-surface — `serde_json` reviewed; the `tool_call` span; the live trial's login
**Section:** §Dependency Security → Pinning · §Logging & Monitoring (escher's sink · Stdout output) · §Secret Management → Storage (Development) · What counts as secret
**Change:**
- Pinning: `serde_json = "1"` (workspace, no `[features]`, so no `preserve_order`), the reader of the MCP server's untrusted stdin, writing nothing — reviewed by hand: no package enters or moves, one lock line, already in both graphs, no adapter, audit green, no ignore.
- The sink prints a second `escher_driver` span, `tool_call` (`tool`, `ending`, neither in `CONTENT_FIELDS`), the parent of `command`; the `mcp` role reads 9 lines built alone and 10 with the workspace over one CRUD flow at `info`, 0 typed sentinels, 0 ids, 0 names at `trace` under both builds. Stdout: the role writes protocol lines only, no ESC byte, the admitted `id` its one echo.
- Development: was "the cold-agent live run only", now also the 2026-10-10-mcp-surface live trial — the same login, no new secret source; What counts as secret names both.
**Why:** `serde_json` as a reader only is the founder's answer (2026-10-10, relayed verbatim by the overseer as his chosen option label «serde_json, только чтение»; the English wording is the overseer's). The span is a boundary widening (a new log field) ratified by him («Span с tool и ending», relayed verbatim the same way) with the same confirm at this wrap's Validate («Подтвердить всё»). The trial's use of the login rests on his word for the one live trial, recorded at this wrap on his answer relayed verbatim by the overseer as his chosen option label «Записать по форку 3», as the operator's.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — `help` and the program's fixed text through the validated surfaces, ratified
**Section:** §Input Validation → Driver command schema · CLI arguments (escher-session) · MCP server · Driver session socket · Driver session state directory · Markup attributes (password and file `input` value) · §API Security (opening; Settle and loads in flight)
**Change:**
- Driver command schema: was a closed table of nine verbs in two levels, now ten in three. `help` sits at the tool level: no argument, seven fixed-table result fields. `validate` refuses it `unknown-verb`, so its span records `help`. `validate_session` returns `SessionCommand::Help` and refuses any argument `malformed` (`Fault::Unnamed`). The MCP server answers it beside `start` · `status` · `stop`. `Outcome`'s `Debug` also prints `differs`, which no log fields. Still eight arguments.
- CLI arguments: the `help` ending — at most one `--session`, not read; status 0, no session reached; `help x` `malformed`, 1; `--session` empty or twice, usage, 2; nothing typed written back. The usage line lists `help`; the statuses are read from `EXIT_STATUSES`; the session error's object carries `remedy`.
- MCP server: `initialize` matched against a closed table of four revisions, any other answered `2025-11-25`. `tools/list` holds ten tools, each description the verb's help plus its result fields — fixed text, 2,984 → 8,844 bytes. `help` is answered by the server, ending accepted, with no `command` child. Still four methods; `server/discover` still `-32601`.
- Socket: was twelve node keys, now thirteen — `differs`, closed node-key words on a changed node, inside the ratified carrier by the rule of 2026-10-10.
- State directory: `StateDirTooLong` states its bound with no path — 94 bytes on Linux (measured), 90 elsewhere (read) — and its remedy.
- Password row: was "a password typed over MCP not separately run"; now measured, `stand_mcp_masked` — 0 occurrences, the mask, both modes. A file input over MCP is still not run.
- API Security: the role serves ten verbs, `help` running nothing on a session; the tool level waits on nothing.
**Why:** a boundary widening — the validated surface admits `help`, and the program's own fixed `'static` table text crosses out on the ratified carriers — ratified by the founder (2026-10-12, «Внутри, с правилом» and «help — десятый глагол», by question dialog in the overseer session, relayed verbatim by the overseer; his confirm of the wrap's answer card, «Подтвердить всё», the same channel), no PROVISIONAL mark. Text from a page, a call or a screen, a new carrier or exit still widens — now a playbook rule. The revision table is the operator's answer at the plan forks.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-self-description — a session error's remedy and its table, ratified
**Section:** §Error Handling → Error format (the escher-driver refusal bullet)
**Change:** was "a `SessionError` is written as `{"error":{"kind","message"}}`"; now `{"error":{"kind","message","remedy"}}`. Each error's kind, meaning and remedy are one row of the crate-private `SESSION_ERRORS: [ErrorSpec; 13]`. `kind()`, the new public `remedy()` and `Display` read it: `Display` is the row's meaning, with `Io` and `NotSettled` adding their detail as before. `dead` shares `no-session`'s remedy and `answer-too-large` shares `not-settled`'s, each later row naming the earlier. `StateDirTooLong` states its bound as a figure (94 / 90 bytes) and holds no path. Every text is fixed table text, served by `help` and carried on stdout and in MCP results. The node keys' help now also states the boolean model's two limits.
**Why:** a boundary widening (fixed program text crossing out) ratified by the founder (2026-10-12, «Внутри, с правилом», by question dialog in the overseer session, relayed verbatim by the overseer; confirmed at the wrap's answer card, «Подтвердить всё», the same channel). The operator's answer for a remedy each and the bound was conditional on that answer.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — the screen's pixels in band on the three ratified carriers
**Section:** §Threat Model Summary → local IPC · local stdio · §Input Validation → Driver session socket · Driver command schema · CLI arguments (escher-session) · MCP server · §API Security (intro) · Settle and loads in flight
**Change:**
- Socket: an answer also takes the screen out — an accepted `screenshot`'s line `{"width","height","png"}`, the PNG as base64, a password in it painted masked — under the unchanged 1,048,576-byte bound (largest whole-screen answer 22,404 bytes, 2.1 %). Threat model: the IPC and stdio vectors' boundaries name the image; what a client does with it, a model provider included, is the client's.
- Schema: was ten verbs, six of the instance; now eleven, seven — `screenshot`, an optional `id`, result `width` · `height` · `png` (`FieldKind::Png`, a tenth kind). An `id` of a `screenshot` is looked up as a `click`'s is. Was all eight causes returned, six by the executor; now nine, seven — `fonts-unavailable` refuses a shot of an app with no font it can paint text with before anything is painted; a crop of no pixel is `off-screen`; a refused shot leaves the instance unchanged. `Outcome`'s `Debug` also prints a shot's PNG bytes; never fielded gains the image, its size and its dimensions (a shot records `verb`, and `cause` when refused); the `image` encoder reads no clock and prints nothing.
- CLI: `screenshot [--id]` answers its PNG line on stdout, status 0; an unread `--id` exits 1 writing nothing typed; the usage line lists `screenshot`.
- MCP: was ten tools, now eleven; an accepted `screenshot` is an `image` block then a text block, a refused one one text block (`isError`); was every result bounded by its text, now a result holding an image is bounded by its whole written form.
- Settle row: seven instance verbs; a shot waits on nothing and moves no time; where overlay scrollbars are painted a thumb within 0.7 s of a scroll fades by wall clock, so such a shot can differ — owned by a CARRY on "Stand requirement sweep".
**Why:** pixels are no closed value set, so the in-band image is a boundary widening — the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13). No new carrier, exit, path argument or bound.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-12-headless-screenshot — the engine's password mask, nine causes, no image in a log, `png`
**Section:** §Input Validation → Markup attributes, password and file `input` value · §Error Handling → Error format (escher-driver's refusal) · §Logging & Monitoring → Log format and backends (escher's own sink) · §Dependency Security → Pinning
**Change:**
- Password row: was "password- and file-only, and snapshot-only"; now the password's paint is masked too — blitz-paint paints a `password` input's value as bullets laid out by `BaseDocument::layout_as_text_input`, in every document and renderer through blitz-paint, windowed included. Measured: two values of one length paint byte-identical frames, focused or not, both layout modes; a password typed through the driver never leaves as pixels in a shot. The frame still depends on the value's length, and a click inside a filled field places the caret by the real glyph widths — not measured, owned by a CARRY on "Stand requirement sweep". A file input's paint is unchanged.
- Error format: was the cause set closed at eight; now nine, `fonts-unavailable` added, its texts fixed and holding no path; every cause returned by code, two by `validate` and seven by the executor; a refused shot over MCP is one text block.
- Logging: the table holds seven instance verbs and nine causes; `stand_act_spans` also reads a refused shot's `cause=fonts-unavailable`; a shot's image reaches no log — the first 64 base64 characters of a shot occur 0 times in the host's and the `mcp` role's stderr at `trace`, under both builds.
- Pinning: `png` ("0.18", 0.18.1) is taken by escher-driver and blitz-tests (dev); blitz-test-harness takes `blitz-paint` (default features off), `anyrender` and `anyrender_vello_cpu` — no package entered `Cargo.lock`, the audit green with no ignore, no `base64` crate.
**Why:** the mask narrows what leaves, so it is no widening; as an engine change for every document it is the founder's fork 2, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer) — the class the playbook's new engine-change rule now covers.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the 28-name falsy predicate; no git dependency; the sync's dependency delta
**Section:** §Input Validation → Dioxus mutations, Boolean attributes with a falsy value · §Dependency Security → Pinning
**Change:**
- Dioxus mutations row: was one of the 27 names of the bridge's `BOOLEAN_ATTRIBUTES` list (the `isBoolAttr` list of Dioxus 0.7.10's web interpreter); now one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set, `inert` added. `BOOLEAN_ATTRIBUTES` retired; `mutation_writer.rs` is byte-identical to upstream's at `434e0661`. The row stays an in-process mutation path: no new input class and no crossing.
- Pinning: was "the one git dependency, taffy, is pinned by commit rev"; now no git dependency is left in the root manifest — taffy is a crates.io registry version, "0.15" resolved 0.15.0, with a checksum in `Cargo.lock`.
- Pinning gains the 2026-10-13 sync's delta (`434e0661`, merge `a6c24d24`):
  - taffy moves from git rev `7d33901c` (0.14.0) to crates.io 0.15.0, its eight features byte for byte;
  - the workspace version and the nine in-repo `=` pins move to `0.3.0-beta.3`, escher-driver and escher-telemetry following by `cargo update --workspace`;
  - blitz-tests gains the dev-dependency `cursor-icon`, already in the lock;
  - no other third-party package enters, leaves or moves — the lock's set equals upstream's with the two `escher-*` entries aside;
  - the audit leg reads `advisories ok`, with `deny.toml` unchanged.
**Why:** a closed set taken from upstream and a dependency source moved to the registry with a checksum: no boundary widened and no package added. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the founder's word respelled to the verbatim form; the `@font-face` row's bare pipe
**Section:** §Threat Model Summary (the session socket; the MCP stdio crossing) · §Input Validation (the socket's answer, the command schema, the CLI, the MCP server, the `@font-face` source, the `id` and accessible-name rows, the three aria states) · §Dependency Security → Pinning (`serde_json`) · §Secret Management (the live trial) · §Logging & Monitoring (the `tool_call` span)
**Change:**
- Eighteen sites whose «relayed by the overseer» copies one of the six respelled entries now read "relayed verbatim by the overseer as his chosen option label(s) …; the English wording is the overseer's", naming the entry's labels:
  - «Сокет несёт команды»;
  - «Бинарник приложения, путь» · «Именованные флаги» · «Четыре класса»;
  - «Утвердить + проверка» — the `@font-face` source row;
  - «Ратифицировать»;
  - «Ратифицировать как есть»;
  - «serde_json, только чтение», «Span с tool и ending» and «Записать по форку 3».
- Three short spellings stay: the inside-the-crossing ruling ×2, whose entry kept the short form, and the orphaned-host answer, which names no label.
- `@font-face` source row: the searched span `font-face|font_face` is now written `font-face` or `font_face`, so no bare pipe splits the table row, which counts the same as its neighbours.
**Why:** the body follows the entry it copies; no claim, ratification or PROVISIONAL mark moved. The lists were confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить списки (Рекомендую)»; the English wording is the overseer's), with playbook rule A added on the same answer.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-stand-contrast-harness — blitz-test-harness takes `color`, reviewed
**Section:** §Dependency Security (Pinning)
**Change:** a Pinning bullet: since this chunk blitz-test-harness takes `color = { workspace = true }` (the workspace's "0.3", resolved 0.3.3, already in the graph through blitz-dom), the luminance source of its contrast reader — reviewed at the chunk: no package enters `Cargo.lock` (its one changed line `"color",` inside the harness's dependency list), the audit leg reads `advisories ok` with `deny.toml` unchanged, the harness takes no `tracing` and no `style`, and blitz-tests gains no dependency; no ban list exists.
**Why:** every dependency an escher crate takes is recorded as a reviewed Pinning bullet; no boundary moved — the crate was already in both graphs.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-13-range-input-interaction — the range input row, a boundary widening ratified
**Section:** §Input Validation (a new `Markup attributes` row for `input type="range"`; the `disabled` row)
**Change:**
- New row. In every document, the browser app's remote pages included, a range input holds a current value apart from its `value` attribute.
  - `min`, `max`, `step` and `value` are read through `parse_float`, the HTML floating-point rules: a parse error, NaN or an infinity reads absent; `-0` reads `0`.
  - Defaults: `min` 0, `max` 100, `step` 1. A `max` below `min` reads `min`; a step ≤ 0 or unparsable reads 1; `any` sets no grid. The base is `min`, else `value`, else 0; the default is the overflow-free midpoint.
  - Every seed, write and move is sanitized: clamped, put on the grid with an overflow-free index, rounded to the step's and base's decimals and written shortest round-trip. The value stays finite, bounded and on the grid on degenerate markup (8 unit rows, 7 integration rows).
  - The keys (arrows, `Home`, `End`), a main-button pointerdown by position and a focusing click act only with no `disabled` attribute.
  - One `input` per change. A `value` write re-seeds and a `min`/`max`/`step` write re-sanitizes.
  - No print, log site, `unwrap`, `expect`, panic or env read.
  - The value leaves only as the form entry, the `input` event's value and the snapshot's `value`, a control value inside the ratified carriers. No carrier, verb, field, log field, span or accessibility value or action is added.
- `disabled` row: presence now also gates the range input's key, pointerdown and click arms.
**Why:** a boundary widening, a validated surface admitting a new input class from any document's markup. It was ratified by the founder, never routine, by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label:
- his ruling of 2026-10-07 «Пусть будет а»;
- his three fork answers of 2026-10-13 «Отдельное текущее значение», «Стрелки + Home/End (Рекомендую)» and «Клик даёт фокус (Рекомендую)»;
- his confirm of this wrap's answer card «Подтвердить (Рекомендую)», which also covers the three consequences the operator confirmed at the review: submission sends the current value, a `value` write resets it, and the pointerdown moves the value while the click focuses.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-stand-keyboard-harness — keyboard activation, a boundary widening ratified, and `disabled` by presence
**Section:** §Input Validation → Markup attributes: keyboard activation (new row) · `disabled` · `attr_parsed`
**Change:**
- New row, keyboard activation (Enter keydown, Space keyup) and a button click's focus, in every document, the browser app's remote pages included:
  - A pressed `Enter` on the focused node, after the text-input and range arms, dispatches a Click on a `button`, a `summary`, an `input` of type `submit`/`reset`/`button` or an `a` with `href`.
  - A released Space dispatches the same Click through the new `handle_key_up` (KeyUp's first default action) on a `button`, a `summary` or an `input` of type `submit`/`reset`/`button`/`checkbox`/`radio`. A link takes no Space; a text or range input takes neither key there.
  - The Click runs the app's handlers, then `handle_click`, so Enter and Space reach form submission, link navigation and `summary` toggling.
  - A click on a button focuses it before a submit button submits its form owner.
  - The gates are a closed element-name and type match and the `disabled` presence check. No print, log site, env read, `unwrap`, `expect` or panic in the added engine lines. No carrier, verb, key name, field, log field or span added.
- `disabled` row: focusability reads presence. Any `disabled`, bare, `"false"` or `"true"`, on any element removes it, for the Tab walk, autofocus and `unkeyed_actionable`. Presence also gates the Enter and Space arms. The snapshot's `enabled` and focusability read the same presence. Was: "Parsed as a boolean value for focusability", and a `disabled="false"` and a bare `disabled` "both stay focusable". Known limit: a `<div disabled tabindex=0>` is not focusable, unlike in browsers, its narrowing owned by a CARRY on "Stand requirement sweep".
- `attr_parsed` row: "as for `tabindex` and `disabled`" is now "as for `tabindex`"; `disabled` no longer goes through it.
**Why:** a boundary widening, never routine: a key now reaches the click default actions of any document. The founder answered the forks himself by question dialog in the overseer session, each answer relayed verbatim by the overseer as his chosen option label: «Клавиши нажимают (Рекомендую)» (keyboard activation), «Клик даёт фокус (Рекомендую)» (a click focuses a button), «По наличию (Рекомендую)» (`disabled` by presence), 2026-10-13. He confirmed on this wrap's answer card, 2026-10-14, «Подтвердить (Рекомендую)», that the three rows lie inside those answers; the `disabled` row rode the card because presence gates the widening's arms.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-14-stand-a11y-assertions — the tree the shell hands the adapter proven headlessly; the AT witness re-homed
**Section:** §Input Validation → the `id` (stable element id) row · the `aria-selected` · `aria-pressed` · `aria-invalid` (control states) row
**Change:**
- `id` row: the tree the crossing is handed is asserted headlessly on the stand in both layout modes — `Document::accessibility_tree` through `&dyn blitz_dom::Document`, as `AccessibilityState::update_tree` calls it, equals the document's own tree node for node, every `author_id` its node's `element_id`, none empty, all digits or holding `0x`; the refresh's two halves proven there too. What the platform adapter hands an assistive technology stays unread — the stand builds no adapter — that witness owned by a `.andromeda/residuals.md` line (target: next).
- control-states row: the witness owned by that residuals line, the headless half proven; was "owned by a route entry".
**Why:** the check the operator's ratification of 2026-10-06 (the id leaving as `author_id`, tracked) asked of this route entry; the adapter staying unbuilt is the founder's F1 «Пока нет, перенести (Рекомендую)», by question dialog in the overseer session, relayed by the overseer as his chosen option label, placed as a residuals line by the operator's P5 word. No new crossing, carrier, field or input class — not a widening.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-requirement-sweep — the password click caret a known 0.1.0 limit; the off-Linux bound measured; `covered`'s second limit
**Section:** §Input Validation → password and file `input` value row · `disabled` row · Driver session state directory row; §API Security → settle and loads row; §Error Handling → escher-driver refusal bullet; §Logging & Monitoring
**Change:**
- password row: a click-placed caret depends on the real characters' widths — index 5 after eight narrow characters, 2 after eight wide — so where it paints in a shot says something of their relative widths: a known 0.1.0 limit; the snapshot, the diff and the bullets stay masked and nothing new crosses; its fix owned by the C11 `.andromeda/residuals.md` line (target: next). Was "not measured by any check", a CARRY on "Stand requirement sweep".
- `disabled` row, settle row: the `disabled` narrowing and the overlay fade's move onto the harness clock are each owned by a residuals line; were CARRYs on that entry.
- state directory row: the 90 off Linux is measured on the macOS leg — 90 holds a session, 91 ends `state-dir-too-long`, status 3, no path in the message; was "read, not measured". The refusal bullet's restatement marks both bounds measured.
- refusal bullet: `covered`'s meaning states two measured limits, the one-axis clip added; remedy, the nine causes and their names unchanged.
- Logging: the workspace-built host re-read at 6 lines / 978 B (the `focussed node` line twice; was 5 / 860 B), the `mcp` role at 11 lines (was 10); built alone unchanged; needles 0.
**Why:** measured at this chunk. Stating C11 as a known 0.1.0 limit is the overseer's directive relayed by the operator at this wrap, on the founder's C11 «Замерить сейчас, исправить потом (Рекомендую)»; the owners rest on his C10 «Отложить на следующую версию (Рекомендую)» and C15 «Оставить по наличию, отложить (Рекомендую)» — each by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels — placed by the operator's word on the P5 card. `covered`'s text is the program's own fixed text inside the ratified carriers; no carrier, field or input class added — not a widening.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-quality-gates — the whole-workspace audit, the licence gate, the owed checks discharged
**Section:** §Dependency Security → Audit tool, Supply chain integrity (Lockfile verification, Licence compliance) · §Input Validation (apt-install.sh · coverage_floor.py, new row · Driver session socket · escher-session CLI · Fetched fonts · `@font-face` source) · §Secret Management → Environment values read · §Logging & Monitoring
**Change:**
- Audit: `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph; the leg reads the `[licenses]` table (was "does not read"). Two ignores: RUSTSEC-2026-0192 and RUSTSEC-2024-0436 (paste 1.0.15, a proc-macro of `rav1e` ← `ravif` ← `image`).
- The pruning reach claim is history of the old invocation. On the new graph paste reaches the gate; memmap2 RUSTSEC-2026-0186 is kept out by the `unsound` default, not pruning — a measured limit. Each is owned by a `.andromeda/residuals.md` line.
- Licence compliance: the fork gates on licences — was "an open question, owned by the route entry Quality gates". Lockfile verification lists the `stand-repeat` leg and both clippy lines.
- Input Validation: `apt-install.sh` takes ten calls with its bounded repair, the usage gate still exiting 2 before `dpkg`. New row `coverage_floor.py`: `CRATE_DIR=PERCENT` ≤ 100, no duplicate, else exit 2; prints no source path.
- The 30 s answer bound and the closed pipe are each pinned by a check. The `@font-face` hint-less source is covered by `font_face_sniffed_source`, which also witnesses the sniff — was "still owed".
- Environment values: `TMPDIR`, read by escher-driver's client unit test through `std::env::temp_dir()`; the library's code reads none beyond `RUST_LOG`.
- Logging: three stderr readings by build and level, 28 runs, 0 needles.
**Why:** the founder's «Аудит всего workspace (Рекомендую)» and «Проверка лицензий (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the owed check is the one his «Утвердить + проверка» (2026-10-10) asked for. The test-only `TMPDIR` read is the operator's answer at this wrap: no crossing.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/

## 2026-10-15-cold-agent-test — the cold-agent pipe on the escher server: the stand's screen to the provider, the agent's check as input
**Section:** §Input Validation (CLI arguments (cold-agent.sh) · Cold-agent check, new row in place of the Cold-agent MCP stub row) · §Threat Model Summary (local stdio — the driver MCP server) · §Secret Management → Storage → Development · §Logging & Monitoring
**Change:**
- CLI row: the task allowlist is `stand` (was `counter`; `run counter` is now a usage error), usage still exits 2 before any precondition; a third precondition, an executable `target/debug/escher-session` (exit 3, the pipe builds nothing); the fixed prompt `STAND_PROMPT` names no element id and no app slug.
- The stub row is retired with `scripts/cold_agent_stub.py`. New row, the agent-written check: the reply's last fenced json block, ≤ 65,536 bytes, a JSON array, ≤ 400 steps, closed keys, `tool` one of the eleven verbs; outside these `no-check` / `check-failed`, never replayed; a valid check replayed in full and ablated on two fresh `escher-session mcp` children, each step validated again by the server, 30 s per reply, killed and reaped; `check.json` never printed.
- Threat Model: a second carriage — the cold-agent pipe's one live run carried the stand's screen (ids, names, values in snapshot texts and diffs, 43 tool calls, no `screenshot`) to the model provider, and the reply returns as the check.
- Storage → Development: the run drives `mcp__escher` with `--model claude-opus-5-5`, `--max-budget-usd 3`, `claude` 2.1.296, through the operator's own login — no new secret source; the env census is over `cold-agent.sh` alone (was "the pipe or its stub … the three scripts"); the evidence's one credential-scan hit is `apiKeySource: none`.
- Logging: the stub's call log is retired; `server.log` (escher's sink at `RUST_LOG=info`, counted by span name) and `check.json` are never printed; four planted markers (tool argument, tool answer, reply, check).
**Why:** a boundary widening, ratified by the founder: his «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)» (2026-10-15, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels), confirmed by his «Подтвердить (Рекомендую)» at this wrap's answer card, relayed verbatim the same way. The CLI allowlist follows his «Все четыре в одной сессии».
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/

## 2026-10-16-handler-splits-and-readme — the range and keyboard-activation rows cite the named functions
**Section:** §Input Validation (the `input type="range"` row · the keyboard-activation row)
**Change:**
- The range row: `parse_float` reads the four attributes through its private steps `take_sign`, `take_digits` and `take_exponent`, the readings unchanged; the keys cite the range arm's gate beside the key mapping (`press_range_input_key`); the pointerdown cites `press_range_input`.
- The keyboard-activation row: the button arm's focus and submit sit in the private `click_button`.
- No gate, input class, reading, carrier or log site moved; the added lines carry no `unwrap`, `expect`, panic, print, `tracing` site or env read, so §Error Handling's panic-path list gains nothing.
**Why:** the code audit's complexity split moved those bodies into private functions with no behaviour changed; the rows' claims stand and cite where the code now is.
**Ref:** .andromeda/runs/2026-10-16T14-54-43-wrap/

## 2026-10-16-fast-failing-mutants — five advisory ignores: zip 2.4.2's three (§Dependency Security)
**Section:** §Dependency Security → Audit tool
**Change:** was «Two ignores stand»; now five — RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (zip 2.4.2, each fixed in zip 9.0) ignored by ID with a written reason: zip reaches the whole-workspace graph only through `anyrender_serialize 0.8.0` ← `browser` (apps/browser's capture feature), which reads archive entries `by_name` into memory and never extracts to disk; no fixed release is reachable (anyrender_serialize 0.8.0 is the latest and requires zip ^2.1); dropped when that chain reaches zip 9.0 or leaves zip, owned by a `.andromeda/residuals.md` line (target: next), and re-read by the audit leg on every push. No dependency added or bumped.
**Why:** external decay — the three advisories turned the Dependency audit red over a `Cargo.lock` and `deny.toml` byte-identical to the base's green run; the ID-scoped ignore with a named owner is the founder's decision (the founder, 2026-10-17, by question dialog, relayed verbatim by the operator), and the count's amendment the operator's directive at this wrap's resume (the operator, 2026-10-17). An advisory ignore by ID with a written reason is the audit policy this section already states, not a boundary widening.
**Ref:** .andromeda/runs/2026-10-17T07-59-25-wrap/

## 2026-10-17-upstream-sync-engine-debts — the merged ARIA reads: `aria-labelledby` names, descriptions to the adapter, three closed values
**Section:** §Input Validation (the `aria-label` · `<label for>` row · a new `aria-describedby` · `aria-description` row · a new `aria-expanded` · `aria-disabled` · `aria-level` row · the `id` row)
**Change:**
- Names row: gains upstream's `aria-labelledby` read — referenced ids resolved to built nodes only, an unknown id ignored, the text gathered cycle-guarded, the computed `label` outranking a `<label>` and an `aria-label` (the fallback when the targets hold no text); upstream's unfiltered `aria-label` read dropped, one trimmed non-empty read left; the text stays inside the ratified name crossing.
- New descriptions row: `aria-describedby`'s targets as `described_by`, `aria-description`'s own free, unbounded text into AccessKit's description field; no log site, print or panic path; the one exit is the platform adapter — the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries it; escher routes no agent- or user-supplied input through it.
- New closed-values row: `aria-expanded` (a boolean), `aria-disabled` (`true` → disabled), `aria-level` (a number, tag and role defaults), each carrying none of the attribute's text, inside the states class; `aria-disabled` moves no snapshot reading.
- `id` row: the adapter crossing gains a fourth content class, descriptions, and its names include `aria-labelledby`'s text.
**Why:** a boundary widening by upstream merge, ratified as upstream's behaviour by the founder's own word at P4 fork 2, by question dialog in the overseer session, relayed by the overseer, and both description sources confirmed by his word on this wrap's answer card, relayed by the overseer; the closed values are a field inside a ratified carrier.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/

## 2026-10-17-upstream-sync-engine-debts — the bump CLI's rows leave; no physical key code hits `todo!()`
**Section:** §Input Validation (the `CLI arguments (bump)` row) · §Error Handling (External responses · the panic paths)
**Change:**
- The `CLI arguments (bump)` row and §Error Handling's «bump's `bail!` macro» clause are gone with the crate.
- The panic paths: was «KeyboardBacklightToggle and any unlisted physical key code hit todo!()»; now the shell passes winit's physical key `Code` through, so no physical key code hits a `todo!()`.
**Why:** upstream's `23b9b26a9` "Delete the bump crate" and its keyboard-types 0.8 converters, taken by the sync.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/

## 2026-10-17-upstream-sync-engine-debts — eight advisory ignores: the zbus family's three; the 2026-10-17 sync record (§Dependency Security)
**Section:** §Dependency Security (Audit tool · Pinning)
**Change:**
- Audit tool: was «Five ignores stand»; now eight — RUSTSEC-2026-0370 (zvariant 5.15.0), RUSTSEC-2026-0371 (zbus_names 4.3.4) and RUSTSEC-2026-0372 (zvariant_derive 5.15.0), each unmaintained and folded into zbus 6.0.0, reached only through zbus 5.19 + atspi 0.29 under accesskit_unix 0.24.0 (the latest, requiring zbus ^5.19) ← accesskit_xplat ← blitz-shell, so no fix is reachable; each ignored by ID with that reason, external decay over an unchanged tree, dropped when accesskit_unix moves to zbus 6, owned by a `.andromeda/residuals.md` line, re-read by the audit leg on every push.
- Pinning: gains the 2026-10-17 sync record — upstream's set taken whole at `23b9b26a9`: keyboard-types 0.8 (Dioxus keeping its 0.7), test-that and test-that-macro 0.6.0 reviewed by hand, the workspace at 0.3.0-beta.4, the three bump-only toml crates leaving, Taffy still the registry "0.15" with no git dependency, the lock's set equal to upstream's with the two `escher-*` entries aside; the audit green with `deny.toml` unchanged at implement and, on the final HEAD, only with the three ignores.
**Why:** the operator's word under the playbook's External decay pattern, recorded as a widening in the chunk's scope record — the three advisories turned the audit red over a tree that had read green; the (security) criterion «with `deny.toml` unchanged» stays UNMET as written, its letter unchanged and nothing reverted, on the operator's resume directive.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
## 2026-10-18-engine-debts — §Input Validation: the `inert` read and the toggle per role, PROVISIONAL
**Section:** §Input Validation (Markup attributes: a new `inert` row · the control-states row, renamed `aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid` (control states), `role="switch"` · the `id` row · the merged closed-values row)
**Change:**
- A new `inert` row: read by presence from parsed content, remote pages included, on the element or an ancestor (`Node::is_inert`) — an inert node not focusable (Tab walk, autofocus, `unkeyed_actionable`), no hit at it (`hit_inner`), its subtree out of the tree the shell hands the adapter and of the snapshot; a presence check only, no text carried out, no log site, env read or panic path, no carrier or field added; the focus-fixup limit stated, owned by a `.andromeda/residuals.md` line.
- Control states: was «on every element whatever its role … `aria-checked` is not read and no role is consulted»; now the toggle by role — `aria-pressed` off the five checkable roles, `aria-checked` (`true` · `false` · `mixed`, trimmed, exact) on `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` alone, `"switch"` → `Role::Switch`, a native checkbox or radio reading neither and its native checked state reaching no tree node (a measured absence, owned by a `.andromeda/residuals.md` line); the snapshot's `pressed` off those roles and `checked` from a native input or a checkable role's toggle; the readings still closed enums and booleans on the same two exits.
- The `id` row's states class names `toggled`'s two sources and the renamed row.
- Merged closed values: was «no test names any of the three»; now `accessibility_merged_states`' 3 tests pin them.
**Why:** F1 A and F2 A with the `switch` mapping and the native exclusion — PROVISIONAL, delegate operator; each a closed read carrying no text out, inside the states class already ratified, so no new crossing; standing for the founder's own word at the Epoch 2 Provisional discharge, which the operator directed the wrap not to wait for.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-engine-debts — §Input Validation: `disabled` on form controls, the password caret by the bullets; §Error Handling's refusal texts
**Section:** §Input Validation (Markup attributes: `disabled` · keyboard activation and a button click's focus · password and file `input` value) · §Error Handling (the command refusal paragraph)
**Change:**
- `disabled`: was «any `disabled` attribute … on any element removes focusability … the `<div disabled tabindex=0>` limit … C15»; now presence on a form control alone through `ElementData::is_disabled`, read by focusability, the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate and the activation arms; the `DISABLED` state and the range input's gates read presence as before; the presence reading's ratification kept.
- Keyboard activation: the gate is `is_disabled` (was «carries no `disabled` attribute (presence)»), so a `disabled` attribute on a `summary` or an `a` gates neither key, and a disabled ancestor stops no click walk.
- Password value: was «a click … places the caret by the real characters' widths … index 5 … 2 … a known 0.1.0 limit … C11»; now the caret by the bullets — `click_password_input` lays out one U+2022 per character through `layout_as_text_input`, scrolled as paint scrolls it, and moves the real editor by `Cursor::from_point` (a click, a shift-click, two or more clicks select all) — so no width of the real characters reaches the caret or a shot; nothing new crosses.
- Refusal paragraph: `covered`'s limits drop «a hit reaches content clipped by `contain: paint`» and gain the no-area sentence; `off-screen`'s gains the transformed-scrolling-box sentence; the `checked` / `pressed` node-key help restated; the cause set nine, every text a fixed `'static` string.
**Why:** F5 A, F6 A, F7 A, F8 A and F9 A — PROVISIONAL, delegate operator, as the first entry states; the refusal and help texts are the program's own fixed text inside the ratified carriers.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
