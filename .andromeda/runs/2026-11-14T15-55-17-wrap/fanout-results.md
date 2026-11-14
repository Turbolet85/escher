# Fan-out results — 2026-11-14-auto-exploration

architecture: add 9 · set 3 · retire 0 · none 0 · ops-architecture.md
security-plan: add 6 · set 4 · retire 0 · none 0 · ops-security-plan.md
design-system: add 7 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 7 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 14 · set 1 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 4 · set 1 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 1 · ops-a11y-plan.md

Homes (check 2): the explore call's contract (the call, expansion, exhaustion, the budget, path endings, explored verbs) — architecture `contract/driver-explore/*`; the exploration report's closed content — security-plan `decision/exploration-report-closed-content`; the panic catch — obs-plan `decision/exploration-catches-panics`; no span — obs-plan `decision/exploration-adds-no-span`; the check format — architecture `contract/kept-check/opening-record` (a standing row in each other master that states the opening record is corrected by its own set); the carriers and navigation absences — architecture `absence/exploration-on-carriers` · `absence/exploration-follows-navigation`; exploration's kept checks per path — test-plan `contract/session-recording/exploration-checks` and the security-plan medium set.

ops-architecture.md
contract/kept-check/opening-record · apply — check 1: rul-ipb3atbm, the opening record's `format` key is this chunk's (report Schema / config); expected amendment
decision/no-seed-on-carriers · escalate → Q1 — check 1: rul-3ndrhilc (provisional discharge, escalate): the set removes the PROVISIONAL state on the founder's own word relayed verbatim (inputs#I6), outside an epoch boundary · fit: — · resolved: reject (Q1) — operator, 2026-11-14: «Hold to epoch boundary» — the row stays provisional and joins the Epoch 4 boundary batch, inputs#I6 named in the handoff
decision/unit-tests-at-file-end · apply — check 1: rul-ipb3atbm, the explore module adds a `#[path]` test child, so the exception's count no longer holds
contract/driver-explore/explore · apply — check 1: rul-ipb3atbm, `escher_driver::explore` is this chunk's (report Symbols / APIs); expected amendment
contract/driver-explore/expansion · apply — check 1: rul-ipb3atbm, the frontier rule is this chunk's (report Symbols / APIs)
contract/driver-explore/exhausted · apply — check 1: rul-ipb3atbm, the two stops and the measured unbounded case (Spec claims disproved 1) are this chunk's
contract/driver-explore/step-budget · apply — check 1: rul-ipb3atbm, `StepBudget` and `MAX_EXPLORE_STEPS` are this chunk's; the budget's home
contract/driver-explore/path-ending · apply — check 1: rul-ipb3atbm, `PathEnding` is this chunk's
contract/driver-explore/explored-verbs · apply — check 1: rul-ipb3atbm, `explored_verbs` by `FieldKind::Png` is this chunk's (Spec claims disproved 2)
contract/driver-explore/exploration-report · reject — check 2: the fact's home is security-plan decision/exploration-report-closed-content
absence/exploration-on-carriers · apply — check 1: rul-ipb3atbm, the expected absence row on the founder's own F1 A (inputs#I6); no carrier widens, so not rul-izni57qv's class; ratified
absence/exploration-follows-navigation · apply — check 1: rul-ipb3atbm, the expected absence row on the founder's own F2 A (inputs#I9, inputs#I10); ratified

ops-security-plan.md
contract/recorded-check/start-record · apply — check 1: rul-ipb3atbm, the standing row states the two-key opening record; `format` read strictly is this chunk's; expected amendment (format read strictly)
contract/kept-check-file/entry · apply — check 1: rul-ipb3atbm, the standing row's opening record gains `format`
contract/kept-check-file/medium · apply — check 1: rul-ipb3atbm, explored paths' checks under the owner-only medium; expected amendment
contract/command-span/line · reject — check 2: the fact's home is obs-plan decision/exploration-adds-no-span
decision/explore-step-budget-bounded · reject — check 2: the fact's home is architecture contract/driver-explore/step-budget
decision/explored-verbs-no-image · reject — check 2: the fact's home is architecture contract/driver-explore/explored-verbs
decision/exploration-report-closed-content · apply — check 1: rul-ipb3atbm, the report's closed content is this chunk's; expected amendment (§Data Protection); the fact's home
decision/exploration-catches-handler-panic · reject — check 2: the fact's home is obs-plan decision/exploration-catches-panics
absence/exploration-on-a-carrier · reject — check 2: the fact's home is architecture absence/exploration-on-carriers
absence/exploration-following-navigation · reject — check 2: the fact's home is architecture absence/exploration-follows-navigation

