proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: '## Project Intent'
    change: 'In the Upstream sync line, «upstream `main` last read at `ebacd68bd`, twenty-four of its commits taken by» becomes «upstream `main` last read at `e42ee1f25`, thirty-six of the 57 commits past the merge base taken by», and after «`efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors)» add «, then twelve together (`45c333f16`), listed in the record below»; the merge base and the three pins stand.'
    sidecar: 'architecture §Project Intent, Upstream sync line: upstream read moves `ebacd68bd` to `e42ee1f25`, twenty-four to thirty-six of 57 taken, merge base `23b9b26a9` unmoved.'
    rationale: 'Report Counts, sync-line, names project-intent.md:9 @c0, @c234 and @c246 changed (read `ebacd68bd` to `e42ee1f25`, twenty-four to thirty-six of 57); Expected amendments carries the line; the pins stand under Dependencies, so D-arch-decisions is not violated.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: '## Project Intent'
    change: 'In the Upstream sync line, «sixteen left out, each with its measured reason, among them» becomes «twenty-one left out, each with its measured reason, among them»; the rest of the clause stands.'
    sidecar: 'architecture §Project Intent, Upstream sync line: left out moves sixteen to twenty-one at `e42ee1f25`.'
    rationale: 'Report Counts, left-out, and the over-2 000 listing name project-intent.md:9 @c1802 changed; 57 past the merge base less 36 taken is the 21 of inputs#I6 and Cross-project claims.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:61
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 11
    section: '## Project Intent'
    change: 'After line 11 add its own `<!-- detail: values · intent -->` block holding one record: - **The take at `e42ee1f25` (2026-11-01):** upstream `main` read at `e42ee1f25`, merge base `23b9b26a9` unmoved, 57 commits past it, 36 taken, 21 left out; twelve more taken together by no-commit cherry-pick (`45c333f16`, with its twelve `(cherry picked from commit …)` trailers): `fe33b778b` (#1276, restyle narrowing) · `8b1a0a3ce` (#1312, a table box rebuilt in `compute_layout_damage`) · `dee854c76` (#1317, `<picture>` no longer `display: contents`) · `f6edb9701` (#1318, a failed stylesheet stops blocking render) · `6fca6308d` (#1316, `mix-blend-mode` and `isolation` paint) · `7a3dd8918` (#1310, Stylo prefs before the UA sheet) · `91ef65e21` (#1314, `URLSearchParams`) · `a26c6ddeb` (#1322, a `CustomWidget` replays existing attributes) · `f76777766` (#1319, inline decorations) · `6ef4b532d` (#1305, a removed element skipped by mounted events, its test left out) · `ae774d7f4` (#1339, sub-pixel computed sizes) · `e42ee1f25` (#1342, culling bounds, a port); the five left out beyond the sixteen, each with its measured reason: `22f0cd000` (`AGENTS.md`, which no build reads and whose instructions contradict the fork) · `ac6f9f582` (`background-clip: text`, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22) · `e5929f9e2` · `0674e8efa` · `784a567e4`; Stylo stays "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3" and winit "=0.31.0-beta.3" (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md)'
    sidecar: 'architecture §Project Intent: new record of the 2026-11-01 take, its twelve commits, the five newly left out and the pins that stand.'
    rationale: 'Line 9 enumerates every commit taken and is already over the 2 000-char entity limit, so the report twelve and five go in a record of their own (Symbols / APIs, inputs#I6 row one, Reverted facts, Dependencies).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:78
note: 'Line 9 stays over 2 000 chars as it stood, and its closing as-measured pointer and «(target: next)» are not proposed: the first is the citation sweep, the second follows the P5 move of residuals.md:35-36; architecture.md:291 owes the same sync-line move outside this file; D-arch-resources and D-arch-decisions found no violation here.'
