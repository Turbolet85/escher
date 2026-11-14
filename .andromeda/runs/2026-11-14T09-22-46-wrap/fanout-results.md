# Fan-out results — 2026-11-14-bundled-fonts (resume, P2)

architecture: add 1 · set 1 · retire 0 · none 1 · ops-architecture.md
security-plan: add 1 · set 1 · retire 0 · none 0 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 1 · set 2 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

ops-architecture.md
decision/stand-typography-dejavu-sans · apply — check 6: Spec claims disproved 1 (registered for four generics only; every family lays out in DejaVu Sans by fallback), the fallback the stated reason per inputs#I6, inputs#I7; check 5: Expected amendment 1's site
decision/stand-readings-same-on-every-ci-os · apply — check 5: Expected amendment 1 (the stand contract's typography read the same on three OSes, guarded by stand_cross_os_readings), settled by the P5-approved plan (inputs#I4)

ops-security-plan.md
decision/driver-session-unix-socket-only · apply — check 4: the report measures in-process sessions of the six served tasks on windows x86_64 and macOS aarch64 (CI#43578124948), so «runs on unix only» holds for the socket host alone; no surface added
decision/cross-os-readings-committed-as-digests · reject — check 2: the fact's home is test-plan decision/cross-os-readings-by-answer-digest (one committed table of answer digests, no kept check file); a digest carries no content, so no data-protection medium is added

ops-design-system.md
decision/stand-families-read-bundled-font · escalate → Q1 — check 2: the families reading by fallback is also stated by architecture decision/stand-typography-dejavu-sans as set here, whose standing clause puts the stand's typography in the stand contract, «not on the design-system product surface»; Expected amendment 3 (the P5-approved plan, inputs#I4) names design-system §Typography — one fact, two homes, the pair stated differently · fit: stands as decision/unavailable-families-use-default-font · resolved: reject (Q1) — operator, 2026-11-14: «Reject the design-system add; the families reading keeps its one home in the architecture row, which takes stand_bundled_font_families as its guard by the second set.»
decision/bundled-typography-same-on-every-os · reject — check 2: the fact's home is test-plan decision/cross-os-readings-by-answer-digest (and its guarantee architecture decision/stand-readings-same-on-every-ci-os)

ops-layout-templates.md
decision/stand-tasks-read-same-across-os · reject — check 2: the fact's home is test-plan decision/cross-os-readings-by-answer-digest (and its guarantee architecture decision/stand-readings-same-on-every-ci-os)

ops-test-plan.md
decision/cross-os-readings-by-answer-digest · apply — check 5: Expected amendment 2 (cross-OS readings compared by the driver's answer digests in a stand check, not a kept check file), settled by the P5-approved plan (inputs#I4)
decision/stand-checks-bundled-font · apply — check 5: Expected amendment 2 («decision/stand-checks-bundled-font gains the families guard»), settled by the P5-approved plan (inputs#I4)
decision/mutation-controlled-tests · reject — check 1: no exception to the rule — the substitute control (families-host) turned the families check red, so the rule holds as written; the one-font-collection lesson is the report's Decisions & corrections bullet 1, routed to P3 curation

ops-main-architecture.2.md
decision/stand-typography-dejavu-sans · apply — check 5: Expected amendment 3 disposed into architecture on Q1 A (operator, 2026-11-14) — the families reading's one home takes its guard stand_bundled_font_families; lands after ops-architecture.md (seen 2)

ops-a11y-plan.md
decision/stand-readings-same-on-every-ci-os · reject — check 2: the fact's home is test-plan decision/cross-os-readings-by-answer-digest (and its guarantee architecture decision/stand-readings-same-on-every-ci-os)
