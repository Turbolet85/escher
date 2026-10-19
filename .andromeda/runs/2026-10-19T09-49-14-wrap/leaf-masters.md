group: masters
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «each instance-verb call's `tool` · `arguments` · `accepted` and its answer's FNV-1a 64 `digest` (`answer_digest`), a masked or unresolved `type`'s text kept as `MASKED_STAND_IN`, no screen reading, never logged»
  source: .andromeda/architecture.md:186 «`tool` · `arguments` in the verb's schema order · `masked` (`true`, on a `type` alone) · `accepted` · `digest` · `answer` — the answer's line as the driver wrote it»
  replace-with: «each instance-verb call's `tool` · `arguments` · `masked` (on a `type` alone) · `accepted`, its answer's FNV-1a 64 `digest` (`answer_digest`) and the answer itself as `answer` — its line as the driver wrote it, masked as on every carrier, a shot's `{"width":…,"height":…}` with no `png` — a masked or unresolved `type`'s text kept as `MASKED_STAND_IN`, the screens' readings at rest, never logged»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «`Check::from_bytes` reads a kept check strictly through `serde_json`, anything else refused `malformed` (ratified by the founder, 2026-10-19).»
  source: .andromeda/architecture.md:187 «The replay: `Check::replay(&self, session: &mut Session) -> Replay` runs nothing on a session whose `label()` is not the check's `app()`»
  replace-with: «`Check::from_bytes` reads a kept check strictly through `serde_json` (`answer` required, a JSON object), anything else refused `malformed`, and `Check::replay` runs it again — nothing on a session of another app, else each step through `Session::run` alone, stopping at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding no reading (ratified by the founder, 2026-10-19); the repository's own two scripted Counter checks are committed under `tests/blitz-tests/checks/` (`100644`), the one exception to owner-only.»

- leaf: CLAUDE.md:44   class: stale
  leaf-text: «and its answers' digests, no screen reading, in an owner-only file never logged (ratified by the founder, 2026-10-19)»
  source: .andromeda/security-plan.md:192 «and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel — never logged»
  replace-with: «and its answers, each a digest and kept whole — masked as on every carrier, a shot's without its pixels, so the screens' ids, names, values, snapshot text and diffs at rest — in an owner-only file never logged (ratified by the founder, 2026-10-19), the two committed Counter checks under `tests/blitz-tests/checks/` (`100644`, public) the one exception to owner-only»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «each answer a 64-bit digest and no screen reading — read back strictly by `Check::from_bytes` and replayed with every step's acceptance and digest equal, ratified by the founder (2026-10-19)»
  source: .andromeda/architecture.md:119 «each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels»
  replace-with: «each answer a 64-bit digest and kept whole — masked as on every carrier, a shot's without its pixels, so the screens' ids, names, values, snapshot text and diffs at rest — read back strictly by `Check::from_bytes` and replayed through `Session::run` alone by `Check::replay`, which stops at the first step whose acceptance or kept answer differs, ratified by the founder (2026-10-19); the repository's own two scripted Counter checks committed under `tests/blitz-tests/checks/`, the one exception to owner-only, replayed by `stand_replay` and kept fresh by seven_guis `recording`»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «byte-identical across processes).»
  source: .andromeda/architecture.md:335 «a shot of one state byte-identical in two child runs and in the parent on one host, 12 shots in both layout modes — a shot's PNG bytes differ between linux x86_64 and macOS aarch64»
  replace-with: «byte-identical across processes on one host — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step).»

- leaf: CLAUDE.md:83   class: omission
  leaf-text: «`packages/escher-driver/src/{session,host,client,wire,error,recording}.rs` · host `examples/seven_guis/src/session_host.rs` · checks `tests/blitz-tests/tests/stand_session_*.rs` and `stand_settle.rs` (shared»
  source: .andromeda/architecture.md:218 «The committed checks: `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, git mode `100644` in the public repository — the one exception to a kept check's owner-only medium»
  replace-with: «`packages/escher-driver/src/{session,host,client,wire,error,recording,replay}.rs` · host `examples/seven_guis/src/session_host.rs` · the committed checks `tests/blitz-tests/checks/counter-{cli,mcp}.json`, written by `examples/seven_guis/tests/flows/record_checks.sh` · checks `tests/blitz-tests/tests/stand_session_*.rs`, `stand_replay.rs` and `stand_settle.rs` (shared»

- leaf: CLAUDE.md:13   class: omission
  leaf-text: «the stand checks share `tests/common/` and the checks that hold a session `tests/session_common/`»
  source: .andromeda/architecture.md:245 «tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays»
  replace-with: «the stand checks share `tests/common/` and the checks that hold a session `tests/session_common/`, and `checks/` holds the two committed kept checks `stand_replay` replays (data, no target)»
