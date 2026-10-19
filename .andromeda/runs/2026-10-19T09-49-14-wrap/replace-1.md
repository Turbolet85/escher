- line: .andromeda/architecture.md:57
old: «taken by escher-driver alone as its reader of MCP requests and of kept checks (packages/escher-driver/Cargo.toml:17)»
new: «taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17)»
- line: .andromeda/architecture.md:115
old: «a reader of MCP requests and kept checks only that writes nothing»
new: «a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing»
- line: .andromeda/architecture.md:119
old: «each answer as a 64-bit digest alone and no screen reading, never logged and never named by a tool call; the answer kept whole, a shot's pixels excepted, is owed at «Recorded-check replay».»
new: «each answer as a 64-bit digest and kept whole — its line as the driver wrote it, masked as on every carrier, a shot's without its pixels — so a kept check holds the screens' ids, names, values, snapshot text and diffs at rest, never logged and never named by a tool call; a kept check replays through `Session::run` alone (`Check::replay`), and the repository's own two scripted stand checks are committed in git, the one exception to owner-only.»
- line: .andromeda/architecture.md:180
old: «`TaskShell` staying private (examples/seven_guis/src/app.rs:7; examples/seven_guis/src/app.rs:90)»
new: «and `pub fn TaskShell`, the shell a test-local variant of a task mounts in so its unkeyed ids read as the task's (examples/seven_guis/src/app.rs:7; examples/seven_guis/src/app.rs:90; examples/seven_guis/src/app.rs:164)»
- line: .andromeda/architecture.md:182
old: «the crate re-exports 49 names»
new: «the crate re-exports 51 names»
- line: .andromeda/architecture.md:182
old: «the recording's `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN` from private modules»
new: «the recording's `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN` and the replay's `Replay` and `Change` from private modules»
- line: .andromeda/architecture.md:186
old: «`accepted` · `digest`, and rewrites the check whole»
new: «`accepted` · `digest` · `answer` — the answer's line as the driver wrote it (`Outcome::to_json` or `Refusal::to_json`, private `written`) inlined as its own JSON object, masked as on every carrier, a shot's written by `json::shot_without_image(width, height)` so `{"width":…,"height":…}` with no `png` (crate-private `kept_line`, computed once by `Recording::record`, whose sole caller is still `Session::run`) — and rewrites the check whole»
- line: .andromeda/architecture.md:186
old: «every other `type` its text as it crossed; no screen reading is kept.»
new: «every other `type` its text as it crossed; each answer is kept whole, a shot's pixels excepted.»
- line: .andromeda/architecture.md:186
old: «`digest` 16 lowercase hex — anything else refused»
new: «`digest` 16 lowercase hex, `answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, as for every key) — anything else refused»
- line: .andromeda/architecture.md:186
old: «`Step::{call, accepted, digest, masked}` read it (packages/escher-driver/src/recording.rs:37-60;»
new: «`Step::{call, accepted, digest, masked}` read it, the kept answer held in a private field read by `pub(crate) fn answer` and no public accessor returning a `serde_json` type (packages/escher-driver/src/recording.rs:37-60; packages/escher-driver/src/recording.rs:385-388;»
- line: .andromeda/architecture.md:192
old: «and no log or event carries the snapshot or its text, nor does a kept check, which holds a recorded session's calls and its answers' digests alone;»
new: «and no log or event carries the snapshot or its text, while a kept check holds it at rest — each kept answer whole, a `snapshot`'s text and an acting verb's diff included — in an owner-only file never logged, the two committed checks under `tests/blitz-tests/checks/` the one exception to owner-only;»
- line: .andromeda/architecture.md:220
old: «and `recording` does both, with `--record` —»
new: «and `recording` does both, with `--record`, and runs `tests/flows/record_checks.sh` under `sh` into `rc-fresh`, both streams drained, a `Started` guard and the bounded run's 90 s, comparing both recorded files byte for byte with the committed copies —»
- line: .andromeda/architecture.md:328
old: «fifteen private modules»
new: «sixteen private modules»
- line: .andromeda/architecture.md:328
old: «recording (the crate-private writer of a kept check and its directory check, unix only, with `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN`; json gaining a signed `integer` writer for it)»
new: «recording (the crate-private writer of a kept check — each record's kept answer line `kept_line`, a shot's without its image — and its directory check, unix only, with `Check`, `Step`, `answer_digest` and `MASKED_STAND_IN`; json gaining a signed `integer` writer for it) and replay (`Check::replay`, `Replay` and `Change`: a kept check's calls run again through `Session::run`, stopping at the first step whose acceptance or kept answer differs)»
- line: .andromeda/architecture.md:333
old: «the four lean tasks recorded, read back and run again with every step's acceptance and digest equal in both layout modes,»
new: «the four lean tasks recorded, each record ending with its kept answer (a shot's `{"width","height"}`, no `png` and no shot's first 64 base64 characters), and replayed through `Check::replay` in both layout modes — each script ending `snapshot` · `screenshot`, so the shot path's kept form is covered in process on each unix OS CI runs (linux, macOS; the file is unix only, as recording is) —»
- line: .andromeda/architecture.md:333
old: «(tests/blitz-tests/tests/stand_session_recording.rs:109-302);»
new: «(tests/blitz-tests/tests/stand_session_recording.rs:109-302); the committed checks replayed — `stand_replay`, 3 tests in both layout modes, not unix-gated: each committed check holding on a fresh stand Counter, each red at its first activation step (1 for the command line's check, 3 for the MCP role's) on a test-local variant `Counter` that adds 2, every earlier step held and `changed` among its keys, and a check on a session of another app running nothing, its failure messages holding the mode, the file and a step index alone (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:206-237; tests/blitz-tests/tests/stand_replay.rs:239-261);»
- line: .andromeda/architecture.md:333
old: «a shot of one state byte-identical in two child runs and in the parent, 12 shots in both layout modes»
new: «a shot of one state byte-identical in two child runs and in the parent on one host, 12 shots in both layout modes — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step (as measured at CI#39225375577)»
- line: .andromeda/architecture.md:333
old: «by `stand_requirement_sweep` and by `stand_virtual_clock`, twenty-three readers,»
new: «by `stand_requirement_sweep` and by `stand_virtual_clock`, and by `stand_replay`, twenty-five readers,»
- line: .andromeda/architecture.md:333
old: «the two `screenshot` builders and the `shot` reader (tests/blitz-tests/tests/session_common/mod.rs:1-7;»
new: «the two `screenshot` builders and the `shot` reader, and `shot_prefix`, the first 64 base64 characters of a whole-screen shot as the driver writes it (tests/blitz-tests/tests/session_common/mod.rs:1-7; tests/blitz-tests/tests/session_common/mod.rs:240-250;»
- line: .andromeda/architecture.md:334
old: «`tests/flows/`, a directory of two POSIX `sh` scripts (`counter.sh`, `flight.sh`) that `cli_flow` runs and that is no target either (examples/seven_guis/tests/host_binary.rs:1-4;»
new: «`tests/flows/`, a directory of three POSIX `sh` scripts — `counter.sh` and `flight.sh`, which `cli_flow` runs, and `record_checks.sh`, which `recording` runs to record the two committed checks and compare them byte for byte — and no target either (examples/seven_guis/tests/recording.rs:205-262; examples/seven_guis/tests/host_binary.rs:1-4;»
