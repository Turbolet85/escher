# Cascade dispositions — 2026-10-06-project-readme

Amendments this pass (architecture only): E1 §Conventions → Licensing exceptions (+ `stylo_taffy` triple licence);
E2 §Project Intent (+ **Front page** bullet: root README is escher's; upstream README changes resolve to escher's at a
sync). Both are ADDITIONS — no claim retired — so the sweep looks for sites that state a contrary or now-incomplete
claim: anything about README content/ownership, the licensing-exception list, the dual licence, MPL, and the Upstream
sync's merge practice.

Search: `cascade.py sweep` over the seven masters, `.andromeda/registries/**`, the curation homes, the judgment bases
and the leaves — patterns `readme` (fixed `README`), `lic-exceptions` (fixed `Licensing exceptions`), `dual-licence`
(`MIT OR Apache-2\.0|Apache-2\.0 OR MIT|dual[- ]licen`), `mpl` (`\bMPL\b`, case-sensitive — `-i` matches
"exaMPLe"), `upstream-sync` (`[Uu]pstream sync`); every control fired on the pre-pass masters (listing:
`cascade-sweep.txt`).

| row | disposition |
|---|---|
| architecture.md:136 readme standing | true claim sharing the token — rdme's runtime lookup of the nearest README.md (§Standard Contracts CLIs); no change |
| architecture.md:207 readme new ×4 | this pass's E2 text |
| security-plan.md:295 readme standing | true claim sharing the token — rdme exits when no README.md is found; no change |
| design-system.md:350 readme standing | true claim sharing the token — rdme's window title; no change |
| readme case-variants +33 | `readme` lower-case: the `apps/readme` crate path / `readme_application` identifiers — not the root page; no change |
| architecture.md:106 lic-exceptions standing (edited) | the amended line (E1); re-read — the list now names stylo_taffy once, no duplicate |
| architecture.md:45 dual-licence standing | true — the workspace package licence `MIT OR Apache-2.0` (Cargo.toml:36); stylo_taffy's own field is the exception E1 records; no change |
| architecture.md:106 dual-licence new ×2 | this pass's E1 text |
| .claude/docs/stack.md:7 dual-licence leaf | true — the workspace licence; no change |
| architecture.md:106 mpl standing (edited) ×3 | default.css MPL header (standing, true) + E1's MPL-2.0 — two subjects on one line, both true |
| architecture.md:202 upstream-sync standing | the merge-base record — true, unchanged by this chunk |
| architecture.md:207 upstream-sync new | this pass's E2 text |

Curation homes 0 rows, judgment bases 0 rows, registries 0 rows — for these patterns.

## Leaf re-derivation (cascade step 3)
- `.claude/docs/conventions.md` (§Conventions leaf, provenance "Extracted from architecture.md §Conventions"): recomputed against the amended §Conventions — the leaf has never distilled the Licensing-exceptions bullet (no `wasm_hello` / `wgpu_texture` / `accesskit_xplat` licence line in it before this pass), so the recompute yields no change.
- CLAUDE.md `GENERATED:setup:overview` / `:architecture` (from §Project Intent / §Design Philosophy): recomputed — the overview paragraph and key-directories list carry Project Intent's prose, not its bullet list (Product type / Crate purposes / Apps are not in it either); the Front page bullet adds no directory, module or warning → no change. `:warnings` / `:pointer-table` / `:modules`: neither amended section feeds them → no change.
- No plan amended → no specialist summary, rule file or warnings-block line re-derived.
