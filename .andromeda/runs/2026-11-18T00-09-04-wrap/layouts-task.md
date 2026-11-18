@task_prose part=session
The session swept every 0.3.0 requirement to a headless witness run on the stand or a fixture page inside the stand-repeat leg: a 0.3.0 table of sixteen rows (v030-01 to v030-17 bar 15), v030-02, v030-03 and v030-16 alone naming no screen witness.
The stand-repeat membership check now reads the stand-repeat arm of the CI leg script alone, and its named-target list grew from ten to seventeen words, the arm's own.
It added a stand check that the masked password and the file input's path never appear in the graph, actions and layout answers, in a kept check that replays, in every explored path's check, and that over MCP the file input's value reads the mask, in both layout modes.
It added five unit tests pinning the browser's document config fields: the root shell provider, the HTML parser provider, the font context, the passed abort signal and parallel styling; the config builder itself is unchanged.
It measured three verified ledger refs naming renamed tests, and re-counted agent-run run stand 332 · 0 · 10 and run all 879 · 0 · 15; coverage held above its three floors, CI green on a re-run after a corrupt profile merge.
No public symbol, verb, endpoint, env var, dependency, CI leg or schema was added or retired: test code only.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand requirement sweep check: each 0.2.0 and 0.3.0 requirement names its headless witness, and every named target runs inside the stand-repeat arm
@task_file n=2 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: a new stand check that masked input values (password, file input path) stay out of the graph, actions and layout answers, the kept check file, every explored path's check, and the MCP answer, in both layout modes
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds a document's config (shell provider, HTML parser, font context, abort signal, parallel style); only its unit tests changed
@task_item n=1 part=touches
decision/no-caller-input-echoed: a new stand check now proves a typed password and a file input's path are never written back in the graph, actions or layout answers, nor in a kept check or an explored path's check; the rule gains a witness, not a change.
@task_item n=2 part=touches
contract/escher-session-graph/answer, contract/escher-session-actions/answer and contract/escher-session-layout/answer: each answer is now checked to hold no masked value in both layout modes; the answer shapes are unchanged.
@task_item n=3 part=touches
contract/escher-session-record/check-file and decision/session-recording-replayable-check: a kept check-{n}.json now proven to hold no masked value (the kept type reads MASKED_STAND_IN) and still replay green.
@task_item n=4 part=touches
contract/escher-session-coverage/answer and contract/escher-session-coverage/check-bound: every explored path's kept check is now proven to hold no masked value; a row over the explored checks gains a witness.
@task_item n=5 part=touches
contract/escher-session-mcp/stdio and contract/escher-session-mcp/tools: over the MCP server the file input's value reads MASKED_VALUE; the sweep pins the tools listing by the test of the fifteen verbs, where a ledger ref still named twelve.
@task_item n=6 part=touches
decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot: the mask check boots a fresh session per explored path with the sentinel already typed; no change to the boot rule.
@task_item n=7 part=touches
decision/headless-stand-pinned-viewport and decision/every-stand-control-keyed: every 0.3.0 requirement is now witnessed headless on the stand or a fixture page inside stand-repeat; a row naming the stand-repeat named targets or their count (ten, now seventeen) is touched.
@task_item n=8 part=touches
contract/agent-run-cli/stdout: agent-run run stand now reads 332 · 0 · 10 and run all 879 · 0 · 15; a row holding the earlier counts is stale.
@task_item n=9 part=touches
decision/load-page-only-network and contract/shell-provider-window-chrome: the browser's document config is now pinned by unit tests to carry the root shell provider, an HTML parser provider, the font context, the passed abort signal and parallel styling; the window chrome methods themselves are unchanged.
@task_item n=10 part=touches
absence/arbitrary-index-html-templates: the document config's builder is unchanged and still offers no template; only tests over its fields were added.
@task_item n=11 part=untouched
The stand stable element ids, stand accent colour, stand a11y attributes and ids-as-attributes decisions: no id, colour, role or attribute of the stand was added, renamed or retired.
@task_item n=12 part=untouched
The cold-agent pipe, its prompt and its CLI contract (exit codes, record dir, stdout, usage, Windows pass-through): the session reached none of them.
@task_item n=13 part=untouched
The escher-session exit codes, refusals, help, lint, screenshot, snapshot, command line, host log and served-task rows: no verb, exit code, refusal or served task moved.
@task_item n=14 part=untouched
The multi_doc and paint_bench command lines, and the mobile-native and web-spa surfaces: nothing there was measured or changed.
@task_item n=15 part=untouched
The CRUD keyboard region, flight type select, open list rows, scroll-into-view and clipping-box visible region decisions: no layout or input behaviour changed.
