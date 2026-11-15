@task_prose part=chunk
The fork's eleventh upstream take: upstream/main at ea07ee716 (16 commits past the last pin 563d3db73) cherry-picked onto the workspace's registry pins, taking what builds without a git Taffy, a git AnyRender or Dioxus 0.8, each left-out commit keeping its measured reason.
The four Taffy-main commits are measured whole (what the merge breaks, the edits, the dependency audit and MSRV legs with a git Taffy, whether a registry taffy carries them); taken only from a registry release, else left out and the no-git-dependency reversal put to the founder as one question.
Conflicts are resolved keeping both sides' intent, and the previously left-out delta and the carried merge surfaces are re-read against ea07ee716.
Every escher proof is re-run after the take: fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, the embolden witness and fork CI green.
The per-OS test tallies of the take's own CI run are re-read and the sink allowlist is re-checked against the taken delta.
@task_file n=1 path=Cargo.toml mark=merge
what: the root manifest, the workspace's coupled dependency pins (taffy from the registry) that the take is cherry-picked onto
@task_file n=2 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer event handling, handle_click and its arms standing as private click_* functions — edits: the click_* arm function an upstream handle_click edit is re-applied to by hand
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser app's tab, TabWebView and its clippy expect_used allow — edits: the justification beside that allow, only where the take touches the browser app
@task_file n=4 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the clippy ban on Request::get in blitz-dom outside stamped_request, read against the taken font-face loading
@task_file n=5 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the take — fast, doc, the dependency audit and the a11y leg
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the blitz-tests target list, one file and one target per behaviour, that upstream's new test files run under
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the sink allowlist's standing capture checks, held at 0 needles
@task_item n=1 part=needs
The take adds engine behaviour on every document, remote pages included: SVG presentation attributes and inline SVG overflow and transform mapped to CSS, the hr element's color, noshade and size as presentational hints, @font-face font-stretch passed to the font system, and SVG image paint clipping in an isolated layer; each carried attribute read is listed for the operator, marked PROVISIONAL.
@task_item n=2 part=needs
Every conflict resolution keeps escher's contract for what the engine's changed set marks, and the document clock invariant the double-click threshold reads.
@task_item n=3 part=needs
Every conflict resolution keeps the single-choice select model and its engine overlay list (its rows, keys, focus and colours) and the multiple select's rows.
@task_item n=4 part=needs
Every conflict resolution keeps the hit stops and the scrolled-box bounds, and an inert element taking no hit; where the take touches hit_inner, the inline-root early return is fixed to return content-box coordinates and a check of the returned coordinate is added.
@task_item n=5 part=needs
Every conflict resolution keeps the stable served node ids and the snapshot, the range input and keyboard models (Tab order, Enter and Space activation, focusability), inert subtrees, and the accessibility tree's state reads (ARIA states, native checked, select expanded, hidden content excluded).
@task_item n=6 part=needs
An upstream commit editing a handle_click arm does not pick cleanly; it is re-applied by hand to its private click_* arm function, keeping click focus, checkable toggle and disabled-control rules.
@task_item n=7 part=needs
The Dioxus document keeps escher's behaviour: its ids, snapshot, UI graph, reached-handler record and the buffered render into a mutation buffer; upstream's mounted-event test stays out until the Dioxus 0.8 move.
@task_item n=8 part=needs
The a11y leg and the stand checks in both layout modes stay green after the take, with fork CI green on the pushed commit and every job settled.
@task_item n=9 part=needs
A layout, text or paint number the take moves (text shaping under font-stretch, stand contrast ratios included) is re-measured and recorded, never loosened to pass.
@task_item n=10 part=needs
The workspace takes no git dependency: the git Taffy and every AnyRender git bump stay out unless a registry release carries them; MSRV stays 1.91; any new or removed dependency is reviewed by hand and run through the audit's licence check.
@task_item n=11 part=needs
No new crate, port, env var or listener of ours; the CDP server's served listener commit stays out unless the take cannot stand without it.
@task_item n=12 part=needs
A taken upstream sub-resource Request::get in blitz-dom routes through stamped_request with its document's URL, and an edited stylesheet-handler construction or font-face fetch caller takes the new document_url.
@task_item n=13 part=needs
The taken delta is read for every new tracing or log call site, target, print and env read, against the sink's target allowlist and its field redaction.
@task_item n=14 part=needs
Additivity: edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded.
@task_item n=15 part=needs
Where the take touches the browser app, the expect_used allow on its tab view gets its justification beside it, per the justified expect/unwrap rule.
@task_standing n=1
convention: a new accessibility check (such as the hit coordinate check owed) is a Rust test over the AccessKit tree or the engine, in the a11y leg's slow tier
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: stand checks run repeated in three rounds and against the shared expectation tables, stated once
home: ci-integration
@task_item n=1 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): the take adds no form validity display.
@task_item n=2 part=not_needed
The driver's screenshot, actions answer and MCP tool result contracts: the take adds no driver verb, answer field or tool result.
@task_item n=3 part=not_needed
Password masking and password caret rows: no taken commit touches password inputs or their text.
