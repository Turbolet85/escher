# Fan-out results — 2026-11-20-codebase-health · wrap 2026-11-20T08-17-14-wrap

a11y-plan: none · ops-a11y-plan.md — a structure-only refactor; every picked row stays true, no new fact.
architecture: add 3 · set 1 · retire 0 · none 0 · ops-architecture.md
design-system: none · ops-design-system.md — a behaviour-preserving split; no row it touches moves.
layout-templates: none · ops-layout-templates.md — no surface, exit code, tool or region changed.
obs-plan: none · ops-obs-plan.md — no span, log site or hot-path operation changed; the touched files stay silent.
security-plan: none · ops-security-plan.md — no surface, input class, dependency or config changed.
test-plan: add 4 · set 3 · retire 0 · none 0 · ops-test-plan.md
test-plan (main): add 2 · set 0 · retire 0 · none 0 · ops-main-test-plan.md

## Dispositions

ops-architecture.md
decision/source-file-size-bar · reject — check 2: the fact's home is test-plan decision/oversize-test-targets-kept (F4) and decision/upstream-oversize-files-kept (F1), the quality-gates area; its rule also states a standing 800-line bar on escher's own files that neither the founder's rulings (inputs#I5) nor any gate makes
decision/cognitive-complexity-ceiling · reject — check 2: the fact's home is test-plan decision/complexity-ceiling-host-only, which already reads «engine functions over it are left as they are», the founder's F2 now standing behind it
decision/upstream-tests-not-deduplicated · reject — check 2: the fact's home is test-plan decision/upstream-test-clone-pairs-kept
decision/coverage-reading-leaves-out-tests-files · reject — check 2: the fact's home is test-plan decision/test-files-out-of-coverage-by-suffix

ops-test-plan.md
decision/inline-cfg-test-modules · apply — check 5: the plan's expected amendment (test-plan §Test strategy — the out-of-line modules gain snapshot::tests, json::tests a fixtures child), the report's Symbols / APIs and Crates / modules bullets
decision/inline-unit-test-modules · apply — check 5: the same expected amendment on the twin row, so the two sections say one thing
decision/complexity-ceiling-host-only · reject — check 4: the row stands true as it reads — no CI leg runs the reading and none binds a function, the chunk brought escher's seven functions under by its own plan; «engine functions over it are left as they are» is now the founder's F2 (inputs#I5); the new rule would state a standing hold on escher's functions his rulings do not make
decision/upstream-oversize-files-kept · reject — check 3: its second clause «the 800-line file ceiling binds escher's own files only» states a standing bar no ruling makes; F1 lands as main's own add of this handle in ops-main-test-plan.md
decision/own-file-size-ceiling · reject — check 3: «escher's own files are held under 800 code lines» is a standing bar beyond the founder's F4, which keeps the seven targets and rules no bar; F4 lands as main's decision/oversize-test-targets-kept in ops-main-test-plan.md
decision/upstream-test-clone-pairs-kept · apply — check 1: the founder's F3 (inputs#I5, relayed verbatim by the overseer) «Оставить как у upstream (Recommended)», recorded by the operator's wrap directive (inputs#I6)
decision/test-files-out-of-coverage-by-suffix · reject — check 7: not_master type 4 — a measurement of how the coverage leg's exclusion matches file names, held by the coverage configuration in the code; the report's disproved claim is disposed to curation (P3) and the leaf testing.md

ops-main-test-plan.md
decision/upstream-oversize-files-kept · apply — check 1: the founder's F1 (inputs#I5, relayed verbatim by the overseer) «Оставить как у upstream (Recommended)», recorded by the operator's wrap directive (inputs#I6)
decision/oversize-test-targets-kept · apply — check 1: the founder's F4 (inputs#I5, relayed verbatim by the overseer) «Не заводить, оставить файлы (Recommended)», recorded by the operator's wrap directive (inputs#I6)

ops-main-test-plan.2.md
decision/complexity-ceiling-host-only · apply — check 1: the founder's F2 (inputs#I5, relayed verbatim by the overseer) «Оставить как у upstream (Recommended)», recorded by the operator's wrap directive (inputs#I6); the row's rule stands as it reads, its why now names the ruling
