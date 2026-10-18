
## 2026-10-17-upstream-sync-engine-debts — the harness re-exports `NamedKey`; test-that named as the assertion crate (§3)
**Section:** §3 Test Harness Contract (blitz-test-harness → Core · Runners and invocation → Rust tests · Third-party test frameworks)
**Change:**
- Core: was «re-exports `keyboard_types::Key` and `keyboard_types::Modifiers`, the two types `press_with` takes»; now keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` — the two types `press_with` takes and the named-key enum a 0.8 `Key::Named` holds.
- Third-party test frameworks: beside the slice search, blitz-tests carries test-that 0.6.0 and test-that-macro 0.6.0 as dev-dependencies (0.5.2 until this sync, reviewed by hand at the bump), whose `verify_that!` and `TestResult<()>` the merged checks assert through.
- Rust tests: the merged checks return `TestResult<()>` and assert with `verify_that!` under the same `#[test]` harness.
**Why:** the keyboard-types 0.8 move and the test-that bump, both taken by the sync.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
