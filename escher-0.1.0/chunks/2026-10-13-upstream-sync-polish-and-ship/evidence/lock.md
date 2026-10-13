# Lock review — 2026-10-13-upstream-sync-polish-and-ship

Read 2026-10-13 by /andromeda-implement on the merged, uncommitted tree (chunk start `a703136`, `MERGE_HEAD` `434e0661`).

## The textual merge
- `Cargo.lock` merged without a conflict marker. As merged it listed `escher-driver` and `escher-telemetry` at `0.3.0-beta.2`, while both take `version.workspace = true` and the workspace version is now `0.3.0-beta.3`.

## Step 7 — the two workspace entries
- `cargo update --workspace`, exit 0: "Locking 2 packages" — `escher-driver v0.3.0-beta.2 -> v0.3.0-beta.3`, `escher-telemetry v0.3.0-beta.2 -> v0.3.0-beta.3`.
- The lock's diff against the textually merged lock is two lines, the `version` line of each of those two stanzas. No other line moved.

## The package set (name@version, read from the `name` / `version` lines)
- Against upstream's lock at the pin `434e0661`, with our two `escher-*` entries set aside: `diff` prints nothing, exit 0. No third-party package enters, leaves or moves.
- Against the chunk start `a703136`, 15 entries move, and only these:
  - the eleven in-repo packages, `0.3.0-beta.2` → `0.3.0-beta.3`: `blitz`, `blitz-dom`, `blitz-html`, `blitz-net`, `blitz-paint`, `blitz-shell`, `blitz-test-harness`, `blitz-tests`, `blitz-traits`, `blitz-vibey-script`, `stylo_taffy`;
  - our two, `escher-driver` and `escher-telemetry`, `0.3.0-beta.2` → `0.3.0-beta.3`;
  - `taffy` `0.14.0` → `0.15.0`.

## taffy 0.15.0 as the lock records it
- before: `version = "0.14.0"` · `source = "git+https://github.com/DioxusLabs/taffy?rev=7d33901cc383489d9945ae796a1ceed0a935ef66#7d33901cc383489d9945ae796a1ceed0a935ef66"`
- after: `version = "0.15.0"` · `source = "registry+https://github.com/rust-lang/crates.io-index"` · `checksum = "3c8480887e8803986d96a7173729565520d4bbb721ad6b7ad6941a22897207aa"`
