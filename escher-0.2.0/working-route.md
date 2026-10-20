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
Upstream sync ahead of polish and ship — upstream/main merged, changes mostly additive; CI and audit green, per-OS tallies re-read, sink allowlist re-checked  CARRY: upstream's four Taffy-main commits past `23b9b26a9` (`1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`; upstream's manifest names git Taffy `ea07c4c3`, still at `17de049b1`), left out again at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness on F1 «Cherry-pick two (Recommended)» (delegate the operator, PROVISIONAL) because they put a git Taffy in the root manifest; re-read in escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/readings.md (that chunk's P5 review card row 1, the operator's word, its inputs#I3)  CARRY: whether upstream retired `keyboard_compat.rs` — still listed at `17de049b1`, both coverage-leg exclusions standing; the filename exclusion drops with it (test-plan §10); re-read in the same readings.md (card row 2)  CARRY: the merge surface of 2026-10-18-engine-debts' seven engine fixes — `inert`, the toggle per role with `role="switch"`, `Node::absolute_position`, the hit's stop at `contain: paint`, the password click caret, `disabled` on form controls, the transformed client rect — flagged upstreamable against `23b9b26a9`, no upstream pull request, re-read per row at each sync (at `17de049b1`: every row's named functions untouched, `node/node.rs`, `events/keyboard.rs` and `document.rs` moved by upstream's focus hunks in other functions only); by file in escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md, the re-read in the same readings.md (card row 3)  CARRY: the merge surface of 2026-10-18-one-virtual-clock's document clock — `DocumentConfig.clock`, the `Clock` · `WallClock` · `VirtualClock` types and every engine time read taken through it, in `packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs` and `events/pointer.rs` — flagged upstreamable, no upstream pull request, an API addition whose default keeps wall time; re-read at each sync (at `17de049b1`: `document.rs` alone reached, in its three focus functions, no wall-clock read added; the same readings.md, card row 4)  CARRY: the merge surface of 2026-10-20-select-by-id's select model — its keys and typeahead, click focus, the choice held as `CHECKED`, the options kept in the accessibility tree, the `change` event and the labelable `<label>`, in blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs` and `assets/default.css`, blitz-traits `events.rs` and blitz-vibey-script `runtime.rs` — flagged upstreamable, no upstream pull request; re-read at each sync (escher-0.2.0/chunks/2026-10-20-select-by-id/report.md)
   ↓
Stand requirement sweep — every 0.2.0 capability proven headless on stand or fixture, inside stand-repeat; coverage floors held, new verbs' stderr id-free (intent §Principles)  CARRY: the visual lint's readings no check asserts — a clipped or off-screen node holding text also reads `truncated`, and `BaseDocument::text_extent`'s stated limits (RTL and vertical text read as LTR lines, a transformed box at its client rect); measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md (Decisions «Untested reading»; Symbols / APIs)
   ↓
A11y re-read — stand Tab order, Enter/Space activation and contrast re-asserted after 0.2.0's engine changes; new a11y checks in the `a11y` leg (a11y-plan §9)
   ↓
Capability statement — what escher can do, its stated limits, a11y reach and what MCP client carries onward, where Viola's planning reads it (v020-10)  CARRY: README.md still says the MCP role gives «the eleven driver commands as tools»; since 2026-10-19-visual-lint's gated part the verb table holds twelve, `lint` the eighth instance verb on every carrier (escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md, Symbols / APIs)
   ↓
Cold-agent bar — fresh agent over 0.2.0's new capabilities via the cold-agent pipe, operator's login; judged by the founder's bar from 0.1.0's baseline (v020-11)
