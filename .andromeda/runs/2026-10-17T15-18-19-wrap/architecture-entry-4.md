
## 2026-10-17-upstream-sync-engine-debts — the keyboard-types 0.8 surfaces: `NamedKey`, `META`, `keyboard_compat`, the coverage exclusion
**Section:** §Standard Contracts (Test harness · CI contracts) · §Established Decisions ([Driver session] · the interaction constants) · §Existing Scopes (dioxus-native-dom)
**Change:**
- Test harness: was «`Key` and `Modifiers` — the two types `press_with` takes»; now `Key`, `Modifiers` and `NamedKey`, re-exports of keyboard-types 0.8 (`pub use keyboard_types::{Key, Modifiers, NamedKey};`).
- [Driver session]: the driver's key types come through those three re-exports; `keyboard_key` maps each of the 11 named keys to `Key::Named(NamedKey::…)`, Space staying `Character(" ")`, and the macOS `ACTION_MODIFIER` reads `Modifiers::META`.
- The engine's platform action modifier: was `SUPER` on macOS; now `META` (`SUPER` until keyboard-types 0.8), `CONTROL` elsewhere.
- dioxus-native-dom: gains the crate-private `keyboard_compat`, upstream's bridge converting keyboard-types 0.8 into Dioxus 0.7's types for `events.rs`.
- CI contracts, the `coverage` leg: both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'` until upstream retires the bridge, pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`; `coverage_floor.py` and the three floors unchanged.
**Why:** upstream's keyboard-types 0.7 → 0.8 move, taken by the sync, and our sites migrated in the smallest edit; the filename exclusion on the operator's answer to P4 fork 3.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
