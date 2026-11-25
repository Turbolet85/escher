@row_set n=1 row=decision/driver-executor-integration-tested cell=rule seen=1 detector=D-tests-coverage
value: The driver's executor verbs, its refusals and the session lint's readings over a held instance are tested by integration checks, not unit tests; the canvas label reader, which reads recorded paint commands and no held instance, is unit-tested.
basis: fct-mbmku3j6
because: The chunk adds the session lint's canvas label reader with unit tests over recorded paint commands, so a rule saying the session lint is tested by integration checks only no longer holds whole.
@row_set n=2 row=contract/lint-findings/order cell=promise seen=1 detector=none
value: Lint findings come in document order, then in the stated order of the defects; a canvas element reads `truncated` once per label, those findings in paint order.
basis: fct-nolgh3rx
because: The lint now chains a canvas element's label truncations after its DOM truncated reading, one finding per label in paint order, which the order contract must state.
@row_set n=3 row=decision/no-screenshot-step-in-committed-checks cell=except seen=2 detector=none
value: A screenshot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
basis: fct-wjm56irm
because: Circle Drawer's canvas digest has now read green on three hosts and its watch met its retire condition, so a row saying screenshot repeatability is asserted on one host only must name the canvas-alone exception.
@row_add n=1 row=decision/canvas-lint-proven-on-planted-page area=integration-test-strategy fact=F1 detector=D-tests-coverage
name: Canvas lint proven on a planted page and the stand
rule: The canvas lint is proven on a planted page of numbered label cases (past an edge, a clip or the ascent; inside; inside a clipped box; after a pop) and on Circle Drawer reading no canvas finding at boot, after a click and at 12 and 100 circles, in both layout modes.
guard: stand_canvas_lint
state: provisional
basis: fct-gsu6d4et
because: The chunk adds the canvas lint with an integration strategy chosen on a delegate's answer, and no test-plan row states how the canvas lint is proven.
