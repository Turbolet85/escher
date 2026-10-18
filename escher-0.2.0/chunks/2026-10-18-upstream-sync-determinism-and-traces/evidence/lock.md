# Manifest, lock and audit — 2026-10-18-upstream-sync-determinism-and-traces

Measured by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18, after the take.

## Byte-identical to `fb842671`
- `git diff --quiet fb842671137fc4f5e48059f269e796b57cf25017 -- Cargo.toml Cargo.lock deny.toml` exits 0.
- `sha256sum Cargo.lock` = `117baa86731a0b01d9b4ad27e4cc4034b5d7e38c3960c2f379ee3b3dfe1f949b`, which equals
  `git show fb842671:Cargo.lock | sha256sum`.
- `Cargo.toml` `624e077bdbdbe76ff8200e138e9d9620799eb4c1f0253f46659359b679f2ef54` · `deny.toml`
  `82f08061b36dcf3c3bce2729546bdcddbecca728a5bf00335ec942580ffcc5dc`, both unchanged (the diff above).
- Entry 3 (preservation guard): green.

## Taffy stays the registry 0.15
- `Cargo.lock:8038-8041`:
  ```
  name = "taffy"
  version = "0.15.0"
  source = "registry+https://github.com/rust-lang/crates.io-index"
  checksum = "3c8480887e8803986d96a7173729565520d4bbb721ad6b7ad6941a22897207aa"
  ```
- Entry 4 (`grep -c -E 'DioxusLabs/taffy|git = ' Cargo.toml`): `0`, exit 1. The root manifest holds no git dependency.
- Entry 5 (`grep -c -E '^taffy = \{ version = "0\.15",' Cargo.toml`): `1`, exit 0.
- F1 was answered B by the delegate (the operator, 2026-10-18, PROVISIONAL; inputs#I5). It keeps the founder's rule of
  2026-10-13 (no git dependency in the root manifest).

## Audit (entry 16, `bash .github/scripts/ci-leg.sh audit`, green)
```
warning[license-not-encountered]: license was not encountered
   ┌─ ./deny.toml:46:6
   │
46 │     "NCSA",
   │      ━━━━ unmatched license allowance

advisories ok, licenses ok
```
(The tool's host path before `deny.toml` is respelt `./` here.) `advisories ok` and `licenses ok`, with the one
`license-not-encountered` warning for NCSA, as at the baseline. `deny.toml` is unchanged, and no ID was added.
