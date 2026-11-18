# Fan-out results — 2026-11-18-cold-agent-bar (wrap 2026-11-18T09-02-58-wrap)

architecture: add 5 · set 3 · retire 0 · none 0 · ops-architecture.md
security-plan: add 1 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: add 0 · set 0 · retire 0 · none 2 · ops-design-system.md
layout-templates: add 3 · set 1 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 5 · set 3 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 8 · set 6 · retire 1 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — reason: the chunk changes only the cold-agent pipe and its contract tests; no a11y row it touches moves.

Nothing was removed by a fix. Check 5: the plan's test-plan entry on `decision/cold-agent-passes-on-positive-evidence` takes no operation — the standing rule («every counted capability was used») reads true over the widened counted set; the prompt half of that entry is `decision/cold-agent-prompt-names-no-ids` below. The architecture `decision/cold-agent-wrong-call-bar` stands true at 5 (F1, inputs#I5): no operation.

ops-architecture.md
contract/cold-agent-verdict-reasons/capability-unused · apply — check 5: the plan's expected amendment; the standing promise names three verbs and a part read, the requirement set is now eight
decision/cold-agent-six-apps · reject — check 7: not_master type 5 — the prompt's per-app goals are the pipe's script's; the standing row stays true
decision/cold-agent-run-carries-stand-screen · escalate → Q1 — check 1: a boundary widening (graph and actions answers to the model provider), the founder's F4 (inputs#I5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-18, inputs#I5: «Утвердить graph и actions (Recommended)» (the card confirmed by the operator, 2026-11-18)
contract/cold-agent-verdict/graph-calls · apply — check 2: architecture's verdict.json registry already holds lint_calls, screenshot_calls and advance_calls beside obs-plan's log-source rows; the new field joins its siblings here
contract/cold-agent-verdict/actions-calls · apply — check 2: as graph-calls
contract/cold-agent-verdict/layout-calls · apply — check 2: as graph-calls
contract/cold-agent-verdict/list-opens · apply — check 2: as graph-calls
contract/cold-agent-verdict/refused-by-cause · apply — check 2: as graph-calls

ops-security-plan.md
decision/cold-agent-screen-to-provider · reject — check 1: its value adds «an acting step's caused text», which no served app reaches until F5's error page lands, a later route entry; main's own set in ops-main-security-plan.md carries the F4 half
decision/cold-agent-events-metadata-only · apply — check 1: a correction; refusals now reach the verdict as counts per closed cause word
decision/cold-agent-transcript-answers-counted-only · reject — check 7: not_master type 4 — it retells how the list-opening counter decodes, stating no reason

ops-main-security-plan.md
decision/cold-agent-screen-to-provider · escalate → Q1 — check 1: a boundary widening (graph and actions answers to the model provider), the founder's F4 (inputs#I5); the plan's expected amendment · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-18, inputs#I5: «Утвердить graph и actions (Recommended)» (the card confirmed by the operator, 2026-11-18)

ops-layout-templates.md
decision/cold-agent-prompt-names-no-id · reject — check 2: the fact's home is architecture decision/cold-agent-six-apps; the standing row (one fixed prompt naming no id) stays true
contract/cold-agent-cli/counted-verbs · reject — check 2: the fact's home is obs-plan contract/cold-agent-counted-verbs/*
contract/cold-agent-cli/refused-by-cause · reject — check 2: the fact's home is obs-plan contract/cold-agent-verdict/refused-by-cause
contract/cold-agent-cli/capability-unused · reject — check 2: the fact's home is obs-plan contract/cold-agent-verdict-reasons/capability-unused

ops-test-plan.md
decision/cold-agent-server-log-counted-only · reject — check 7: not_master type 5 — the list of what is counted is the code's; the correction is main's shorter set in ops-main-test-plan.md
decision/cold-agent-prompt-names-no-ids · reject — check 7: not_master type 5 — the prompt's sentences are the pipe's script's; the standing row stays true
contract/cold-agent-verdict/capability-unused · reject — check 2: the fact's home is obs-plan contract/cold-agent-verdict/capability-unused; the standing test-plan row («a run that leaves a capability unused fails») stays true
contract/cold-agent-verdict/calls · reject — check 2: the fact's home is obs-plan contract/cold-agent-counted-verbs/* and the per-field contract/cold-agent-verdict rows
contract/cold-agent-verdict/refused-by-cause · reject — check 2: the fact's home is obs-plan contract/cold-agent-verdict/refused-by-cause
contract/cold-agent-verdict/list-opens · reject — check 2: the fact's home is obs-plan contract/cold-agent-verdict/list-opens
decision/prompt-goal-control-phrase-once · apply — check 1: a test-strategy convention the chunk measured (control (d)); fit judges
decision/ci-scripts-controls-read-by-hand · reject — check 1: a pipeline tool's limit (gate.py control's case reader), not the project's decision — curation's

ops-obs-plan.md
contract/cold-agent-counted-verbs/graph · apply — check 5: the plan's expected amendment
contract/cold-agent-counted-verbs/actions · apply — check 5: the plan's expected amendment
contract/cold-agent-counted-verbs/layout · apply — check 5: the plan's expected amendment
contract/cold-agent-verdict/graph-calls · apply — check 5: the plan's expected amendment (new verdict rows)
contract/cold-agent-verdict/actions-calls · apply — check 5: as graph-calls
contract/cold-agent-verdict/layout-calls · apply — check 5: as graph-calls
contract/cold-agent-verdict/list-opens · apply — check 5: the plan's expected amendment (a new row for list_opens)
contract/cold-agent-verdict/refused-by-cause · apply — check 5: the plan's expected amendment (a new row for refused_by_cause)
contract/cold-agent-verdict-reasons/capability-unused · apply — check 5: the plan's expected amendment
contract/cold-agent-verdict/capability-unused · reject — check 7: not_master type 5 — the eight-field list is the code's; the correction is main's shorter set in ops-main-obs-plan.md
decision/cold-agent-server-log-counted-only · reject — check 7: not_master type 5 — the counted-verb list is the code's; the correction is main's shorter set in ops-main-obs-plan.md
decision/cold-agent-server-log-read-by-span-name · apply — check 1: a correction; the command span's cause field is read
decision/verdict-fields-closed · apply — check 1: a correction; refused_by_cause is an object of closed words to counts
decision/cold-agent-verdict-holds-no-content · apply — check 1: a correction; as verdict-fields-closed
decision/graph-actions-layout-not-counted-verbs · apply — check 5: the plan's expected amendment, the decision reversed

ops-main-obs-plan.md
contract/cold-agent-verdict/capability-unused · apply — check 7: the shorter set the not_master on the proposer's correction owes
decision/cold-agent-server-log-counted-only · apply — check 7: the shorter set the not_master on the proposer's correction owes

ops-main-test-plan.md
decision/cold-agent-server-log-counted-only · apply — check 7: the shorter set the not_master on the proposer's correction owes
