# The lock and the manifests — 2026-10-27-upstream-sync-engine-debts

Read 2026-10-27, on the take's working tree (base `de85792f5e9dcb895d75bb65a3b91e79a4ea24c2`), run
`.andromeda/runs/2026-10-27T09-58-03-implement/`.

- **No `cargo update`, no hand edit of `Cargo.lock`.** `git diff --stat de85792f5 -- Cargo.toml Cargo.lock deny.toml
  '*/Cargo.toml'`: empty (exit 0). The preservation guard (gate entry 5, `git diff --quiet de85792f5 -- Cargo.toml
  Cargo.lock deny.toml packages examples tests apps wpt .github scripts flake.nix` minus the take's files) reads green,
  exit 0. So `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to the base, and no package enters, leaves
  or moves: Stylo 0.22.0, taffy 0.15 from the registry, Dioxus 0.7.3, winit `=0.31.0-beta.3`.
- **No git dependency** in the root manifest (entry 6: `grep -c -E 'DioxusLabs/taffy|DioxusLabs/stylo|git = '
  Cargo.toml` → 0, exit 1).
- **The audit** (entry 21, `bash .github/scripts/ci-leg.sh audit`): `advisories ok, licenses ok`. No
  `advisory-not-detected` warning was printed, so every standing `deny.toml` ignore is still encountered —
  RUSTSEC-2026-0192 through winit beta.3's chain among them; no ignore added or dropped. Its two standing warnings:
  `license-not-encountered` (`"NCSA"`, `deny.toml:46`) and `yanked` clipboard-win 5.4.1 (← arboard 3.6.1 ←
  blitz-shell).
- **No platform adapter in the stand graph** (entry 29: `cargo tree --locked -p seven_guis -e normal` → 0
  `accesskit_(xplat|winit)` lines).

## Left out — sixteen of upstream's 27 not yet in (research.md §The trial takes; the founder's word, inputs#I4)

| commits | measured reason |
|---|---|
| `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6` · `b8f4ba6fd` · `9babd796a` | git Taffy — residual line 35, left out at the last sync |
| `06ecec886` with `3954ecb85` · `1c5265a8b` | git Stylo 0.23-pre; our `contain: paint` hit stop's `clone_contain()` getter absent under Stylo 0.23 — residual line 35 |
| `286874cc1` | Dioxus 0.8.0-alpha.3 with the vdom port of escher's stable-id derivation (10 + 15 errors measured then) — residual line 35 |
| `a09f46792` · `527394724` | winit `=0.31.0-beta.4` and winit's clipboard API in place of `arboard` — shape B, measured green (210 · 1128 · 0 · 12) but it moves the exact winit pin and the lock; not the founder's take |
| `d3ed4e2c7` · `ccfe15332` | git Taffy `1009534e…` then `e90751fd…` |
| `0980cce5f` | reversed scroll origins through Taffy's `ScrollableOverflow`: needs git Taffy (the pin alone: 10 errors in blitz-dom), authored on Stylo 0.23, and its hunks collide with escher's scrolled-box bounds and hit (`writing_mode.rs`, `node/node.rs` ×2, `document.rs`) |
| `ebacd68bd` | fixes a panic `0980cce5f` introduced in `hit_inner` ("Since #1294"); its `node/node.rs` hunk edits a line our tree does not hold |

The merge base stays `23b9b26a9`.
