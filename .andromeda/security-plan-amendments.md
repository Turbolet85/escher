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
