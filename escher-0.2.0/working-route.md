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
Upstream sync ahead of visual lint and real-app readiness — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked  CARRY: upstream's four Taffy-main commits past `23b9b26a9` (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`; upstream's manifest names git Taffy `ea07c4c3`, v0.15.0 and 14 unreleased commits, still at `fb8d6c15e`), left out again at 2026-10-18-upstream-sync-determinism-and-traces on F1 B (delegate the operator, ratified by the founder 2026-10-18) because they put a git Taffy in the root manifest  CARRY: whether upstream retired `keyboard_compat.rs` — still listed at `fb8d6c15e`, both coverage-leg exclusions standing; the filename exclusion drops with it (test-plan §10)  CARRY: the merge surface of 2026-10-18-engine-debts' seven engine fixes — `inert`, the toggle per role with `role="switch"`, `Node::absolute_position`, the hit's stop at `contain: paint`, the password click caret, `disabled` on form controls, the transformed client rect — flagged upstreamable against `23b9b26a9`, no upstream pull request, re-read per row at each sync (at `fb8d6c15e`: F1–F7 untouched, F8's `document.rs` moved in other functions only); by file in escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md, the re-read in escher-0.2.0/chunks/2026-10-18-upstream-sync-determinism-and-traces/evidence/readings.md (the P5 review card's rows 1–3, the operator's word, inputs#I6; placed on the operator's direction, inputs#I7)  CARRY: the merge surface of 2026-10-18-one-virtual-clock's document clock — `DocumentConfig.clock`, the `Clock` · `WallClock` · `VirtualClock` types and every engine time read taken through it (the scrollbar fade, the double-click window, pan, the fling and smooth scroll) in `packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs` and `events/pointer.rs` — flagged upstreamable, no upstream pull request, an API addition whose default keeps wall time, ratified by the founder 2026-10-18 (F1 B, delegate the operator); re-read at each sync (that chunk's P5 review card, row 1, the operator's word, its inputs#I3; placed on the operator's direction, its inputs#I4)
   ↓
Shared fixtures — fixture pages beside the stand and one held network stand-in, stated once in blitz-tests' shared modules, run through agent-run
   ↓
Visual lint — one command: overlap, clipping, truncated text, off-screen, zero size, text contrast as `stand_contrast` reads it; stable id, measured reason; fixture-proven (v020-05)
   ↓
Load wait — a command meeting a load in flight waits it out within a stated bound, on a fixture (v020-07; per security-plan §API Security)
   ↓
Select by id — a `select` driven by id like any control, read in the snapshot with role and name, Tab order kept; fixture-proven (v020-08)
   ↓
Large screen in parts — a screen over the snapshot text budget read in parts an agent can walk, proven on a fixture (v020-09)

### Epoch 5 — Polish & ship
Upstream sync ahead of polish and ship — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
Stand requirement sweep — every 0.2.0 capability proven headless on stand or fixture, inside stand-repeat; coverage floors held, new verbs' stderr id-free (intent §Principles)
   ↓
A11y re-read — stand Tab order, Enter/Space activation and contrast re-asserted after 0.2.0's engine changes; new a11y checks in the `a11y` leg (a11y-plan §9)
   ↓
Capability statement — what escher can do, its stated limits, a11y reach and what MCP client carries onward, where Viola's planning reads it (v020-10)
   ↓
Cold-agent bar — fresh agent over 0.2.0's new capabilities via the cold-agent pipe, operator's login; judged by the founder's bar from 0.1.0's baseline (v020-11)
