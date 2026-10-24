# Cascade compare — architecture · phase · 2026-10-24-a11y-re-read
E items 119 · held 36 · lost at classify 6 · lost at strong 3 · beside a placed fact - · cascade only 11 · body 29 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### conventions#9.2 · lost at classify · .andromeda/architecture-detail/conventions.md:61
- (.andromeda/architecture-detail/conventions.md:61) A fixture page a new a11y check reads must go in `fixtures/mod.rs` only when more than one check reads it, else stay in its check's file.
  > A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; what it holds is test-plan §3's (Crate-local test helpers), and a page with one reader stays in its check's file (tests/blitz-tests/tests/fixtures/mod.rs:17-25; tests/blitz-tests/tests/fixtures/mod.rs:87-130).

### cross-cutting-patterns#3.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:21
- (.andromeda/architecture-detail/cross-cutting-patterns.md:21) The Tab-order re-read must leave the redraw wiring as stated: a focus set or cleared requests a redraw, and the Tab key requests none of its own. The chunk is a re-read and changes no engine behaviour.
  > redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover, device changes and a focus set or cleared, the Tab key asking for none of its own

### design-philosophy#3.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:34
- (.andromeda/architecture-detail/design-philosophy.md:34) The a11y checks the leg is widened to witness spec-cited behaviour (WAI-ARIA tree exclusion, HTML-AAM role mapping, HTML implicit form submission on Enter), and an asserted row states what that spec requires, not only what the engine does today.
  > WAI-ARIA tree exclusion (packages/blitz-dom/src/accessibility.rs:165), HTML-AAM role mapping (packages/blitz-dom/src/accessibility.rs:441), CSS Transforms current transformation matrix (packages/blitz-dom/src/stylo_to_kurbo.rs:11-24), HTML dimension-value parsing (packages/blitz-dom/src/stylo.rs:921-927), Pointer Events `touch-action` intersection (packages/blitz-dom/src/events/pointer.rs:158-166), HTML implicit form submission (packages/blitz-dom/src/events/keyboard.rs:357)

### established-decisions#9.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:57
- (.andromeda/architecture-detail/established-decisions.md:57) The contrast re-read's colour states can reach `:focus-within` but never `:focus-visible`. A focus colour state must be read through `FOCUS_WITHIN` or focus, not `:focus-visible`.
  > `:focus-visible`, `:valid`, `:invalid`, `:required`, `:target` always match false (packages/blitz-dom/src/stylo.rs:460-508), while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state

### established-decisions#15.2 · lost at strong · .andromeda/architecture-detail/established-decisions.md:105
- (.andromeda/architecture-detail/established-decisions.md:105) The `:focus-within` change came in during 0.2.0 and marks no changed-set entry. `stand_accessibility_ids`' refresh reading must still see exactly the two nodes that changed focus.
  > The `FOCUS_WITHIN` writes go through `snapshot_node_and` and mark no changed-set entry, so a focus move still marks exactly the node that lost focus and the node that gained it

### established-decisions#6.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:39
- (.andromeda/architecture-detail/established-decisions.md:39) `stand_contrast`'s re-asserted ratios hold only in the headless Light colour scheme.
  > headless documents and the WPT viewport use `ColorScheme::Light`

### project-intent#1.0 · lost at classify · .andromeda/architecture-detail/project-intent.md:9
- (.andromeda/architecture-detail/project-intent.md:9) The re-read must cover the ten upstream commits cherry-picked together at `34f6f051b` as 0.2.0 engine changes, among them the hit test's skip of dropped nodes, reading the stand's Tab order, activation and contrast against the engine as these commits leave it.
  > then ten together (`34f6f051b`): `28f848d3b` (#1233, the inline layout module split) · `9eeebb302` (#1239, mimalloc in the browser and the WPT runner) · `e4ca5b9b5` (#1240, class atoms on `ElementData`) · `46d7cf993` (#1238, out-of-flow-ness on `Node`) · `fb297a38c` (#1243, `Parallel` style traversals serialised; `examples/multi_doc.rs`) · `50bf2f235` (#1246, a replaced element's natural height through `aspect-ratio`) · `f8723f495` (#1247, WPT runner) · `21edc92d9` (#1258) · `f102663b8` (#1244, dropped nodes skipped in hit testing) · `f9cbb2756` (#1264, the flake's toolchain)

### project-intent#1.0 · lost at classify · .andromeda/architecture-detail/project-intent.md:9
- (.andromeda/architecture-detail/project-intent.md:9) The earlier cherry-picks are also upstream engine code inside 0.2.0, and `:focus-within` is a focus-state style change, so the contrast re-read's colour states must be read with it in place.
  > `fb8d6c15e` (the default cursor over no box, upstream #1220) alone (`275950dc0`), then `38ca4e777` (`:focus-within`, upstream #1226) and `17de049b1` (the WPT pin, upstream #1228) together (`79603d1ef`)

### project-intent#1.0 · lost at classify · .andromeda/architecture-detail/project-intent.md:9
- (.andromeda/architecture-detail/project-intent.md:9) The re-read takes the stack as pinned: it must not pull in git Stylo 0.23-pre, the Taffy-main commits or Dioxus 0.8.0-alpha.3, which wait for the next version under a residuals line, and it must add no git dependency.
  > wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency

### project-intent#1.0 · lost at classify · .andromeda/architecture-detail/project-intent.md:9
- (.andromeda/architecture-detail/project-intent.md:9) The upstream merge base stays where it is: a re-read takes no new upstream commit and moves no sync record.
  > Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a (merge commit 3eda07bc, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync's merge base, unmoved

### project-intent#5.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:33
- (.andromeda/architecture-detail/project-intent.md:33) The checks added to the `a11y` leg must be blitz-tests integration targets, inside the project's stated validation sources.
  > blitz-tests is "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3)

### stack-and-technologies#1.10 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:36
- (.andromeda/architecture-detail/stack-and-technologies.md:36) The accessibility checks the a11y leg is widened to must read the tree through the workspace's AccessKit as pinned, with no AccessKit or platform adapter version moved and no adapter dependency added; the platform adapter stays out of scope.
  > accesskit "0.25" (Cargo.toml:139-146); accesskit_xplat with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.24.0, accesskit_android 0.9.0 per target (packages/accesskit_xplat/Cargo.toml:16-35)

## Held

- conventions#1.1
- conventions#8.1
- cross-cutting-patterns#1.1
- cross-cutting-patterns#4.1
- cross-cutting-patterns#5.1
- design-philosophy#4.1
- design-philosophy#5.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#14.1
- established-decisions#15.1
- established-decisions#15.3
- established-decisions#5.1
- existing-scopes#1.31
- existing-scopes#1.32
- existing-scopes#2.2
- inherited-defaults#2.1
- occupied-resources#3.1
- occupied-resources#9.1
- stack-and-technologies#1.20
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- stack-and-technologies#1.9
- standard-contracts#13.1
- standard-contracts#14.1
- standard-contracts#15.1
- standard-contracts#2.2
- standard-contracts#21.1
- standard-contracts#22.2
- standard-contracts#26.1
- standard-contracts#3.1
- standard-contracts#4.1
- standard-contracts#9.1
- standard-contracts#9.2

## Cascade only

- conventions#3.1
- existing-scopes#1.13
- existing-scopes#1.14
- existing-scopes#1.23
- existing-scopes#1.24
- existing-scopes#1.4
- existing-scopes#1.7
- existing-scopes#1.8
- existing-scopes#1.9
- standard-contracts#1.1
- standard-contracts#2.1
