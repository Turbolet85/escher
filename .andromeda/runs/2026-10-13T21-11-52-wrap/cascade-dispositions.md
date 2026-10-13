# Cascade dispositions — 2026-10-13-range-input-interaction

**The search.** Ten patterns from every amendment of the pass (`cascade-patterns.toml`), run by `cascade.py sweep` after the last body edit. Every pattern's control fired on the pre-pass masters. Baseline: 97ce760d, the pre-CI parent.
- `rangeretired`: the retired claim that no verb moves a range input — its own phrasings ("measured reading", "no verb can move", "reads its boot value", "cannot be changed", "no range").
- `range`: every range-input mention.
- `valueclause`: the retired snapshot `value` clause.
- `changedsites`: the changed set's writing sites.
- `variants`: the `SpecialElementData` list.
- `engchange`: the direct-engine-change counting.
- `counts`: each moved count's old figure.
- `nodemods`: the node module list.
- `clickfocus`: the claim that a click on a range input does not focus it, by its verb.
- `silent`: the census wording.

Sections read beside the rows: the four amended §Established Decisions lines, §Standard Contracts (Events, Nodes and widgets, Dioxus DOM bridge), §Cross-cutting, §Existing Scopes, security-plan §Input Validation, test-plan §1/§2/§4/§9, obs-plan §3, a11y-plan §5/§7, and the §3 key file `session-lifecycle.md:11` (`stand_act_range` 1, true).

**Counts, copied from the listing:** total (10 patterns) · 77 rows over 18 files · per class · new 19/4 · standing 32/6 · leaf 25/11 · curation 1/1 · base 0/0.

## Master rows
- **new (19 rows, 4 files)** — this pass's own text. It was re-read for an intra-line duplicate of a retired claim, and none stands:
  - architecture.md:87, :89, :123, :126, :197, :245, :263;
  - security-plan.md:119, :126;
  - test-plan.md:150;
  - a11y-plan.md:161, :165, :212, :279.
- **standing, edited** — amended lines whose other hits were read:
  - architecture.md:136 @c26970: the role list naming Slider — true, no change.
  - architecture.md:263 ×5 @c2061,2258,2475,7504,7642: the new `range_input_interaction` clause and the restated `stand_act_range` clause — amended.
  - architecture.md:197 changedsites ×2: "four kinds of site" and "the last two wherever the node is" — amended, read whole.
  - architecture.md:126 variants ×2: amended.
  - architecture.md:87 engchange ×2: @c2409 "a third direct engine change" stands (the accessibility builder's) and @c3433 is the new fifth — amended.
  - test-plan.md:24 ×2: amended.
  - test-plan.md:26: ×9 range and ×3 clickfocus. @c4626 "the range input reading the app's duration" is the boot reading, true, no change; the rest is this pass's `stand_act_ids`, `stand_act_range` and engine-check text — amended.
  - test-plan.md:330 counts ×2: "180 result lines, 900 passed" is the stand-contrast re-count, kept as history beside the new 181 · 915 re-count — no change.
  - security-plan.md:126 (range row) engchange: amended.
  - a11y-plan.md:148, :160, :279 @c4502: the disabled-presence keying, the checkbox click and a textarea fixture's focus — true, no change.
  - obs-plan.md:70 silent ×8: the census with `range_input_interaction` added — amended.
- **standing, not edited — each read, true, no change:**
  - layout-templates.md:10 ×3: `timer-duration`'s label `for`, markup unchanged.
  - a11y-plan.md:107: range→Slider.
  - a11y-plan.md:227 ×2: `timer-duration` no-text in the contrast record.
  - architecture.md:95: the password paint "beside those under [Scrolling and input]".
  - security-plan.md:9: `SpecialElementData::SubDocument`.
  - security-plan.md:127: the password row's "engine change for every document".
  - security-plan.md:213: a clock "engine change for every document".
  - architecture.md:133 @c20631 and test-plan.md:181 @c3795: a button in a scrolled-out row's place, no element focused after the click — `stand_act_obstructed`, not a range input.
  - a11y-plan.md:158 ×2: a plain button leaves no element focused.
  - a11y-plan.md:211 ×2: the Tab focus check.
  - the `silent` rows architecture.md:101, :133 ×2, :193; security-plan.md:81, :82; obs-plan.md:69 ×2, :106: other crates' census sentences.
- `nodemods`: 0 rows (control fired). The old list is gone from architecture.md:245.

## Curation and base rows
- `.claude/docs/session-learnings.md:155` clickfocus (curation): disabled keying on presence for click targeting — true, no extension owed. It goes to P3 only as read.
- base 0.

## Leaf rows (25 rows, 11 files) → cascade step 3's set
These leaves are added to `cascade.py leaves`: CLAUDE.md, .claude/docs/services/escher-driver.md, .claude/docs/tests-summary.md, .claude/docs/services/seven_guis.md, .claude/docs/services/dioxus-native-dom.md, .claude/docs/services/blitz-dom.md, .claude/docs/services/blitz-paint.md, .claude/rules/a11y.md, .claude/docs/a11y-summary.md, .claude/rules/observability.md, .claude/docs/obs-summary.md — re-derived by the comparers, never patched from this listing.

## Leaves
Pass mode: `cascade.py leaves` over 10 groups and 25 leaves, with the 11 swept leaves added. One parallel batch of 10 comparers ran inside one snapshot pair (before `tree 6e9d74c6…`, after `tree e95eb310…`, index `b9f087a1…` and head `dde6b170…` unchanged). `diff-tree` showed exactly ten `A leaf-{group}.md` lines. `leaves --apply` total: blocks 24 · applied 24.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 4 | 4 | 0 | 0 |
| architecture-2 | pass | 2 | 2 (the driver key names `arrow-right` … `end` checked at packages/escher-driver/src/command.rs:85-90) | 0 | 0 |
| architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan (CLAUDE.md) | pass | 5 | 5 | 0 | 0 |
| architecture+test-plan | pass | 1 | 1 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 3 | 3 | 0 | 0 |
| design-system | pass | 0 | 0 | 0 | 0 |
| test-plan | pass | 3 | 3 | 0 | 0 |
| obs-plan | pass | 1 | 1 | 0 | 0 |
| a11y-plan | pass | 5 | 5 | 0 | 0 |

`unmapped .claude/rules/host-linux.md (host)`: the host's own shell mechanics. No master states them, so no comparer runs and no step-3 edit is made.
