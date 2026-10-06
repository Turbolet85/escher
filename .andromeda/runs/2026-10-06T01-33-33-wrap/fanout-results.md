# Fan-out results — 2026-10-06-telemetry-bootstrap

Report: `escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/report.md` · detectors 15 (arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = the drift-base's 15 `doc:` names) · keyed-contract renders: test-plan · obs-plan · a11y-plan (architecture NOT MIGRATED; security / design / layout n/a). Entity probe: no `&lt;`/`&gt;`/`&amp;` in any return (entities=0).

## Verdicts
- architecture — 12 proposals (D-arch-resources 7 · D-arch-decisions 5). Stripped: nothing.
- security-plan — 1 proposal (D-security-auth). Stripped: trailing comments — D-security-input no drift (no IPC/HTTP/socket; RUST_LOG parse-with-fallback); D-security-deps no drift (tracing-log resolves the locked 0.2.0, no registry crate, no ban list); pointer to §Bootstrap phases logging-redaction-wire (:241), §Logging & Monitoring (:341), §Data Protection redaction-absent claim (:161) as uncovered expected amendments.
- design-system — `proposals: []`. Stripped: commentary (every Coverage row `tokens n/a`; the 2 `407` hits are line numbers / colour values). No raw twin (the empty list itself was unaltered).
- layout-templates — `proposals: []`. Stripped: commentary (no UI surface; stderr line is diagnostic, owned by obs/arch/security). No raw twin.
- test-plan — 5 proposals (D-tests-obs-harness 1 · D-tests-coverage 4). Stripped: trailing comments — D-tests-coverage invariant holds; D-tests-framework no drift; test-plan keyed contract untouched.
- obs-plan — 10 proposals (D-obs-stack 8 · D-obs-pii 2). Stripped: trailing notes — D-obs-instrumentation no violation (bootstrap / one-shot paths); slice-scoped "observed absent" search records deliberately left (§2:49-52, §3:69-71, §6:203, §7:241, §8:263-271); §9:285 CI-log claim unchanged.
- a11y-plan — 1 proposal (D-a11y-obs-schema). Stripped: nothing (D-a11y-surface no drift recorded inside the rationale).

## Parsed proposals + dispositions

### architecture
- A1 · D-arch-resources · warning · §Occupied Resources → Names — add crate `escher-telemetry` (lib, publish = false, workspace member + `[workspace.dependencies]` path entry). basis architecture.md:150. → **apply** (check 1: playbook "Accurate this-chunk addition"; expected amendment 1 names it).
- A2 · D-arch-resources · dependent-of D-arch-resources · §Infrastructure Patterns → Directory structure — add `escher-telemetry` to the `packages/` tree rows. basis :161-163. → **apply** (same rule; the tree lists every packages/ crate).
- A3 · D-arch-resources · dependent · §Existing Scopes — add row `escher-telemetry | packages/escher-telemetry | modules lib, format, panic`. basis :244-247. → **apply** (same rule).
- A4 · D-arch-resources · dependent · Inherited Defaults → Publishability — add escher-telemetry to the `publish = false` crates; exclude it from "packages/ libs released by bump". basis :216. → **apply** (same rule).
- A5 · D-arch-resources · warning · §Occupied Resources → Environment variables — add `RUST_LOG` (escher_telemetry::init's EnvFilter, default warn; also the upstream fmt::init installs). basis :149. → **apply** (rule "Accurate this-chunk addition"; expected amendment 2).
- A6 · D-arch-resources · warning · §Occupied Resources → Process-wide state and threads — the global subscriber, LogTracer bridge, chaining panic hook, `OnceLock`/`Mutex` statics, log targets `escher_telemetry` / `escher_telemetry::panic`; no thread. basis :146. → **apply** (expected amendment 3).
- A7 · D-arch-resources · warning · §Standard Contracts → Telemetry (escher-telemetry) — public API, stderr line shape, allowlist scrub, panic event, sole caller. basis :118-138. → **apply, condensed** (check 1: "Accurate this-chunk addition" — a contract SHAPE (API + line format) of a new crate, the same grain as the harness / provider entries the section carries; "Registry over-reach" read and NOT matched: the section enumerates per-crate public contracts).
- A8 · D-arch-decisions · warning · §Stack and Technologies → Parallelism and misc — add `tracing-log "0.2"`; escher-telemetry's tracing-subscriber features. basis :59. → **apply** (Dependencies bullet).
- A9 · D-arch-decisions · dependent · §Stack and Technologies → Testing — blitz-tests dev-deps + escher-telemetry, tracing, tracing-log. basis :64. → **apply**.
- A10 · D-arch-decisions · dependent · §Cross-cutting Patterns → Logging and timing — escher binaries install `escher_telemetry::init` (today `seven_guis_native`, native, ungated): stderr, identity, log bridge, chaining panic hook, allowlist scrub; upstream fmt::init installs stay tracing-gated, stdout, unscrubbed. basis :187. → **apply** (expected amendment 4).
- A11 · D-arch-decisions · dependent · §Conventions → Feature gating — "tracing gated per call site" scoped to engine / upstream crates; escher-telemetry ungated. basis :110. → **apply** (check 1 "Accurate this-chunk addition" + check 3: plan Constraints frame the gate as the engine call sites' rule — "do not remove any engine call site's #[cfg(feature = "tracing")] gate" — and plan steps 3-4 specify the ungated startup/panic events; scope.md: "Engine crates keep the invariant"; not a reversal — the engine rule stands).
- A12 · D-arch-decisions · dependent · Inherited Defaults → Optional capabilities — same qualification. basis :218. → **apply** (as A11).

### security-plan
- S1 · D-security-auth · warning · Secret Management → Environment values read — add `RUST_LOG` … `(packages/escher-telemetry/src/lib.rs:113)`. basis :270. → **reject** (Validate pre-check: the `change` cites a source location the report does not carry — the re-derivation tell). Its fact enters through the orchestrator: R-S1.
- R-S1 · orchestrator raise (check 5, expected amendment 8) · Secret Management → Environment values read — `RUST_LOG`, non-secret, read by escher-telemetry's EnvFilter (default warn). → **apply** (report Symbols / APIs carries it; rule "Accurate this-chunk addition").
- R-S2 · orchestrator raise (check 5, expected amendment 8) · Bootstrap phases → logging-redaction-wire — discharged for escher's own sink (the allowlist scrub in seven_guis_native); upstream fmt::init / env_logger sinks still unscrubbed. → **apply** (report Schema / config + Spec claims disproved (2)).
- R-S3 · orchestrator raise (check 5, expected amendment 8) · Logging & Monitoring — the escher-telemetry stderr sink, its allowlist scrub and reach; the examples "println!/eprintln! only" bullet qualified for seven_guis_native. → **apply**.
- R-S4 · orchestrator raise · dependent-of R-S2 · Data Protection → At rest, "redaction … observed absent in s05" — qualified: the s05 search stands; escher's own sink now scrubs (see Logging & Monitoring). → **apply** (the retired claim's restatement, security agent's pointer :161).

### test-plan
- T1 · D-tests-obs-harness · warning · §3 Test Harness Contract — record the stand's stderr line shape (escher-telemetry, obs-plan §3) and its exercising tests/smoke; drop "log format" from the NOT YET MEASURED list. basis :97. → **apply** (bind test §3 ↔ obs §3, applied with O1/O2).
- T2 · D-tests-coverage · warning · §9 → Local baseline — workspace 416 · 0 · 4, 114 result lines, basis the implement `fast` log; keep 2026-10-05 timings and 407 · 0 · 3 as that baseline's count. basis :282. → **apply** (expected amendment 9).
- T3 · D-tests-coverage · dependent · §9 → Local baseline `cargo test -p blitz-tests` (255 · 0 · 3) — mark as the 2026-10-05 reading, +4 / +1 ignored since, not re-measured per crate. basis :281. → **apply**.
- T4 · D-tests-coverage · dependent · §9 → coverage leg count (404 · 0 · 3) — mark as the 2026-10-05-ci-gate-legs reading, not re-run this chunk. basis :284. → **apply**.
- T5 · D-tests-coverage · dependent · §1 Test Scope Summary — add escher-telemetry (5 inline unit tests) and the four telemetry integration files to the blitz-tests coverage. basis :24. → **apply**.

### obs-plan
- O1 · D-obs-stack · warning · §3 Logging stack — add the escher-telemetry install (Registry + EnvFilter RUST_LOG/warn + non-ANSI EscherFormat fmt layer to stderr, LogTracer bridge, eprintln-and-continue on Err); scope the fmt::init bullet to the upstream apps; OTel SDK still absent. basis :66. → **apply** (expected amendment 5).
- O2 · D-obs-stack · dependent · §3 NOT YET MEASURED note — service identity + log line format now measured for the stand. basis :78. → **apply**.
- O3 · D-obs-stack · dependent · §6 Log format — the measured one-line stderr format, RUST_LOG/EnvFilter levels; no JSON / file sink / rotation. basis :134. → **apply**.
- O4 · D-obs-stack · dependent · §6 Logged events — escher-telemetry startup info + panic ERROR events; examples no longer println-only (seven_guis_native). basis :146. → **apply**.
- O5 · D-obs-stack · dependent · §1 Instrumentation scope — tracing-log workspace dep; escher-telemetry entity. basis :7. → **apply**.
- O6 · D-obs-stack · dependent · §1 examples bullet — not timing-prints-only. basis :10. → **apply** (merged with O5's examples clause — one edit).
- O7 · D-obs-stack · dependent · §2 Telemetry mechanism — feature-gated claim scoped to engine / upstream crates; escher-telemetry ungated. basis :29. → **apply** (as A11 — plan Constraints scope the gate to engine call sites).
- O8 · D-obs-stack · dependent · §7 Panic hooks — the chaining native hook. basis :209. → **apply** (expected amendment 6).
- O9 · D-obs-pii · escalate · §8 Scrubbing — the allowlist scrub, its reach (escher's sink only), residual as-is values (raw panic message via std hook; log.file host paths). basis :261. → **apply** (check 1: detector severity `escalate` is D-obs-pii's for RAW PII; this proposal records added redaction — no boundary widened (a sink gains a scrub; nothing new crosses); rule "Accurate this-chunk addition"; the P5-approved expected amendment 7 names this change itself — "the allowlist scrub (engine prefixes, safe set, content set) replaces 'observed absent' for escher's sinks" — settling it without a halt per check 1's recorded-direction clause).
- O10 · D-obs-pii · escalate · dependent · §3 → Bootstrap phases key — pii-scrubbing-wire discharged for escher's sink, open for upstream sinks; otel-sdk-install open, deferred at P4. basis registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:4. → **apply** (as O9; expected amendment 5 names it: "pii-scrubbing-wire discharged; otel-sdk-install still open, carried to 'Driver command spans'").

### a11y-plan
- Y1 · D-a11y-obs-schema · warning · §3 A11y Assertion Harness Contract — retire "obs §3/§6 leave the log format unmeasured"; state the escher sink's line shape + scrub; a11y violation schema stays unmeasured. basis a11y-plan.md:87. → **apply, re-derived** (rule "Accurate this-chunk addition"; the proposal's forward mandate "any future a11y violation JSON schema has to be written for this line shape" is NOT applied — no report fact carries it; the body states only the current obs format and keeps the a11y schema unmeasured).

## Validate checks
1. Playbook — 28 apply (all "Accurate this-chunk addition"; A7 read against "Registry over-reach", not matched); 1 reject (S1, re-derivation tell); no boundary widening (OTel deferred — nothing crossed); escalate-severity O9/O10 settled by the recorded P5 direction (expected amendments 5, 7).
2. Cross-contradiction — none: test §3 (T1) ↔ obs §3 (O1/O2) ↔ a11y §3 (Y1) state one line shape; arch A11/A12 ↔ obs O7 state one gating scope.
3. Intent-consistency — scope record: none (gate.py scope clean, 0 recorded). Divergence: the working entry's "opt-in OTel export" did not ship — justified (P4 deferral, overseer under the founder's standing delegation, 2026-10-06, provisional on the founder's word; plan §Implementation notes route CARRY → "Driver command spans"). Report deviations each justified.
4. Absence needs evidence — arch "escher-telemetry / RUST_LOG 0 hits" carry their greps (report Expected amendments); line profiles read (`splice.py summary`): architecture 14 lines > 2 000 chars, none of A1-A12's target lines among them (59 588c · 64 936c · 110 1913c · 146 1159c · 149 335c · 150 1249c · 187 1274c · 216 625c · 218 150c); security / test / obs: 0 lines > 2 000.
5. Expected-amendments reconciliation — 1 arch Names → A1 · 2 arch Env vars → A5 · 3 arch Process-wide → A6 · 4 arch Logging and timing → A10 · 5 obs §3 Logging stack + Bootstrap phases → O1, O10 · 6 obs §7 → O8 · 7 obs §8 → O9 · 8 security-plan Bootstrap / Env values read / Logging & Monitoring → raised R-S1, R-S2, R-S3 (+ R-S4) · 9 test-plan §9 Local baseline → T2. None under-run.
6. Disproved claims — (1) plan.md smoke `expect = ['exit 124', …]` unreadable by gate.py → routed: a pipeline tool defect (overseer-recorded; friction record 2026-10-06T00:40:59Z-b), chunk-local plan (not a master), operator's word sets entry 7's result = evidence/smoke-004017Z.txt — DISPOSED. (2) obs-plan §8 / CLAUDE.md "no scrub layer exists yet" → O9 (+ O10, R-S2, R-S4) and the cascade's CLAUDE.md re-derivation — DISPOSED.

Escalations: 0. → Apply.

## Cascade-side additions (orchestrator)
- C1 · citation re-point — this chunk inserted lines into `Cargo.toml` (after 16, 58, 184), `tests/blitz-tests/Cargo.toml` (after 15; 2 after 35), `examples/seven_guis/Cargo.toml` (after 31), `examples/seven_guis/src/main.rs` (3 after 5): 96 citations into them across the masters + registries, 80 shifted (architecture 62 · obs-plan 7 · a11y-plan 4 · security-plan 3 · design-system 2 · test-plan 2), each endpoint checked line-content-equal old (1ff57e3c) vs new; range ends extended where the inserted line joins the cited group (`Cargo.toml:43-58`→`44-60` in-repo deps · `183-184`→`185-187` tracing deps · `tests/blitz-tests/Cargo.toml:15-35`→`15-38` · `28-35`→`29-38` · `examples/seven_guis/src/main.rs:4-7`→`4-10`). Script: session scratchpad `repoint.py` (mechanical, dry-run 0 mismatches).
