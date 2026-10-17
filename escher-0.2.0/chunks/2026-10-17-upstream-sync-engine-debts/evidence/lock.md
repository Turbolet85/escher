# Lock — `Cargo.lock` resolved at the pin

## The conflict
One region, in the `blitz-tests` package's dependency list: ours listed `escher-driver`, `escher-telemetry` and
`keyboard-types 0.7.0`; upstream `keyboard-types 0.8.3`. Resolved to `escher-driver`, `escher-telemetry`,
`keyboard-types 0.8.3` — our two workspace dependents kept, upstream's keyboard-types taken (the workspace moves to
"0.8" with the merged `Cargo.toml`). Every other lock hunk auto-merged.

## The refresh (plan step 4)
- `cargo metadata --locked` on the resolved lock: exit 101 (`cannot update the lock file … because --locked was
  passed`) — the two `escher-*` entries still read `0.3.0-beta.3` beside a `0.3.0-beta.4` workspace.
- `cargo update -p escher-driver -p escher-telemetry`: exit 0, `Locking 2 packages`, `escher-driver v0.3.0-beta.3 …
  -> v0.3.0-beta.4`, `escher-telemetry v0.3.0-beta.3 … -> v0.3.0-beta.4`. `diff` of the lock before and after this
  call: the two `version` lines alone (lock lines 3094 and 3105).
- `cargo metadata --locked` after it: exit 0. It downloaded `test-that` 0.6.0 and `test-that-macro` 0.6.0.

## The set against the pin
The name@version set of the lock, `escher-*` entries set aside, against `git show 23b9b26a9:Cargo.lock`: `diff`
exit 0, no output — no third-party package enters, leaves or moves beyond upstream's set.

Against HEAD `143b6013`'s lock (whole set): the eleven in-repo packages and our two `escher-*` go 0.3.0-beta.3 →
0.3.0-beta.4; `bump` 0.0.0, `toml_datetime` 0.6.11, `toml_edit` 0.22.27 and `toml_write` 0.1.2 leave; `test-that` and
`test-that-macro` go 0.5.2 → 0.6.0. Nothing else moves (`syn` 3.0.6, the macro's dependency, was in the lock at HEAD).

## `test-that` / `test-that-macro` 0.6.0, reviewed by hand
Read from the registry sources cargo unpacked (`~/.cargo/registry/src/index.crates.io-*/test-that{,-macro}-0.6.0/Cargo.toml`):

| Crate | Licence | Repository | Build script | proc-macro | Dependencies (lock) | Direct dependent |
|---|---|---|---|---|---|---|
| `test-that` 0.6.0 | Apache-2.0 (0.5.2: the same) | https://github.com/hovinen/test-that (the same) | none — `build = false`, no `build.rs` | no | `num-traits`, `regex`, `test-that-macro` | `blitz-tests` (dev-dependency, `tests/blitz-tests/Cargo.toml:39`, workspace `test-that = "0.6.0"` at `Cargo.toml:198`) |
| `test-that-macro` 0.6.0 | Apache-2.0 (the same) | the same | none — `build = false`, no `build.rs` | yes (as 0.5.2) | `quote`, `syn` 3.0.6 | `test-that` |

Both stay test-only (blitz-tests' dev-dependencies), registry-sourced with lock checksums, no build script, and the
licence is in `deny.toml`'s allow list as before (the audit entry reads `licenses ok`).
