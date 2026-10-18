
## 2026-10-17-upstream-sync-engine-debts — the sync at `23b9b26a9`: pin, workspace version, keyboard-types 0.8, test-that 0.6.0
**Section:** §Project Intent (Upstream sync) · §Stack and Technologies (Workspace package · Windowing and input · Parallelism and misc) · §Inherited Defaults (Framework)
**Change:**
- Upstream sync: was «last merged at `434e0661` (merge `a6c24d24`, 2026-10-13)»; now last merged at `23b9b26a9` (merge commit `3eda07bc`, parents `143b6013` · `23b9b26a9`, 2026-10-17), the next sync's merge base — upstream read at `1e5479ae`, 14 commits past `434e0661`, the pin taking 10; the four Taffy-main commits past it wait for the next sync (`6032d0b9d` inside it reverted by upstream's own `85340347f`), so Taffy stays the registry "0.15".
- Workspace package and the engine crates' pins: was `0.3.0-beta.3`; now `0.3.0-beta.4`, the nine in-repo `=` pins with it.
- keyboard-types: was "0.7"; now "0.8" (lock 0.8.3), while Dioxus 0.7.3 carries its own 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`.
- test-that: was "0.5.2"; now "0.6.0", reviewed by hand at the bump.
**Why:** the upstream sync's merge of `23b9b26a9` at the pin the operator chose at its P4 fork; dated readings at `434e0661` elsewhere in the body stand as dated.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