ops-design-system.md
decision/exploration-in-process-only · reject — check 2: the fact's home is architecture absence/exploration-on-carriers
decision/exploration-follows-no-navigation · reject — check 2: the fact's home is architecture absence/exploration-follows-navigation
contract/explore-call/step-budget · reject — check 2: the fact's home is architecture contract/driver-explore/step-budget
contract/explore-call/path-ending · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
contract/explore-call/explored-verbs · reject — check 2: the fact's home is architecture contract/driver-explore/explored-verbs
contract/kept-check/format · reject — check 2: the fact's home is architecture contract/kept-check/opening-record
contract/explore-call/recording-dir · reject — check 2: the fact's home is security-plan contract/kept-check-file/medium

ops-layout-templates.md
contract/escher-driver-explore/step-budget · reject — check 2: the fact's home is architecture contract/driver-explore/step-budget
contract/escher-driver-explore/path-ending · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
contract/escher-driver-explore/explored-verbs · reject — check 2: the fact's home is architecture contract/driver-explore/explored-verbs
contract/escher-driver-explore/record-dir · reject — check 2: the fact's home is security-plan contract/kept-check-file/medium
contract/escher-session-record/format · reject — check 2: the fact's home is architecture contract/kept-check/opening-record
absence/exploration-on-carriers · reject — check 2: the fact's home is architecture absence/exploration-on-carriers
absence/exploration-following-navigation · reject — check 2: the fact's home is architecture absence/exploration-follows-navigation

ops-test-plan.md
decision/mutation-controlled-tests · apply — check 1: rul-ipb3atbm, the bound-loosening control is this chunk's (report Deviations 9, Decisions & corrections)
contract/explore-call/paths · reject — check 2: the fact's home is architecture contract/driver-explore/explore
contract/explore-call/step-budget · reject — check 2: the fact's home is architecture contract/driver-explore/step-budget
contract/exploration-path-ending/accepted · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
contract/exploration-path-ending/refused · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
contract/exploration-path-ending/panicked · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
contract/exploration-path-ending/diverged · reject — check 2: the fact's home is architecture contract/driver-explore/path-ending
decision/exploration-report-holds-no-content · reject — check 2: the fact's home is security-plan decision/exploration-report-closed-content
contract/explore-call/panic · reject — check 2: the fact's home is obs-plan decision/exploration-catches-panics
decision/explored-verbs-chosen-by-field-kind · reject — check 2: the fact's home is architecture contract/driver-explore/explored-verbs
contract/session-recording/exploration-checks · apply — check 1: rul-ipb3atbm, one kept check per path replaying green is this chunk's (stand_explore); expected amendment (test-plan §5); the fact's home
contract/session-recording/format · reject — check 2: the fact's home is architecture contract/kept-check/opening-record
decision/exhaustion-asserted-on-finite-page · apply — check 1: rul-ipb3atbm, the test strategy moved by the measured premise (Spec claims disproved 1); expected amendment (test-plan §5)
absence/exploration-on-driver-carriers · reject — check 2: the fact's home is architecture absence/exploration-on-carriers
absence/exploration-following-navigation · reject — check 2: the fact's home is architecture absence/exploration-follows-navigation

ops-obs-plan.md
decision/exploration-adds-no-span · apply — check 1: rul-ipb3atbm, exploration adds no span by decision; expected amendment (obs-plan §4); the fact's home
decision/exploration-report-holds-no-content · reject — check 2: the fact's home is security-plan decision/exploration-report-closed-content
decision/exploration-catches-panics · apply — check 1: rul-ipb3atbm, the per-path panic catch, the hook still running; expected amendment (obs-plan §4); the fact's home
contract/kept-check/format · reject — check 2: the fact's home is architecture contract/kept-check/opening-record
contract/escher-session-coverage/malformed · apply — check 1: rul-ipb3atbm, the check reader now refuses a format other than 1 as malformed, so the standing closed list is incomplete

ops-a11y-plan.md
decision/exploration-reads-snapshot-and-actions · reject — check 2: the fact's home is architecture contract/driver-explore/explored-verbs and security-plan decision/exploration-report-closed-content
