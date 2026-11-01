proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 39
    section: '## Established Decisions'
    change: 'After the Stylo prefs clause, add: `pub(crate) fn init_stylo_prefs`, behind a `Once`, sets them, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 pref names, upstream''s 0.23 `set_pref!` names not written (packages/blitz-dom/src/document.rs:87-102).'
    sidecar: 'Document defaults: the Stylo prefs are set by `init_stylo_prefs` behind a `Once`, before the UA sheet, on the 0.22 names (`7a3dd8918`).'
    rationale: 'The report states `init_stylo_prefs` as the one setter, called from both sites, and the 0.22 names kept over upstream''s (report.md:16, report.md:84).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 45
    section: '## Established Decisions'
    change: 'After the render-blocking clause, add: a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-dom/src/net.rs:144-146; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91). After the `<link>` clause, add: a bad `href` starts no load, `resolve_url` giving way to `doc.url.resolve_relative` (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107).'
    sidecar: 'Resource loading: a head stylesheet still blocks while pending, and a failed, empty or non-CSS one no longer stays pending; a bad `href` starts no load (`f6edb9701`).'
    rationale: 'The row''s render-blocking and `<link>` rules each take a limit through `f6edb9701` (report.md:14).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 57
    section: '## Established Decisions'
    change: 'A new continuation line following this one, the row''s line being too long to take it: ; `background-clip: text` is unsupported, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22, so upstream `ac6f9f582` was tried and left out (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).'
    sidecar: 'Unsupported features: `background-clip: text`, gecko-only on Stylo 0.22, `ac6f9f582` tried and left out.'
    rationale: 'The report records the trial and its reason among its negative API facts (report.md:84).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:84
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 67
    section: '## Established Decisions'
    change: 'The `ebacd68bd` record stays as written. Directly after it add: at upstream `main` `e42ee1f25` this take''s twelve reach `node/element.rs`, `layout/construct.rs` and `mutator.rs` of the range model''s surface (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md); and break the line after that sentence, as the group wraps, since the line stands over 2 000 chars.'
    sidecar: 'Range input merge surface: this take''s reach at `e42ee1f25` stands beside the `ebacd68bd` record.'
    rationale: 'The last take''s record is history and holds, and this take takes a new sentence beside it (report.md:35); the three files are in the take''s list and `node/range.rs`, `node/mod.rs`, `form.rs` and `events/*` are not (report.md:8).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:35
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 69
    section: '## Established Decisions'
    change: 'The `ebacd68bd` record stays as written. Directly after it add: at upstream `main` `e42ee1f25` this take''s twelve reach `node/element.rs` (`ElementData::set_custom_widget`''s replay) and again leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md); and break the line after that sentence, as the group wraps, to keep it under 2 000 chars.'
    sidecar: 'Keyboard activation merge surface: this take''s reach at `e42ee1f25` stands beside the `ebacd68bd` record.'
    rationale: 'The same history line (report.md:35); `a26c6ddeb` edits `node/element.rs` (report.md:18) and no `events/` file is among the 33 (report.md:8).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: '## Established Decisions'
    change: 'The merge-surface sentence''s owner now reads: its owner «Upstream sync ahead of determinism and exploration», in place of Epoch 3''s «Upstream sync ahead of the UI graph»; after the re-read clause add: re-read again at this take, each row given a named outcome and nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md).'
    sidecar: 'The seven''s merge surface: owner re-pointed to the next sync''s entry, the sync that owned it having run.'
    rationale: 'The report re-points this owner row (established-decisions.md:75, report.md:62) to the card''s named entry (report.md:79), `.andromeda/residuals.md:35` and `:36` moving with it (report.md:131), and records the re-read (report.md:148).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: '## Established Decisions'
    change: 'The closing clause now reads: its `hit_inner` hunk is a textual conflict with the non-invertible answer for the next sync, `0980cce5f` being left out at `e42ee1f25`, their merge surface a line owned by «Upstream sync ahead of determinism and exploration». The rest of the row stands.'
    sidecar: 'The eight''s merge surface: owner re-pointed; `0980cce5f` still left out, so its `hit_inner` conflict passes to the next sync.'
    rationale: 'An owner row the report re-points (established-decisions.md:76, report.md:62); the left-out delta names `0980cce5f` and goes to that entry (report.md:78).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:78
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: '## Established Decisions'
    change: 'A new row following this one: - The panic''s fix stands beside those eight, a direct engine change for every document, ratified: `BaseDocument::deallocate_anonymous_block` takes the block''s `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so a key press on a control whose anonymous block was dropped no longer panics, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts, `DocumentMutator::flush` still reading `is_inert()` alone. Flagged upstreamable and nothing is sent; its merge surface is owned by «Upstream sync ahead of determinism and exploration» (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).'
    sidecar: 'The panic''s fix, a direct engine change for every document, on the founder''s answer «Убрать падение, фокус остаётся (Рекомендую)» (inputs#I3), for the answer card at P2''s halt.'
    rationale: 'The report states the fix and its limit (report.md:23), carries it as an arch amendment (report.md:92), names the merge surface''s owner (report.md:79), and gives the body bare `ratified`, his words in the sidecar (report.md:130).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 77
    section: '## Established Decisions'
    change: 'The closing owner clause now reads: whose owner is «Upstream sync ahead of determinism and exploration», in place of Epoch 3''s «Upstream sync ahead of the UI graph». The rest of the row stands.'
    sidecar: 'The clock''s merge surface: owner re-pointed to the next sync''s entry.'
    rationale: 'An owner row the report re-points (established-decisions.md:77, report.md:62), to the card''s named entry (report.md:79).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: '## Established Decisions'
    change: 'The closing owner clause now reads: whose owner is «Upstream sync ahead of determinism and exploration», in place of Epoch 3''s «Upstream sync ahead of the UI graph». The rest of the row stands.'
    sidecar: 'The select model''s merge surface: owner re-pointed to the next sync''s entry.'
    rationale: 'An owner row the report re-points (established-decisions.md:79, report.md:62), to the card''s named entry (report.md:79).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 85
    section: '## Established Decisions'
    change: 'The row now reads: its line owned by «Upstream sync ahead of determinism and exploration», in place of Epoch 3''s «Upstream sync ahead of the UI graph»; the sixteen files stand.'
    sidecar: 'The select list''s merge surface: owner re-pointed to the next sync''s entry.'
    rationale: 'An owner row the report re-points (established-decisions.md:85, report.md:62), to the card''s named entry (report.md:79).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/established-decisions.md
    near: 88
    section: '## Established Decisions'
    change: 'A new row following this one: - Twelve more behaviour changes for every document are upstream''s and are not counted among escher''s direct engine changes above, each ratified for its take, by its commit, under the playbook pattern for an engine behaviour change for every document: `fe33b778b` `8b1a0a3ce` `dee854c76` `f6edb9701` `6fca6308d` `7a3dd8918` `91ef65e21` `a26c6ddeb` `f76777766` `6ef4b532d` `ae774d7f4` `e42ee1f25`; no boundary is widened by the take, `URLSearchParams` a realization on the registered script crossing (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).'
    sidecar: 'The twelve upstream behaviour changes, on the founder''s answer «Двенадцать коммитов (Рекомендую)» (inputs#I3), for the answer card at P2''s halt.'
    rationale: 'The report lists the twelve (report.md:10) and carries their ratification as an arch amendment, with no boundary widening read and `URLSearchParams` a realization (report.md:92), and gives the body bare `ratified`, his words in the sidecar (report.md:130).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:92
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 88
    section: '## Established Decisions'
    change: 'A new row following this one: - `AGENTS.md` from upstream `22f0cd000` stays out of the tree: no build reads it and its instructions contradict this fork''s (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).'
    sidecar: '`AGENTS.md` stays out of the tree, recorded beside the upstream takes.'
    rationale: 'The report records it among its negative API facts and its acceptance reads `git ls-files AGENTS.md` empty (report.md:84, report.md:138).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:84
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 101
    section: '## Established Decisions'
    change: 'At the end of the row add: ; `<picture>` is no longer `display: contents`, upstream''s `picture` rule leaving `packages/blitz-dom/assets/default.css`, every select rule staying (packages/blitz-dom/src/document.rs:4061-4094).'
    sidecar: 'Layout: `<picture>` no longer `display: contents` (`dee854c76`).'
    rationale: 'The report states the rule''s removal and its test (report.md:13); line 101 and not line 100 takes it, line 100 standing too long to.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 101
    section: '## Established Decisions'
    change: 'After the `ALL_DAMAGE` clause add: and `compute_layout_damage`''s `table_needs_rebuild` takes `ALL_DAMAGE` when a table box''s position, padding or border changes (packages/blitz-dom/src/layout/damage.rs:442-458).'
    sidecar: 'Layout damage: a table box''s position, padding or border change takes `ALL_DAMAGE` (`8b1a0a3ce`).'
    rationale: 'The report states the new arm (report.md:12), which the row''s damage clause must now hold.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:12
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 60
    section: '## Established Decisions'
    change: 'The closing clause now reads: the merge surface of those eight is a line owned by Upstream sync ahead of determinism and exploration, `hit_inner` a textual conflict with upstream `0980cce5f`. The rest of the line stands.'
    sidecar: 'Body: the eight''s merge surface re-pointed with its detail row.'
    rationale: 'The broad line of established-decisions.md:76, whose owner moves (report.md:62, report.md:79); `0980cce5f` stays left out (report.md:78).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:79
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 62
    section: '## Established Decisions'
    change: 'The closing clause now reads: a line owned by Upstream sync ahead of determinism and exploration. The rest of the line stands.'
    sidecar: 'Body: the select list''s merge surface re-pointed with its detail row.'
    rationale: 'An owner row the report re-points (architecture.md:62, report.md:62), to the card''s named entry (report.md:79).'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
    dependent-of: D-cascade
note: 'F10 is left to the new-detail pass, its home [Painting] not in this listing; no body line is proposed for F1 or F2; F1 and F2 are `escalate` for P2''s answer card (report.md:130); entity 10.1 already stands over the 8 KB entity bound and line 63 over 2 000 chars, neither addressed here.'
