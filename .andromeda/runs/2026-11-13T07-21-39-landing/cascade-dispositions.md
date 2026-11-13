# Cascade dispositions — 2026-11-13T07-21-39-landing

## Leaves

The landing of rows proposal 2026-11-12T03-36-23 ran `cascade.py leaves --all`: the whole mode, fired once, 15 groups, 25 leaves, 723249 B, `unmapped 1` (`.claude/rules/host-linux.md`, the host leaf: a statement no master states, so no comparer reads it and step 3 makes no edit there). There was one comparer batch (`leaf`, 15 of 15 pass). Every finding's source row was read by its id (`rec.py get`), and every `tree` finding's path was tested in the tree. The counts below are copied from the tool's line `total · blocks 55 · applied 41 · held 14`.

Rule for the decisions: a `stale` or `tree` finding is applied where its source row or the tree confirms it. An `omission` is applied only where a reader of the leaf would otherwise act against the source. Otherwise the detail stays in its master, and the finding is declined (`held`).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan-1 | whole | 3 | 3 | 0 | 0 |
| a11y-plan-2 | whole | 2 | 2 | 0 | 0 |
| architecture-1 | whole | 5 | 5 | 0 | 0 |
| architecture-2 | whole | 3 | 3 | 0 | 0 |
| architecture+layout-templates | whole | 1 | 1 | 0 | 0 |
| architecture+obs-plan | whole | 1 | 1 | 0 | 0 |
| architecture+security-plan+test-plan-1 | whole | 13 | 13 | 0 | 0 |
| architecture+test-plan | whole | 2 | 2 | 0 | 0 |
| design-system+layout-templates | whole | 1 | 0 | 0 | 1 |
| masters-1 | whole | 3 | 2 | 0 | 1 |
| obs-plan | whole | 1 | 1 | 0 | 0 |
| security-plan-1 | whole | 4 | 1 | 0 | 3 |
| security-plan-2 | whole | 7 | 4 | 0 | 3 |
| test-plan-1 | whole | 5 | 1 | 0 | 4 |
| test-plan-2 | whole | 4 | 2 | 0 | 2 |
| total | | 55 | 41 | 0 | 14 |

### Applied — notes
- `a11y-summary.md:24#1`: the file-input clause is read from the code (`is_masked_input` covers `password` and `file`; `snapshot.rs` masks a non-empty value), beside `ctr-37t3bdwi`.
- Each `tree` finding (`blitz-dom.md:15`, `stack.md:9`, `escher-driver.md:3`, `:60`, `:61`, `:63`): its path tested absent in the tree. The `blitz-dom.md` replacement `.andromeda/cache/chunks/2026-10-31-select-popup/evidence/merge-surface.md` was tested present.

### Declined (held) — each with why
- `design-summary.md:30#1` (iOS target built): the leaf's Android statement is true as it stands, and a reader acts against nothing. The detail stays in design-system `decision/mobile-targets-built-not-tested`.
- `CLAUDE.md:22#1` and `security-summary.md:13#2` (a select under a non-invertible transform opens no list): an edge case. A reader of the leaf acts against nothing, and the detail stays in security-plan `decision/select-list-open-gate`.
- `security-summary.md:13#3` (non-UTF-8 stylesheet): a refinement of a correct statement. The detail stays in `decision/failed-stylesheet-blocks-no-render`.
- `security-summary.md:13#4` (`clickable` reads the same presence): a refinement. The detail stays in `decision/disabled-read-by-presence`.
- `security.md:15#1` (fetch GET/HEAD only): a narrowing the rule file does not need for a reader to act rightly. The detail stays in `decision/fetch-get-head-only`.
- `security.md:21#1` (a refused or malformed request restarts the idle count): a refinement of a correct statement. The detail stays in `decision/host-idle-expiry`.
- `security.md:28#2` (every CI action pinned to a SHA, "keep it so"): the row's fate is `rule`. The tree does not yet hold it for `wpt.yml`, `publish-browser.yml` and `wpt-post-results.yml` (`route-owed.md`), so "keep it so" would state as standing what the route still owes. It is left to the route's owed lines.
- `tests-summary.md:16#1` (exit 3 on `claude` / `python3` absent), `tests-summary.md:16#2` and `verification-harness.md:25#1` (the 600 s timeout and `client-exit`): refinements of the cold-agent pipe. A reader acts against nothing, and the detail stays in test-plan `contract/cold-agent-exit-codes/3` and `decision/cold-agent-run-bounded`.
- `tests-summary.md:21#1` (`stand_handler_coverage` since 2026-11-08): the proposed text carries claims beyond its cited source row. CLAUDE.md's module and architecture lines already state handler coverage. The detail stays in test-plan `decision/handler-coverage-by-replay`.
- `tests-summary.md:28#1` (fork WPT reading report-only): a refinement. The detail stays in `decision/fork-wpt-reading-report-only`.
- `testing.md:48#1` (`keyboard_compat.rs` excluded from coverage): already stated in CLAUDE.md's dioxus-native-dom module line. The detail stays in `decision/keyboard-compat-coverage-exclusion`.
