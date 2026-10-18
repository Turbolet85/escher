
## 2026-10-17-upstream-sync-engine-debts — the bump CLI's rows leave; no physical key code hits `todo!()`
**Section:** §Input Validation (the `CLI arguments (bump)` row) · §Error Handling (External responses · the panic paths)
**Change:**
- The `CLI arguments (bump)` row and §Error Handling's «bump's `bail!` macro» clause are gone with the crate.
- The panic paths: was «KeyboardBacklightToggle and any unlisted physical key code hit todo!()»; now the shell passes winit's physical key `Code` through, so no physical key code hits a `todo!()`.
**Why:** upstream's `23b9b26a9` "Delete the bump crate" and its keyboard-types 0.8 converters, taken by the sync.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
