
## 2026-10-13-upstream-sync-polish-and-ship — the 28-name falsy predicate; no git dependency; the sync's dependency delta
**Section:** §Input Validation → Dioxus mutations, Boolean attributes with a falsy value · §Dependency Security → Pinning
**Change:**
- Dioxus mutations row: was one of the 27 names of the bridge's `BOOLEAN_ATTRIBUTES` list (the `isBoolAttr` list of Dioxus 0.7.10's web interpreter); now one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set, `inert` added. `BOOLEAN_ATTRIBUTES` retired; `mutation_writer.rs` is byte-identical to upstream's at `434e0661`. The row stays an in-process mutation path: no new input class and no crossing.
- Pinning: was "the one git dependency, taffy, is pinned by commit rev"; now no git dependency is left in the root manifest — taffy is a crates.io registry version, "0.15" resolved 0.15.0, with a checksum in `Cargo.lock`.
- Pinning gains the 2026-10-13 sync's delta (`434e0661`, merge `a6c24d24`):
  - taffy moves from git rev `7d33901c` (0.14.0) to crates.io 0.15.0, its eight features byte for byte;
  - the workspace version and the nine in-repo `=` pins move to `0.3.0-beta.3`, escher-driver and escher-telemetry following by `cargo update --workspace`;
  - blitz-tests gains the dev-dependency `cursor-icon`, already in the lock;
  - no other third-party package enters, leaves or moves — the lock's set equals upstream's with the two `escher-*` entries aside;
  - the audit leg reads `advisories ok`, with `deny.toml` unchanged.
**Why:** a closed set taken from upstream and a dependency source moved to the registry with a checksum: no boundary widened and no package added. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/
