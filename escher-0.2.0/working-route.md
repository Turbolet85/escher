# Working Route — escher-0.2.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-10-16-handler-splits-and-readme] Handler splits and README — range and activation handler arms as named functions, `parse_float` in steps; README states 0.1.0 done, 0.2.0 planned, «Escher» capitalized (v020-01)
   ↓
[2026-10-16-file-splits] File splits — escher's inline test modules out of `document.rs`, upstream body untouched; `mcp.rs`, `schema.rs` along their seams, spans unchanged (v020-01; audit M2, M3)
   ↓
[2026-10-16-fast-failing-mutants] Fast-failing mutants — the six timeout-only mutants each fail a test fast; mutation score held at 0 survivors (v020-02)

### Epoch 2 — Engine debts
[2026-10-17-upstream-sync-engine-debts] Upstream sync ahead of engine debts — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
[2026-10-18-engine-debts] Engine debts — nine recorded engine deferrals, each the founder's fork: fixed and proven on stand or fixture, pinned as a limit, or kept (v020-03)

### Epoch 3 — Determinism and traces
[2026-10-18-upstream-sync-determinism-and-traces] Upstream sync ahead of determinism and traces — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
[2026-10-18-one-virtual-clock] One virtual clock — every time a headless run reads, scrollbar fade included, from one clock the driver alone moves; repeat shots identical (v020-06)
   ↓
[2026-10-19-session-recording] Session recording — a driver session via CLI or MCP kept as an owner-only replayable check, never logged; password handling per security-plan (v020-04)
   ↓
[2026-10-19-recorded-check-replay] Recorded-check replay — recorded checks run as ordinary tests under agent-run, CI and stand-repeat, steps re-validated; red on behaviour change, green otherwise (v020-04)

### Epoch 4 — Visual lint and real-app readiness
[2026-10-19-upstream-sync-visual-lint-and-real-app-readiness] Upstream sync ahead of visual lint and real-app readiness — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
[2026-10-19-test-splits] Test splits — inline test modules out of escher-driver's `cli.rs` and `recording.rs`; `host_log`/`mcp_log`'s shared fragment stated once; behaviour unchanged (audit 2026-10-19T11-16-25-code-audit M1, M2)
   ↓
[2026-10-19-shared-fixtures] Shared fixtures — fixture pages beside the stand and one held network stand-in, stated once in blitz-tests' shared modules, run through agent-run
   ↓
[2026-10-19-visual-lint] Visual lint — one command: overlap, clipping, truncated text, off-screen, zero size, text contrast as `stand_contrast` reads it; stable id, measured reason; fixture-proven (v020-05)
   ↓
[2026-10-20-load-wait] Load wait — a command meeting a load in flight waits it out within a stated bound, on a fixture (v020-07; per security-plan §API Security)
   ↓
[2026-10-20-select-by-id] Select by id — a `select` driven by id like any control, read in the snapshot with role and name, Tab order kept; fixture-proven (v020-08)
   ↓
[2026-10-20-large-screen-in-parts] Large screen in parts — a screen over the snapshot text budget read in parts an agent can walk, proven on a fixture (v020-09)

### Epoch 5 — Polish & ship
[2026-10-24-upstream-sync-polish-and-ship] Upstream sync ahead of polish and ship — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
[2026-10-24-stand-requirement-sweep] Stand requirement sweep — every 0.2.0 capability proven headless on stand or fixture, inside stand-repeat; coverage floors held, new verbs' stderr id-free (intent §Principles)
   ↓
A11y re-read — stand Tab order, Enter/Space activation and contrast re-asserted after 0.2.0's engine changes; new a11y checks in the `a11y` leg (a11y-plan §9)
   ↓
Capability statement — what escher can do, its stated limits, a11y reach and what MCP client carries onward, where Viola's planning reads it (v020-10)  CARRY: README.md still says the MCP role gives «the eleven driver commands as tools»; since 2026-10-19-visual-lint's gated part the verb table holds twelve, `lint` the eighth instance verb on every carrier (escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md, Symbols / APIs)  CARRY: the stated limit that an acting verb's diff and a lint's findings are not paged — on a large screen either can pass 10,000 bytes, held only by the 1,048,576-byte answer bound, while `snapshot` reads a screen over the budget in parts (the operator's «yes» on the 2026-10-20-large-screen-in-parts P5 card, row 1: escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/inputs/I3-relay-3.md.txt; measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md, Counts «The stated limit»)
   ↓
Cold-agent bar — fresh agent over 0.2.0's new capabilities via the cold-agent pipe, operator's login; judged by the founder's bar from 0.1.0's baseline (v020-11)
