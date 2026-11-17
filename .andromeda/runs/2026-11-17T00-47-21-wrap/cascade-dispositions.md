# Cascade dispositions — 2026-11-16-error-field-in-the-answer (wrap 2026-11-17T00-47-21)

## Step 2 — the sweep
No pattern file: every `rows-land` dry run of this pass printed `patterns: none: {n} set(s) and retire(s) changed
prose alone — no file` (architecture 1 + 1, security-plan 4, obs-plan 3, test-plan 2; the test-plan add brings new
text and no old). No file holds a pattern, so `cascade.py sweep` is skipped; step 3's leaves read each changed row's
delta.

The cross-master citation check, by hand over all seven masters (`rec.py find --root . --doc {doc} --regex
'content-named|scrubbed fields|same scrub|error[^.]{0,40}redacted|reaches no carrier'`, the retired mechanism's
phrasings — «a content-named field read as redacted» in a caused item, «a log's scrubbed fields», «the same set»,
«reaches no carrier»): architecture 0 · design-system 0 · layout-templates 0 · a11y-plan 0 · security-plan 10 hits in
9 rows · obs-plan 11 hits in 9 rows · test-plan 2 hits in 2 rows.
- amended this pass (2): security-plan `decision/caused-text-crossing`, `decision/mask-holds-on-every-carrier` (the new
  text's «every other content-named field»); obs-plan `decision/scrub-by-field-name` (the new text's sink clause).
- no change — a true claim of the sink, which this chunk left unchanged (17 rows): security-plan
  `telemetry-sink-target-allowlist`, `agent-run-events-metadata-only`, `closed-span-lines`,
  `cold-agent-logs-metadata-only`, `escher-target-content-fields-withheld` (×2), `escher-target-content-scrub`,
  `contract/tool-call-span/tool`; obs-plan `sink-target-allowlist`, `agent-run-log-no-user-content`,
  `cold-agent-log-no-user-content`, `content-span-fields-never-stored`, `cold-agent-events-carry-no-user-content`,
  `content-named-span-values-not-stored` (×2), `driver-span-fields-print`, `escher-target-content-named-set` (×2);
  test-plan `cold-agent-events-content-free`, `telemetry-target-allowlist`.
- held, stale, owner named: security-plan `contract/driver-session-reply/no-attribute-text` — Q2 «Hold for the
  founder» (operator, 2026-11-17); its owner is P5's carry.
- the lateral binds: obs-plan §3's `answers-echo-nothing-typed` changed in its exception alone; test-plan §3's log
  schema and the a11y violation schema name no caused field — no bind moved.
The curation homes and the judgment bases: the P1 pre-pass sweep's `curation` rows (session-learnings.md:11-12, the
doc-leg learning) route to P3; no `base` row.

## Leaves
Pass mode (`cascade.py leaves`, baseline 21a5bacb; epoch 5 open, markerless 2), 12 groups · 22 leaves, `--add` CLAUDE.md,
rules/observability.md, obs-summary.md, security-summary.md, services/escher-telemetry.md, tests-summary.md (the P1
sweep's leaf rows; each already in a group). `unmapped .claude/rules/host-linux.md (host)` — a host leaf no master
states: no comparer, no step-3 edit.
- architecture-1 · architecture-2 · architecture-3: 0 findings.
- architecture+obs-plan: 1 finding (stale), applied.
- architecture+security-plan+test-plan-1: 3 findings (2 stale, 1 omission), applied — the omission names caused's
  exception to the no-echo rule, which a reader of the service doc would otherwise act against.
- architecture+security-plan+test-plan+obs-plan: 3 findings (2 stale, 1 omission), applied — the omission is the doc
  leg on CLAUDE.md's `fast` line, the operator's directive.
- architecture+test-plan: 2 omissions, applied (the doc leg; the FailingNet boot).
- obs-plan: 4 stale, applied.
- security-plan-1: 3 stale, applied. security-plan-2: 1 stale, applied.
- test-plan-1: 2 omissions, applied. test-plan-2: 4 omissions, applied (the FailingNet boot ×3, the doc leg ×1).
Total (the tool's `total` line): blocks 23 · applied 23 · by hand 0 · declined 0.
