# P4 stop — 2026-11-23-canvas-placement-and-text revision 1 (inputs#I8, inputs#I9 point 3)

Measured on a scratch clone of HEAD `d43bc0e7f` carrying the chunk's tree (the working diff of `Cargo.lock`, `packages/`,
`tests/` and the three new checks), committed in the clone, then moved.

## The directed move: `cargo update -p cookie --precise 0.18.3` — exit 0
`Updating cookie v0.18.2 -> v0.18.3`. `Cargo.lock` 4+ 4−:
- line 856, `bindgen` 0.73.2: `- "syn 3.0.6",` / `+ "syn 2.0.119",`
- line 2042, `cookie`: `version = "0.18.2"` → `"0.18.3"`
- line 2044, `cookie`: checksum `1a373e36…f2f87` → `d55f2d6c…5f8a`
- line 2258, `cssparser-macros` 0.7.1: `- "syn 3.0.6",` / `+ "syn 2.0.119",`

Two packages besides `cookie` re-resolved: the two `syn` edges residual `res-evkq636g` names (the `url` move's
pair), whose `syn` 2.0.119 edge on `bindgen` fails to compile there (E0277). Evidence:
`.andromeda/cache/chunks/2026-11-23-canvas-placement-and-text/evidence/cookie-lock-move.diff`.
Stopped here per inputs#I9 point 3: no `--locked` build or audit was run on this lock, and nothing is planned.

## Read beside it (not planned): the two `cookie` lines alone, edited into the lock
Lock 2+ 2− (the version and checksum lines only, the `syn` edges kept at 3.0.6, as `res-evkq636g`'s `url` lock was kept).
- `cargo tree --locked --workspace --all-features -i cookie --depth 3 -e normal` — exit 0, lock unchanged:
  `cookie v0.18.3` ← `cookie_store v0.22.1` ← `reqwest v0.13.5` ← `blitz-net` · `http-cache-reqwest` · `rdme` ·
  `reqwest-middleware`; and `reqwest v0.13.5` direct.
- `bash .github/scripts/ci-leg.sh audit` — exit 0, last line `advisories ok, licenses ok`.
- `cargo build --workspace --locked` — not run.
Evidence: `.andromeda/cache/chunks/2026-11-23-canvas-placement-and-text/evidence/cookie-lock-hand-edit.diff`.
