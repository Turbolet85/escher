# obs extract

## Relevance
partial — the chunk adds no telemetry surface by its own boundary (the diff is returned to its caller only); obs binds it through the feature-gating rule for any engine log site it adds, the scrub sets its content must stay out of, and the sinkless shape of the stand check.

## Constraints
- Obs tier is 0 and the plan mandates no instrumentation for this chunk: obs-plan §1 names no must-trace path or telemetry trigger (that part of §1 is unmeasured and is not cited), §4 records no spans anywhere in the workspace and §5 records no metric exporter — so the drain, the flag and the diff carry no required span, metric or log event (per obs-plan §1 Obs Scope Summary · §4 Span / Trace Coverage · §5 Metric Coverage).
- Any `tracing` event the chunk adds in blitz-dom, blitz-shell or dioxus-native-dom is required to be compiled only under that crate's `tracing` feature, with a no-op path when the feature is off (per obs-plan §2 Telemetry mechanism). Whether the mutation sites the chunk extends already carry gated log sites is research's question.
- A diff carries ids, accessible names and a text control's value; obs-plan §8 requires that on escher's sink an engine-target event prints only the allowlisted field names and that the content-named set is redacted at every target — so no field the chunk adds may carry diff content, and none may be given a name that reads as allowlisted while holding content (per obs-plan §8 Scrubbing).
- The scrub's reach is escher's sink only: obs-plan §8 records that the upstream apps' stdout subscribers and the WPT runner's logger print engine fields unscrubbed, and blitz-dom and blitz-shell are compiled into those binaries — a content field added at a change-tracking site there is printed raw (per obs-plan §8 Scrubbing, its reach paragraph · §8 Values logged as-is).
- Panic and assertion text is past the scrub: obs-plan §8 records that the chained std panic hook prints the raw panic message to stderr, and §7 that the escher hook redacts only its own `panic.payload` field — so library code the chunk adds must put no diff content into a `panic!` / `expect` / `assert!` message (per obs-plan §7 Panic hooks · §8 Values logged as-is).
- The stand check's raw output is unscrubbed by design: obs-plan §6 and §9 record that `target/agent-run/run.log` and the per-leg CI log hold raw cargo/libtest output, the latter uploaded as a failure artifact described as carrying no user data — a failing assertion that formats diff content would place it in both (per obs-plan §6 Log format (the agent-run harness) · §9 Telemetry artifact handling, the per-leg CI log row · §8 Scrubbing, the agent-run paragraph).
- The headless stand and its checks are recorded as installing no subscriber, reading no env var and printing nothing; the new `stand_*` check is required to keep that description true (per obs-plan §3 Logging stack, the headless-stand bullet).

## Patterns to follow
- The feature-gated call site with a no-op fallback, as obs-plan §2 Telemetry mechanism cites it for blitz-dom's document, mutator and layout sites — the only admissible shape if a change-tracking log site is added at all.
- `node_id` as the per-node field of an engine event: it is the allowlisted node identity (obs-plan §8 Scrubbing) and the field the existing engine events use (obs-plan §6 Logged events → blitz-dom events / layout). The stable element id is not in the allowlist — it would print redacted on escher's sink and raw on the upstream ones.
- Counts and identities, never content, in anything recorded about a run: the shape obs-plan §6 states for the cold-agent verdict and the stub's call log, and §8 for the agent-run events — the model for any evidence the chunk commits (entry counts, mode and id labels; not diff entries).
- The stand check as an in-process read with no sink (obs-plan §3 Logging stack): the diff is asserted on as a returned value, the way the snapshot checks read theirs.

## Anti-patterns to avoid
- obs-plan §11 Obs Anti-Patterns holds no recorded intent, so no ban is cited from it; the bans below follow from §2 and §8.
- An unconditional `println!` / `eprintln!` / `dbg!` or an ungated `tracing` call in library code at a tracking, drain or diff site (per obs-plan §2 Telemetry mechanism).
- A new log field carrying a diff's content — an id, an accessible name, a control's value, or a before/after reading — at any target, or logging the changed set's contents beyond `node_id` (per obs-plan §8 Scrubbing · §8 Values logged as-is). Whether the existing dioxus-native-dom mutation-writer debug events (obs-plan §6 Logged events → dioxus-native, dioxus-native-dom; §8 records them as carrying text contents and attribute values) sit on the chunk's path, and are left as they are rather than extended, is research's question.

## Contract bindings
- obs ↔ tests (test-plan §3, the agent-run contract): the new `stand_*.rs` file enters `agent-run.sh run stand` by glob and surfaces as `test {file, test, outcome}` events; the harness reads nothing between a `failures:` line and the next result line, so the event stream stays content-free while `run.log` does not (obs-plan §3 Agent-run harness log · §6 Log format (the agent-run harness)).
- obs ↔ security (security-plan §Input Validation, the `id` / accessible-name / masked-value rows the scope cites): the returned-value-only answer for the diff is the security side; the scrub sets of obs-plan §8 are the obs side of the same rule. The answer's provisional standing is not obs's to settle.
- obs ↔ a11y: the plan records no obs-side contract for the shell's AccessKit tree update that the corrected flag gates; obs-plan §6 lists blitz-shell's log events as unsupported-input variants only. Whether a log site sits at the flag's caller is research's question.
- Keyed contract `Bootstrap phases (derive for route / setup-project)` (obs-plan §3): not read — the chunk is not obs bootstrap infrastructure and does not turn on it.

## Acceptance criteria contributions
- (obs) No diff content — stable id, accessible name, control value, or a before/after reading — appears in a `tracing` field, a `println!` / `eprintln!` / `dbg!`, or a panic / assertion message in the library or check code the chunk adds (per obs-plan §8 Scrubbing · §8 Values logged as-is · §7 Panic hooks).
- (obs) Every `tracing` call site the chunk adds, if any, sits under `#[cfg(feature = "tracing")]` with a no-op path, and the touched crates build with the feature off and on (per obs-plan §2 Telemetry mechanism).
- (obs) Every field on an engine-target event the chunk adds, if any, is one of the allowlisted names and none is in the content-named set (per obs-plan §8 Scrubbing).
- (obs) The new stand check installs no subscriber, reads no env var and prints nothing, and under `agent-run.sh run stand` yields only `test` events of file · test · outcome (per obs-plan §3 Logging stack · §6 Log format (the agent-run harness)).
