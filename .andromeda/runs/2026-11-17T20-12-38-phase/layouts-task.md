@task_prose part=chunk
Sweeps every 0.3.0 requirement in reach (v030-01 to v030-17, not v030-15): each one that is a behaviour of the running UI is proven headless on the stand's tasks or on a minimal fixture beside them, with a check added where none exists. It records, per requirement, the task or fixture and the check that proves it there.
Every integration witness it names runs inside the stand-repeat leg by the chunk's end. It widens the record's list of stand-repeat targets and makes the leg check read the stand-repeat leg's own body.
Re-reads the coverage floors, re-counts the agent-run contract's selection counts against 0.3.0's added targets, and corrects any that have drifted.
Re-measures stderr as content-free and masks as held on the 0.3.0 verbs and roles (graph, actions, layout, caused, coverage). Re-asserts the stand layout (viewport, font, spacing, the screens' structure) after the bundled font, the select list and the isolated component.
Claims no cap. It may kill five surviving document-config field mutants in the browser's tests, if P4 takes that work.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs mark=edits
what: the stand requirement sweep's record — per-requirement witness tables pinned by test function name, the list of stand-repeat named targets, and the leg check over the CI leg script — edits: a 0.3.0 witness table (or a sibling file, a P4 fork), the v030-11 witness row, the STAND_REPEAT_NAMED list widened to the leg's seventeen targets, the leg check narrowed to the stand-repeat leg's body
@task_file n=2 path=examples/seven_guis/tests/host_log.rs mark=edits
what: the stand host's stderr measurement at RUST_LOG=trace — no id, name, value, typed text or caused message on stderr
@task_file n=3 path=examples/seven_guis/tests/mcp_log.rs mark=edits
what: the MCP server's stderr measurement at RUST_LOG=trace — no id, name, value, typed text or caused message on stderr
@task_file n=4 path=examples/seven_guis/tests/common/mod.rs mark=edits
what: the shared helpers of the stand example's stderr log checks
@task_file n=5 path=scripts/agent-run.sh mark=edits
what: the agent-run test contract — the counts of what run stand, run all and run session select — edits: the counts it states
@task_file n=6 path=.github/scripts/test_agent_run.py mark=edits
what: the agent-run contract tests — edits: the selection counts they assert
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script — the stand-repeat leg's target selection and the coverage leg's per-crate floors — edits: the stand-repeat selection and the coverage floors, only where a P4 fork answers change
@task_file n=8 path=.github/scripts/coverage_floor.py mark=re-reads
what: the coverage leg's per-crate floor check (escher-driver 97 %)
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow and leg contract tests — only where a P4 fork changes a leg's selection or a floor
@task_file n=10 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader and its document config builder — edits: tests of make_doc_config's shell_provider, html_parser_provider, font_ctx, abort_signal and style_threading fields, only if P4 takes the work
@task_item n=1 part=needs
The stand's pinned headless viewport, font and spacing, and the task screens' structure, must still read as the desktop-native surface states them after 0.3.0's bundled font, select list and isolated component.
@task_item n=2 part=needs
Every stand control keeps its stable element id as an attribute. A witness or fixture check reaches controls by those ids, the task cards and every task's controls alike.
@task_item n=3 part=needs
The flight type select's two options read at boot, and an open list's rows read their own bounds. Both are 0.3.0 select-list behaviours the sweep proves on the stand.
@task_item n=4 part=needs
Each 0.3.0 screen behaviour is proven on the stand's tasks or a minimal fixture beside them, run headless in the pinned viewport, and the fixed document skeleton with its main mount stays as it is.
@task_item n=5 part=needs
The session's graph, actions and layout answers and an acting call's caused report are witnessed on the stand. v030-11's row names the layout verb's witnesses on every carrier.
@task_item n=6 part=needs
A password or file input's value reads as the masked value on every answer that can carry a control's reading (graph, actions, layout, caused, a kept check, an explored path's check). A masked type stays the masked stand-in in a session recording.
@task_item n=7 part=needs
Stderr stays content-free at trace level for the graph, actions and layout verbs, an acting call's caused report, and the coverage role. No id, name, value, typed text or caused message is printed, so caller input is never written back.
@task_item n=8 part=needs
The coverage role's answer and check file are proven as 0.3.0 capabilities, and the role's stderr is measured.
@task_item n=9 part=needs
The agent-run contract's stdout and usage counts for run stand, run all and run session are re-counted against 0.3.0's added targets.
@task_item n=10 part=needs
Session roles share the stand boot, and a held instance reads as a fresh boot. A new witness that drives a session role relies on both.
@task_item n=11 part=needs
Stand names, roles, states and ids come from attributes alone, and the stand lints clean at boot. A fixture added beside the stand must meet the same rules.
@task_item n=12 part=needs
The isolated component's visible region is read through clipping boxes. An into-view scroll moves every scroller that holds it, and the CRUD list stays one keyboard region.
@task_item n=13 part=needs
The session recording is a replayable check (check-{n}.json). A kept or explored check's reading is held to the masking rule.
@task_item n=14 part=needs
Requirements that are not screen behaviours (code audit complexity, README and front page, the upstream take, records, the cross-OS legs) are recorded with their witnesses as they stand, or with a statement of why none runs inside stand-repeat.
@task_item n=15 part=needs
A boundary widening (a new carrier, field or exit, or a driver answer that moves on every carrier) is the founder's fork, raised as a question and never decided as a lean.
@task_item n=16 part=needs
A reading the sweep measures and leaves unbuilt keeps an owner: a residual row or a CARRY placed at the wrap, never dropped.
@task_standing n=1
convention: a new fixture check is one file and one test target per behaviour, run headless in both layout modes, and reaches the stand's controls by their stable element ids
home: surface-desktop-native
@task_standing n=2
convention: a new stderr or answer check of a session verb or role asserts that no caller input, id, name or value is echoed and that masked values stay masked
home: surface-cli
@task_item n=1 part=not_needed
The cold-agent pipe's command-line contract and the cold-agent decisions: v030-15 (Cold-agent bar) belongs to a later entry, and this chunk does not take it.
@task_item n=2 part=not_needed
The multi_doc and paint_bench command lines: benchmark tools that no stand requirement witness reaches.
@task_item n=3 part=not_needed
The mobile-native and web-spa surfaces: the sweep runs only on the desktop stand and its fixtures.
@task_item n=4 part=not_needed
The absence of arbitrary index.html templates for a document: the chunk adds no document template.
@task_item n=5 part=not_needed
The Decisions Log area: it records no intent this chunk acts on.
