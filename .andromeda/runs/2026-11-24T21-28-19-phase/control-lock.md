# The lock entries' controls — P5 check 4 (9)

Scratch clone: `git clone --shared` of the project at HEAD `239c197c10917fb9c7a67b687e00b5dce1f74723`, in
`$TMPDIR/{scratchpad}/lock-clone` (the session scratchpad). Each entry fired AS ITSELF through
`gate.py run --chunk 2026-11-24-canvas-lint --docs-root . --entry {n} --project-root {the clone}`, no run dir.

The clone's input (step 1 as the plan builds it): `skrifa = { workspace = true }` added after `serde_json` in
`packages/escher-driver/Cargo.toml`, then one unlocked `cargo tree -p escher-driver -e normal --offline` (exit 0).

The whole lock diff it wrote, measured:

```
diff --git a/Cargo.lock b/Cargo.lock
index c5aaff411..30d070321 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -3216,2 +3216,3 @@ dependencies = [
  "serde_json",
+ "skrifa",
  "tracing",
```

`git diff --numstat`: `1	0	Cargo.lock` · `1	0	packages/escher-driver/Cargo.toml`.

| entry | input | reading |
|---|---|---|
| 1 (numstat) | the untouched tree (the real tree, baseline) | red · last line '1 0 Cargo.lock' ✗ (last: '') |
| 1 (numstat) | the clone with step 1 applied — must pass | green · exit 0 |
| 1 (numstat) | the clone with step 1 applied and escher-driver's `"tracing"` edge also changed to `"tracing-log"` — must fail | red · last line '1 0 Cargo.lock' ✗ (last: '2 1 Cargo.lock') |
| 2 (whole diff) | the untouched tree (the real tree, baseline) | red · contains '+ "skrifa",' ✗ |
| 2 (whole diff) | the clone with step 1 applied — must pass | green · exit 0 |

The clone is the session's own scratch, left in place; no plan step removes it.
