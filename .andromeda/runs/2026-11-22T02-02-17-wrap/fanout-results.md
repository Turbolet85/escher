# Fan-out results — 2026-11-21-dependency-and-ci-debts (wrap 2026-11-22T02-02-17-wrap)

a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md
architecture: add 10 · set 0 · retire 0 · none 0 · ops-architecture.md
design-system: none · ops-design-system.md — reason: only CI scripts, the workflow, deny.toml and Cargo.lock changed; no UI surface; the one picked row (decision/mobile-targets-built-not-tested) stays true
layout-templates: add 4 · set 0 · retire 0 · none 0 · ops-layout-templates.md
obs-plan: add 8 · set 0 · retire 0 · none 0 · ops-obs-plan.md
security-plan: add 4 · set 3 · retire 0 · none 0 · ops-security-plan.md
test-plan: add 8 · set 0 · retire 0 · none 0 · ops-test-plan.md
test-plan (main): add 2 · ops-main-test-plan.md

Nothing was removed by a fix: no file took a scripted fix.

The homes chosen at check 2 (one fact, one home): the skia archive script's exit codes and lines — architecture
contract/skia-archive-exit-codes/*; the skia-building legs and the sweep under the verified URL — architecture
decision/skia-legs-verify-archive-first and decision/cargo-under-verified-skia-url; the verified archive itself — security-plan
decision/skia-default-build-fetch; the fallback and the force flag — security-plan decision/skia-archive-fallback-source-build
and decision/no-force-skia-binaries-download; the matrix step and its witness — test-plan decision/matrix-skia-from-verified-archive
and decision/skia-witness-reads-by-content; the witness step's printed lines — obs-plan contract/skia-archive-witness-line/*.

ops-a11y-plan.md
decision/a11y-leg-verified-skia-archive · reject — check 2: the fact's home is architecture decision/skia-legs-verify-archive-first

ops-architecture.md
contract/skia-archive-exit-codes/0 · apply — check 1: Accurate this-chunk addition (routine); check 5: the plan's architecture §Standard Contracts entry, the script's exit codes and lines
contract/skia-archive-exit-codes/1 · apply — check 1: Accurate this-chunk addition (routine); check 5: the plan's architecture §Standard Contracts entry
contract/skia-archive-exit-codes/2 · apply — check 1: Accurate this-chunk addition (routine); check 5: the plan's architecture §Standard Contracts entry
decision/skia-legs-verify-archive-first · apply — check 1: Accurate this-chunk addition (routine); check 5: the plan's §Standard Contracts entry, the leg script's SKIA_LEGS arm
name/other/skia-binaries-url · apply — check 5: the plan's architecture §Occupied Resources entry `+ name/other/skia-binaries-url`
decision/skia-verified-prebuilt-archive · reject — check 2: the fact's home is security-plan decision/skia-default-build-fetch
decision/no-forced-skia-download · reject — check 2: the fact's home is security-plan decision/no-force-skia-binaries-download
decision/matrix-skia-archive-witness · reject — check 2: the fact's home is test-plan decision/matrix-skia-from-verified-archive
decision/cargo-under-verified-skia-url · apply — check 6: the disproved claim that the leg's own build runs skia-bindings' build script last, fixed in the leg runner; check 5: the plan's §Standard Contracts entry, the sweep under the verified URL
decision/build-output-checks-layout-agnostic · reject — check 2: the fact's home is test-plan decision/skia-witness-reads-by-content

ops-layout-templates.md
contract/skia-archive-cli/usage · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/2; a CI helper script is no layout-template surface
contract/skia-archive-cli/exit-0 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/0
contract/skia-archive-cli/exit-1 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/1
contract/skia-archive-cli/exit-2 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/2

ops-obs-plan.md
contract/skia-archive-lines/success · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/0
contract/skia-archive-lines/attempt · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/1
contract/skia-archive-lines/exit-1 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/1
contract/skia-archive-lines/exit-2 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/2
contract/skia-archive-witness-line/match · reject — check 7: not_master type 4 — the witness step's print as built; the workflow file and its pin test hold it
contract/skia-archive-witness-line/no-match · reject — check 7: not_master type 4 — the witness step's failure print as built; the workflow file and its pin test hold it
decision/skia-source-fallback-read-red · reject — check 2: the fact's home is security-plan decision/skia-archive-fallback-source-build
decision/refused-skia-archive-ends-leg · reject — check 2: the fact's home is architecture decision/skia-legs-verify-archive-first

ops-security-plan.md
decision/skia-default-build-fetch · apply — check 5: the plan's security-plan decision/skia-default-build-fetch entry; check 6: the stale «with no content check»; within the founder's ruling «Проверенный архив (Recommended)», inputs#I2 §1a — a narrowing, no widening
decision/advisory-ignores-by-id · reject — check 3: the set weakens a standing rule; RUSTSEC-2026-0192's owner is the residual row this wrap mints (P5 card row 1, inputs#I6), its id re-pointed into deny.toml by a CARRY on «Upstream sync ahead of the canvas», so the rule stands as written
decision/dependency-audit-cargo-deny · apply — check 5: the plan's memmap2 except entry; it rests on F3 A, the operator's delegated answer for the founder (delegate operator, 2026-11-21, inputs#I4), PROVISIONAL until his own word
decision/skia-archive-fallback-source-build · apply — check 6: the FORCE claim disproved (inputs#I7, the operator's word at implement P1); check 1: Accurate this-chunk addition
decision/no-force-skia-binaries-download · apply — check 6: the FORCE claim disproved (inputs#I7)
contract/environment-variables/skia-binaries-url · apply — check 5: the plan's security-plan §Secret Management entry, SKIA_BINARIES_URL alone
decision/skia-archive-call-validation · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/*

ops-test-plan.md
decision/skia-legs-build-from-verified-archive · reject — check 2: the fact's home is architecture decision/skia-legs-verify-archive-first and decision/cargo-under-verified-skia-url
decision/matrix-skia-from-verified-archive · apply — check 5: the plan's test-plan §9 entry, the matrix's verified-archive step plus the witness
decision/skia-prebuilt-fallback-stated-limit · reject — check 2: the fact's home is security-plan decision/skia-archive-fallback-source-build
decision/skia-witness-reads-by-content · apply — check 1: Accurate this-chunk addition (routine); the after-implement fact of the operator pass
contract/skia-archive-exit-codes/0 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/0
contract/skia-archive-exit-codes/1 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/1
contract/skia-archive-exit-codes/2 · reject — check 2: the fact's home is architecture contract/skia-archive-exit-codes/2
decision/hosted-runner-cancel-hand-rerun · reject — check 1: it rests on F2 A, a delegated answer held PROVISIONAL; main's own add carries `state: provisional` (ops-main-test-plan.md)

ops-main-test-plan.md
decision/hosted-runner-cancel-hand-rerun · apply — check 5: the plan's test-plan §9 entry, the no-runner cancel as a hosted-runner limit; F2 A, delegate operator, 2026-11-21, inputs#I4, landed `state: provisional`
decision/ci-flag-rebuild-limit · apply — check 5: the plan's test-plan §9 entry, the CI=true rebuild as a measured limit (the operator's technical answer, inputs#I3)

Check 5 floor, the entries no row carries: each advisory's stated wait and the zip ignores gone — no master site (sweep
`zip` · `ttf-parser` · `zbus` · `paste` refused unfired; `sqlite` 2 no-change), their homes `deny.toml` and the residual
rows; the ci-scripts leg gaining test_skia_archive.py — test-plan decision/ci-scripts-unit-tested holds as written; a11y-plan
§1, the adapter probe after the lock move — a gate reading, no row; the harness's security.md zip and sqlite clauses — the
cascade's.
Check 6: every Spec claims disproved entry is disposed — the FORCE claim by security-plan's two adds, the key claim by
test-plan decision/matrix-skia-from-verified-archive and security-plan decision/skia-archive-fallback-source-build, the sweep
claim by architecture decision/cargo-under-verified-skia-url.
Escalations: none. No operation is a boundary widening: the archive fetch is the founder's ruling's own realization, a
narrowing of the unchecked fetch.
