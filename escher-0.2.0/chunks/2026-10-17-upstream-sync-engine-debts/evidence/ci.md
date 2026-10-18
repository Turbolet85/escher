# CI — the pushed merge

- **Run:** CI#39043080454 (push), sha `984fabd47bf71695cd77a46410e0d059513163cc` (the two-parent merge commit:
  `143b6013` · `23b9b26a9`).
- **Verdict** (entry 46, `ci.py conclusion --sha HEAD --wait 3600`): `verdict: green · checks 17/17 · wall 986 s ·
  runs CI#39043080454 completed/success`. Every job settled.
- **Checks:** Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples ·
  Clippy · Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks
  repeated · Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features].
  The macOS key migration (`ACTION_MODIFIER` = `Modifiers::META`) built and passed in Test (macos). MSRV 1.91 built
  the moved dependencies. The Coverage report job ran with `keyboard_compat.rs` excluded.

## Per-OS tallies (entry 47: lines · passed · failed · ignored)
| Job | Chunk start, CI#39017887462 (`143b6013`) | This run, CI#39043080454 (`984fabd4`) | Moved |
|---|---|---|---|
| Test [default features] | 186 · 958 · 0 · 12 | 187 · 971 · 0 · 12 | +1 · +13 |
| Test (macos) | 167 · 955 · 0 · 12 | 168 · 968 · 0 · 12 | +1 · +13 |
| Coverage report | 167 · 955 · 0 · 12 | 168 · 968 · 0 · 12 | +1 · +13 |
| Test (windows) | 167 · 931 · 0 · 9 | 168 · 944 · 0 · 9 | +1 · +13 |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — |
| Stand checks repeated | 141 · 576 · 0 · 21 | 141 · 576 · 0 · 21 | — |

The chunk-start column is the plan's known-positive control (fired at P4 on CI#39017887462). Test (ios) and Test
(android) build only and print no result lines.

## Every moved count, by name
A per-job, per-target diff of the two runs' `gh run view --log` dumps (libtest result lines keyed by their `Running`
target, colour codes stripped). In each job its totals equal entry 47's figures. The same three targets move in
every test-bearing job, and nothing else:
- `Running tests/accessibility_description.rs`: absent → 3 passed · 0 failed · 0 ignored — upstream's new file (#1013).
- `Running tests/accessibility_label.rs`: absent → 10 passed · 0 failed · 0 ignored — upstream's new file (#859).
- `Running unittests src/main.rs (bump)`: 0 · 0 · 0 → absent — upstream deleted `apps/bump` (`23b9b26a9`).

So each test job moves +2 −1 = +1 result line and +13 passed, with failed and ignored unchanged. The a11y leg and
the stand repeat read the same: their files did not change in count, and the key migration changed spellings only.
Against test-plan §9's older recorded figures (linux 185 · 951 · 0 · 12, read at 2026-10-14-quality-gates), the
re-read linux tally is now 187 · 971 · 0 · 12.

## The reworded commit — CI#39055555592 (push), sha `3eda07bc825b83ae69324b3b2b0623b93fff0db5`
The sha of the founder's reword (`operator-pass.md`, *The reword*) has the same tree `45fcd733` and the same parents
`143b6013` · `23b9b26a9` as `984fabd4`.
- **Verdict** (entry 46 re-fired): `verdict: red · checks 17/17 · first-fail +549 s Dependency audit · runs
  CI#39055555592 in_progress/-`. Read at 558 s, while 11 jobs were still running.
- **Dependency audit:** `advisories FAILED, licenses ok`. There are three new unmaintained advisories with no safe
  upgrade, all reached through `zbus` 5.19.0 under `accesskit_unix` 0.24.0:
  - RUSTSEC-2026-0370 `zvariant`;
  - RUSTSEC-2026-0371 `zbus_names`;
  - RUSTSEC-2026-0372 `zvariant_derive`.

  The same tree's audit passed in CI#39043080454.
- Entry 47 was not fired: the pass stops on a red entry.

## The fix commit — CI#39056637407 (push), sha `ae8c8daf8977` (the final sha this chunk's verdict reads)
`fix(deps): ignore the zbus-family unmaintained advisories by ID` sits on `3eda07bc`. It adds the three per-ID
ignores to `deny.toml` on the operator's word (`operator-pass.md`, *The fix push*).
- **Verdict** (entry 46): `verdict: green · checks 17/17 · wall 875 s · runs CI#39056637407 completed/success`. The
  Dependency audit passes.
- **Per-OS tallies** (entry 47: lines · passed · failed · ignored). Each equals CI#39043080454's, so every count
  moved against the chunk start is the one attributed above:

| Job | CI#39043080454 (`984fabd4`) | CI#39056637407 (`ae8c8daf`) |
|---|---|---|
| Test [default features] | 187 · 971 · 0 · 12 | 187 · 971 · 0 · 12 |
| Test (macos) | 168 · 968 · 0 · 12 | 168 · 968 · 0 · 12 |
| Coverage report | 168 · 968 · 0 · 12 | 168 · 968 · 0 · 12 |
| Test (windows) | 168 · 944 · 0 · 9 | 168 · 944 · 0 · 9 |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 |
| Stand checks repeated | 141 · 576 · 0 · 21 | 141 · 576 · 0 · 21 |
