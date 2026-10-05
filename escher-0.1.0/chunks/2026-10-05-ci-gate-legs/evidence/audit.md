# Dependency audit — the firing set under the committed `deny.toml` (plan step 5)

Tool: cargo-deny 0.20.2 (dev host), advisory database `https://github.com/RustSec/advisory-db` fetched
at the run (2026-10-05T22:53Z). Command: `bash .github/scripts/ci-leg.sh audit` →
`cargo deny --locked check advisories`, config `deny.toml` (`[graph]`: the six ci.yml platforms,
`all-features = true`).

## Firing set with `ignore = []` (rustls already at 0.23.45)

`advisories FAILED`, exit 1 — ONE finding:

- `error[unmaintained]` **RUSTSEC-2026-0192** `ttf-parser 0.25.1` — `Solution: No safe upgrade is available!`
  Chain: ttf-parser ← owned_ttf_parser 0.25.1 ← ab_glyph 0.2.32 ← sctk-adwaita 0.12.0 ← winit-wayland
  0.31.0-beta.3 ← winit 0.31.0-beta.3 ← blitz-shell / dioxus-native. winit is an exact coupled pin
  (CLAUDE.md "winit exact beta"), so no fix is reachable inside the pins → per-ID ignore with its reason
  in `deny.toml`.

With that one ignore: `advisories ok`, exit 0.

## The other three lockfile advisories

| advisory | crate | how it leaves the audit |
|---|---|---|
| RUSTSEC-2026-0285 (vulnerability) | rustls 0.23.43 | FIXED: `cargo update -p rustls@0.23.43 --precise 0.23.45` (plan step 1); the lockfile delta is the rustls version line alone. cargo-deny also logs `filtered rustls 0.23.45` — no build graph reaches it |
| RUSTSEC-2024-0436 (unmaintained) | paste 1.0.15 | not fired: cargo-deny logs `filtered paste 1.0.15` (and `filtered rav1e 0.8.1`, `filtered ravif 0.13.0`) |
| RUSTSEC-2026-0186 (unsound) | memmap2 0.5.10 | not fired: cargo-deny logs `filtered memmap2 0.5.10` (and `filtered cacache 13.1.0`, `filtered http-cache 1.0.0-alpha.6`) |

**Observed reach gap (for the wrap's security-plan amendment, not fixed here).** paste and memmap2 ARE
in cargo's own graph for a target the config lists:
`cargo tree --workspace --all-features --target x86_64-unknown-linux-gnu -e normal,build -i memmap2@0.5.10`
→ memmap2 ← cacache 13.1.0 ← http-cache 1.0.0-alpha.6 ← blitz-net (its optional `cache` feature,
`packages/blitz-net/Cargo.toml:17`); `-i paste@1.0.15` → paste ← rav1e 0.8.1 ← ravif 0.13.0 ← image
0.25.10. cargo-deny 0.20.2's graph prunes `http-cache` and `ravif` even with `all-features = true`
(`-L debug`: `filtered http-cache 1.0.0-alpha.6`, `filtered ravif 0.13.0`), and passing `--all-features`
on the command line reads the same single finding. So the audit leg's reach is cargo-deny's feature
resolution, which is narrower than `cargo tree`'s on these two optional chains; a lockfile-wide scanner
(cargo-audit 0.22.2, research.md §Measurements) still reports paste and memmap2 as warnings. Neither is
ignored in `deny.toml` — an ignore for an advisory that does not fire would be dead config.
