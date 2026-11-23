@task_prose part=session
The session measured a canvas shot's bytes on CI's linux x86_64, windows x86_64 and macOS aarch64 hosts and found them byte-equal, all twelve digest rows held on the three hosts (CI run green 17/17).
It added one integration test target, a stand check of canvas shots: three fixtures (a solid fill, an antialiased edge, glyphs painted on a canvas with the document's font), each painting a canvas alone, read in two shot forms (whole screen and element) in both layout modes, against one table of twelve answer digests written as literals; no PNG is committed.
It edited the capability statement: the limit that a shot's PNG bytes differ between linux x86_64 and macOS aarch64 now holds for a shot holding document text only, a canvas-only shot being held equal on the three hosts; the statement names the canvas conditions left unmeasured.
The founder's ruling F1 (require byte equality) is the ratification the canvas-only exception lands on; no capability is claimed, no product symbol, verb, error, exit, log field, CI job or leg script moved, nothing retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new stand check in the blitz-tests package, its own integration test target: the cross-OS digest table of canvas shots, three canvas fixtures painted from Rust widgets, whole and element shot forms, both layout modes, row-count check, run by stand-repeat by its prefix
@task_file n=2 path=docs/capabilities.md
what: the capability statement, its stated limit on a shot's PNG bytes across hosts and its list of unmeasured conditions, read by the capability statement's test in the fast leg
@task_item n=1 part=touches
decision/cross-os-checks-hold-no-shot: gains the canvas-only exception, a shot holding no document text is byte-equal across linux x86_64, windows x86_64 and macOS aarch64 and is now compared across hosts by digest; its reason restated to what was measured.
@task_item n=2 part=touches
decision/no-shot-in-cross-os-byte-checks: the same canvas-only exception and restated reason.
@task_item n=3 part=touches
decision/no-screenshot-in-cross-os-checks: the same canvas-only exception and restated reason.
@task_item n=4 part=touches
decision/shot-png-deterministic-per-host: a shot is byte-identical on one host only now holds for a shot with page text; a canvas-only shot was measured byte-equal on three hosts.
@task_item n=5 part=touches
decision/no-screenshot-step-in-committed-checks: the rule stands, no committed check moved, but its why (a shot is byte-identical on one host only) now holds for a shot of page text alone.
@task_item n=6 part=touches
decision/checks-hold-no-screenshot-step and decision/no-screenshot-in-recorded-check-scripts: if their why rests on a shot's bytes differing by OS, that reason is now scoped to a shot holding document text; the rule itself stands.
@task_item n=7 part=touches
decision/cross-os-readings-by-answer-digest: cross-OS readings by answer digest gain a second table beside the served tasks', canvas shots read by the screenshot answer's digest.
@task_item n=8 part=touches
The capability statement rows (decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-statement-from-schema, decision/capability-claims-name-their-checks): the statement's shot-bytes limit was rescoped and its unmeasured conditions extended; no claim and no claim section count moved.
@task_item n=9 part=touches
The one-target-per-file convention (decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file, decision/one-test-target-per-behaviour, decision/blitz-tests-single-test-target, decision/blitz-tests-manifest-difference): a new blitz-tests target was added, auto-discovered, with no manifest line.
@task_item n=10 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: the new check reads every table row in both layout modes.
@task_item n=11 part=touches
decision/table-tests-count-their-rows and decision/expected-value-pinned-as-literal: the new digest table asserts its twelve rows and covers every fixture, each expected digest a literal.
@task_item n=12 part=touches
decision/requirement-witness-inside-stand-repeat, decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat and decision/stand-checks-zero-failure-budget: stand-repeat takes the new stand check by its prefix, three of three rounds green.
@task_item n=13 part=touches
decision/native-platform-ci-matrix: the new check's cross-host reading is the linux, windows and macOS test jobs of the matrix, no job added.
@task_item n=14 part=touches
decision/app-widget-shows-in-shots and contract/handler-coverage-limits/canvas-handler neighbours: canvas widgets painted from Rust are now shot and digested across hosts; the glyph widget keeps the document's font context.
@task_item n=15 part=touches
The bundled-font rows (decision/stand-checks-bundled-font, decision/stand-checks-bundled-font-no-skip, decision/stand-pinned-real-font, decision/font-metric-tests-never-skip): the canvas glyph fixture paints through the document's font and never skips.
@task_item n=16 part=touches
decision/embolden-local-witness and decision/embolden-leg-local-only: the canvas digest table was read green under the font-embolden paint feature on the dev host.
@task_item n=17 part=touches
decision/mutation-controlled-tests: the new check was controlled red-before-green, four controls held.
@task_item n=18 part=touches
The screenshot contracts (contract/harness-screenshot/screenshot, contract/harness-screenshot/screenshot-rect, contract/session-run/screenshot, contract/driver-session/screenshot): the whole and element shot forms are now held to a cross-host digest for canvas content; no contract item moved.
@task_item n=19 part=touches
decision/session-checks-share-one-module and decision/shared-test-modules-not-targets: the new check reads the shared session module's lint, screenshot and screenshot-of helpers.
@task_item n=20 part=touches
decision/test-data-is-code and decision/test-data-in-code: the canvas fixtures and digest table are stated in code, no image or data file committed.
@task_fact n=1 slug=test-strategy
fact: Canvas shots are compared across hosts by digest: three fixtures (solid fill, antialiased edge, canvas glyphs), each a canvas alone, in whole and element forms, both layout modes, one table of twelve answer digests, no pixels committed; held byte-equal on linux x86_64, windows x86_64 and macOS aarch64, ratified by the founder's F1 (require byte equality).
@task_fact n=2 slug=test-strategy
fact: The cross-host equality of a canvas shot is unmeasured under another SIMD level of the CPU renderer, at a scale other than 1.0, or for canvas content beyond the three fixtures; a shot holding document text still differs between linux x86_64 and macOS aarch64.
@task_item n=1 part=untouched
The session wire, socket, lifecycle, session errors and exit codes: no verb, carrier, field, error or exit moved.
@task_item n=2 part=untouched
Refusal causes and their order, including fonts-unavailable: no cause added or reworded.
@task_item n=3 part=untouched
The MCP role, its tools list, results and stderr, and the command-line help and usage contracts: unchanged.
@task_item n=4 part=untouched
Agent-run and cold-agent verbs, events, verdicts and exit codes: unchanged.
@task_item n=5 part=untouched
CI job definitions, leg script exit codes, package installs, caches, action pinning and trigger branches: no CI or leg script line moved.
@task_item n=6 part=untouched
Coverage floors, mutation runs, complexity ceiling and oversize-file rows: no floor, filter or threshold moved.
@task_item n=7 part=untouched
WPT runner, its suites, statuses and expectations: unchanged.
@task_item n=8 part=untouched
Harness construction, settle, virtual clock, timer and input helpers, and the lint defect contracts: no harness API changed.
@task_item n=9 part=untouched
Handler coverage, recorded checks' recording script and committed check content, and the stand file glob rows: no committed check changed, and the rows naming the stand file glob hold no count.
