# Classified — security · dependency-security.b1

## Answers
dependency-security#1.0 · yes · ci-leg.sh:38, a conflict surface, holds the audit leg's `--locked` cargo-deny line over the whole workspace, which the stand-repeat and coverage edits must leave intact (deny.toml's per-ID ignores stay unreached)
dependency-security#2.1 · no · its six lock names and icu pins live in the root Cargo.toml and Cargo.lock, neither reached, and the chunk adds no package, so only the audit re-check every change owes applies
dependency-security#2.2 · yes · tests/blitz-tests/Cargo.toml, a merge surface, carries the cursor-icon dev-dependency, a workspace entry the merge must keep with no new dependency beside it
dependency-security#2.3 · yes · tests/blitz-tests/Cargo.toml, a merge surface, carries the test-that and test-that-macro 0.6.0 dev-dependency pins that the merge must keep
dependency-security#2.4 · no · its take lives in the root Cargo.toml, Cargo.lock and deny.toml, none reached, and the chunk runs no upstream sync and moves no pin, so only the audit re-check every change owes applies
dependency-security#2.5 · no · its take lives in the root Cargo.toml, Cargo.lock and deny.toml, none reached, and the chunk runs no upstream sync and moves no pin, so only the audit re-check every change owes applies
dependency-security#2.6 · no · its take and pins live in the root and WPT runner manifests, the lock and deny.toml, none reached, and the chunk bumps no pin, so only the audit re-check every change owes applies
dependency-security#3.1 · no · the serde_json feature set and its read-only use live in the root Cargo.toml, escher-driver's Cargo.toml and its mcp, recording and replay readers, none of them reached, and the chunk adds no feature or serde_json write, a re-check every change owes
dependency-security#4.1 · yes · tests/blitz-tests/Cargo.toml, a merge surface, carries the png dev-dependency the task names as workspace-inherited, and the chunk adds no base64 crate beside it
dependency-security#5.1 · no · the color dependency and its contrast reader live in packages/blitz-test-harness, not reached, and the chunk adds no dependency, so only the no-new-dependency re-check every change owes applies
dependency-security#6.1 · yes · ci-leg.sh:38, a conflict surface, runs the licence gate over the whole workspace graph inside the audit leg, which the stand-repeat and coverage edits must leave intact (deny.toml's [licenses] table stays unreached)
