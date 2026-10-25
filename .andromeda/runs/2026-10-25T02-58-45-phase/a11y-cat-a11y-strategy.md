# a11y · A11y Strategy — category read

## Carried
- (.andromeda/a11y-plan.md:5) The statement's a11y reach must name the tier-0 WCAG target exactly as the plan states it, no wider.
  > **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
- (.andromeda/a11y-plan.md:9) The statement's a11y reach must say that no stand binary builds a platform adapter, so no assistive technology is given a tree, and that this is owed work.
  > no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary — the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next).
- (.andromeda/a11y-plan.md:27) The statement must describe the tree's refresh on change as proven headlessly, with its windowed witness at the adapter stated as owed and not claimed.
  > proven headlessly on the stand in both layout modes, its windowed witness at the platform adapter owed — owned by a `.andromeda/residuals.md` line (target: next).
- (.andromeda/a11y-plan-detail/a11y-strategy.md:9) The statement's proven half must match what the plan records as measured (both halves headless, both layout modes, the tree the shell hands the adapter), and its unmeasured half must be stated as owed by the residuals line, never as built.
  > since 2026-10-14-stand-a11y-assertions both halves are proven headlessly on the stand, in both layout modes: on Counter, after a drained changed set, each Tab of the whole `tab_order` leaves `has_changes()` true and the focus of the tree read through `&dyn Document` — the tree the shell hands the adapter — reads the new stop
- (.andromeda/a11y-plan-detail/a11y-strategy.md:9) The windowed witness at the platform adapter is an owed item the statement must list among its stated limits, with its residuals owner.
  > its windowed witness, at the platform adapter, is still owed — the stand builds no adapter, kept so on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14, by question dialog in the overseer session, relayed by the overseer as his chosen option label) — and is owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/a11y-plan.md:35) The statement must state as a limit that the accessibility tree never states a native checkbox or radio checked, owned by the residuals.
  > nothing else writes `toggled`, so the tree never states a native checkbox or radio checked — that fix and the windowed AT witness owned by `.andromeda/residuals.md` lines (target: next).
- (.andromeda/a11y-plan-detail/a11y-strategy.md:20) The tree's missing native checked state is a measured absence the statement must list among its limits, and it must keep it apart from the snapshot's `checked`, which does read the element.
  > the builder writes no `toggled` from a native checkbox's or radio's own checked state, so the tree the shell hands the adapter never states it checked while the snapshot's `checked` reads the element — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next);
- (.andromeda/a11y-plan-detail/a11y-strategy.md:21) The statement must not claim that an assistive technology reads the three ARIA states. That reading is unmeasured, and its owner is the residuals line.
  > what an assistive technology reads of them is not measured — no stand binary builds a platform adapter and the dev host's AT-SPI bus reads inactive — and the windowed witness, at an active assistive technology, is owned by a `.andromeda/residuals.md` line (target: next)
- (.andromeda/a11y-plan.md:37) The statement must give upstream's six reads and say that their descriptions and new closed values reach no assistive technology that has been measured.
  > the description and the new closed values leave only to the platform adapter, which no stand binary builds, so what an assistive technology reads of them is not measured.
- (.andromeda/a11y-plan-detail/a11y-strategy.md:22) The statement's limits and its MCP carry-onward section must agree with this: the snapshot reads no description, `aria-disabled` moves no snapshot reading, and the description goes only to the adapter, so no driver carrier hands it to an MCP client.
  > the snapshot reads no description and its `enabled` reads the `disabled` attribute, so `aria-disabled` moves no snapshot reading; no stand file states any of the six; the description and the three new closed values leave only to the platform adapter, which no stand binary builds (§1), so what an assistive technology reads of them is not measured;
- (.andromeda/a11y-plan-detail/a11y-strategy.md:31) If the statement's a11y reach names platform adapters, it must state these per-platform adapter gaps and not claim full adapter parity.
  > Android adapter `set_focus` and `set_window_bounds` are empty; Windows `set_focus` and macOS `set_window_bounds` are no-ops
- (.andromeda/a11y-plan-detail/a11y-strategy.md:39) The statement must say that the snapshot, its text form, its parts, its diff and the actionable-key check sit behind the `accessibility` feature, so a consumer knows those capabilities need it.
  > dioxus-native-dom's feature also gates its own `accessibility_tree` override, its snapshot model (`DioxusDocument::snapshot`), that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all five of which its crate doc names
- (.andromeda/a11y-plan.md:43) The statement must not claim a stated accessibility strategy, such as POUR coverage depth, beyond the tier-0 target. The plan records that strategy as not yet measured.
  > NOT YET MEASURED — a stated accessibility strategy (POUR coverage depth, agent-runnable invariants, naming conventions): no slice states one
- (.andromeda/a11y-plan.md:235) The statement's «`a11y` leg of fourteen files» must match the plan's count and its gate standing: the slow tier with no required status check.
  > ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over fourteen files,

## Bindings
- **Windowed adapter witness ↔ `.andromeda/residuals.md` and the scope's a11y reach**: the statement's «what is not measured» must name the residuals line (target: next) that owns the adapter and the AT witness, and must not move its target (.andromeda/a11y-plan-detail/a11y-strategy.md:9) (.andromeda/a11y-plan.md:9)
- **Tree's native checked absence ↔ screen-reader-support's snapshot `checked`**: the tree reads no native checked state while the snapshot's `checked` reads the element. The statement must keep the two readings apart (.andromeda/a11y-plan-detail/a11y-strategy.md:20)
- **Description adapter-only ↔ security-plan §Surfaces (the fourth content class) and the scope's MCP carry-onward**: the description crosses to the platform adapter alone, so the statement's list of what an MCP client carries must leave it out (.andromeda/a11y-plan-detail/a11y-strategy.md:22)
- **`accessibility` feature gate ↔ CLAUDE.md / architecture (the default feature, the snapshot surfaces)**: the statement's capability list must show that the snapshot surfaces depend on the feature that is on by default (.andromeda/a11y-plan-detail/a11y-strategy.md:39)
- **`a11y` leg of fourteen files ↔ test-plan §10 (stand-repeat)**: the count and the slow-tier, non-required standing come from §9, and the eight repeated files come from the stand-repeat leg (.andromeda/a11y-plan.md:235)
