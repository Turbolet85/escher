# The lock (step 3)

`cargo update -p tokio`, 2026-10-24T11:40Z, after the ten picks — cargo printed `Locking 1 package to highest compatible version` · `Updating tokio v1.53.2 -> v1.54.1`, exit 0. `Cargo.toml` (`tokio = "1.42"`) and `deny.toml` unchanged (the preservation guard entry).

## The lock diff against the base `a356f9086`
`git diff --numstat a356f9086 -- Cargo.lock` → `15	14` (the P3 forecast). Read by parsing both locks (`tomllib`):
- **packages:** 987 on the base, 987 now; entered `tokio 1.54.1`, left `tokio 1.53.2` — the one version move; no other package enters or leaves (`git diff … | grep -c '^[+-]name = '` reads 0: tokio keeps its `name` line).
- **tokio:** version `1.53.2` → `1.54.1`, checksum `e95f91fc…` → `b52fd325…`.
- **twelve edges re-pointed by cargo, each to a version already locked:**
  - `windows-sys 0.61.2` → `0.60.2`: `anstyle-query 1.1.5`, `anstyle-wincon 3.0.11`, `socket2 0.6.5`, `uds_windows 1.2.1`;
  - `windows-sys 0.61.2` → `0.59.0`: `dirs-sys 0.5.0`, `nu-ansi-term 0.50.3`, `term 1.2.1`;
  - `windows-sys 0.61.2` → `0.52.0`: `errno 0.3.14`, `rustix 1.1.5`, `tempfile 3.27.0`, `winapi-util 0.1.11`;
  - `syn 3.0.6` → `syn 2.0.119`: `cssparser-macros 0.7.1`.
  Eleven `windows-sys` edges and one `syn` edge (inputs#I3 item 1's "twelve `windows-sys`" is eleven, as research.md recorded).
- **one edge added by `9eeebb302`:** `"mimalloc"` under `wpt 0.1.0` (the WPT runner's direct `mimalloc = "0.1.48"`, as upstream wrote it).

## mimalloc — the hand review re-read
Both crates were already in the lock (the browser's optional `mimalloc`); `9eeebb302` makes it the browser's default and adds the runner as a dependent. Read from the registry sources (`~/.cargo/registry/src/*/`):
- `mimalloc 0.1.52` — licence MIT · repository `https://github.com/purpleprotocol/mimalloc_rust` · `build = false` · no proc-macro · one dependency `libmimalloc-sys` · direct dependents now `browser 0.0.0`, `wpt 0.1.0`.
- `libmimalloc-sys 0.1.49` — licence MIT · repository `https://github.com/purpleprotocol/mimalloc_rust/tree/master/libmimalloc-sys` · `build = "build.rs"`, `links = "mimalloc"`, builds the vendored C in `c_src/` through its one dependency `cc` · no proc-macro · one dependent `mimalloc 0.1.52`.
Both reach only upstream-only binaries (`apps/browser`, the WPT runner); no escher crate depends on them.

## Each standing `deny.toml` ignore, re-read on the take's graph
Every crate an ignore names is still in the lock, so each ignore is still reached: RUSTSEC-2026-0192 (`ttf-parser 0.25.1`) · RUSTSEC-2024-0436 (`paste 1.0.15`) · RUSTSEC-2026-0367/0368/0369 (`zip 2.4.2`) · RUSTSEC-2026-0370 (`zvariant 5.15.0`) · RUSTSEC-2026-0371 (`zbus_names 4.3.4`) · RUSTSEC-2026-0372 (`zvariant_derive 5.15.0`). The yanked `clipboard-win 5.4.1` is still locked (a warning). The audit's own reading, gate entry 25 (`bash .github/scripts/ci-leg.sh audit`, this run's gate trail), is exit 0 with `advisories ok, licenses ok`. RUSTSEC-2026-0386 is cleared, with no ignore added for it. The log prints no `advisory-not-detected` warning, so every standing ignore is still encountered. Its two warnings are `warning[yanked]` on `clipboard-win 5.4.1` (through `arboard 3.6.1` ← blitz-shell) and `warning[license-not-encountered]` on the `NCSA` allowance at `deny.toml:46`. Neither fails the leg.

## Left out, and why
- `06ecec886` (Stylo 0.23-pre as a git dependency) with its two dependants `3954ecb85` and `1c5265a8b`, whose prefs (`layout.css.text-indent-keywords.enabled`, `layout.css.word-break.break-word.enabled`) and trait methods (`set_may_have_animations`, `has_active_animations_or_transitions`) stylo 0.22.0 lacks (research.md §The trial takes).
- `b8f4ba6fd` and `9babd796a` (git Taffy → `802e497b…`), with CARRY 1's four older Taffy-main commits `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6` — each puts a git Taffy in the root manifest.
- `286874cc1` (Dioxus 0.8.0-alpha.3), against whose vdom escher's stable-id derivation fails to build (10 lib + 15 test errors, research.md; inputs#I3 item 3).
The founder's word on the shape: «Десять коммитов (Рекомендую)» (inputs#I4).
