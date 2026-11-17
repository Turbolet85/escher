@history_item n=1 marker=amd-phad3xqb
**amd-phad3xqb** (2026-10-05-ci-gate-legs) — added the `coverage` leg's lcov `coverage-report` artifact (uploaded on success only, 7 days, the one fork-CI artifact outside `target/ci-logs/`), ratified later by amd-bapu2wis; "floors held" rests on this leg, and a floor change or a crate added to it is a CI gate change, not a new artifact.

@history_item n=2 marker=amd-7v2ajbq2
**amd-7v2ajbq2** (2026-10-06-telemetry-bootstrap) — founded escher-telemetry: stderr-only non-ANSI sink, `RUST_LOG` default `warn`, `service.name`/`service.version` from `CARGO_PKG_*`, the allowlist scrub and the content-named set; the origin of the sink rows (contract/telemetry-sink/*, contract/escher-log-line/*) the stderr-content-free proof reads against.

@history_item n=3 marker=amd-62vk252m
**amd-62vk252m** (2026-10-06-headless-stand) — fixed that the headless stand and its in-process checks install no subscriber, so a headless boot has no escher sink; root of decision/headless-stand-no-sink — a witness run in process proves nothing about stderr.

@history_item n=4 marker=amd-gqcscybd
**amd-gqcscybd** (2026-10-06-stand-test-contract) — added the agent-run harness log (`run.start`, `test`, `run.end` to `target/agent-run/events.jsonl`, raw output in `run.log`, nothing read inside a `failures:` block), harness metadata not a sink; the contract "agent-run re-counted" touches — a re-count must keep its events free of captured output and content-named keys.

@history_item n=5 marker=amd-pbgglsyj
**amd-pbgglsyj** (2026-10-07-audit-corrections) — widened the headless-stand silence census to the checks' shared module `tests/blitz-tests/tests/common/mod.rs`; precedent that any new shared fixture module this sweep adds must be named in the census.

@history_item n=6 marker=amd-vm35icer
**amd-vm35icer** (2026-10-07-driver-session) — recorded `escher-session` as the second sink installer and measured its stderr by level, finding ids and names at `trace`; standing trap: a zero read from a host that installs no sink says nothing about one that does — the stderr-content-free proof must run through a sink-installing binary (`host_log`/`mcp_log`), not in process.

@history_item n=7 marker=amd-yjzlnn3o
**amd-yjzlnn3o** (2026-10-07-sink-target-allowlist) — the sink drops every non-engine, non-`escher_` target whole at every level; rule: a target is admitted only together with a scrub rule for it (decision/sink-target-allowlist).

@history_item n=8 marker=amd-3iczuidb
**amd-3iczuidb** (2026-10-07-act-by-id) — `Command`, `Call`, `ArgValue`, `Outcome` derive `Debug` and are never printed, logged or fielded; rule kept for every later verb (decision/debug-types-never-logged) — a new sweep witness's failure message must not Debug-print an answer.

@history_item n=9 marker=amd-oqhtag4e
**amd-oqhtag4e** (2026-10-07-refusal-detection) — the session's record of ids (bounded, no reader outside the crate, no `Debug`) printed, logged and fielded by nothing; source of decision/session-id-record-unprinted.

@history_item n=10 marker=amd-cschmkve
**amd-cschmkve** (2026-10-07-driver-command-spans) — built the `command` span (eight fields, never fielded content) with `tracing` ungated in escher-driver, and `stand_act_spans`' re-run children as the census exception; origin of the command-span rows and decision/escher-crates-tracing-ungated; workspace build turns engine `tracing` on — why every stderr reading is taken under both builds.

@history_item n=11 marker=amd-6zxxoq74
**amd-6zxxoq74** (2026-10-07-driver-command-spans) — the closed-span line shape and its scrub (content-named span values never stored); typed text first read in process only; origin of contract/span-line/* and decision/closed-span-lines.

@history_item n=12 marker=amd-u2nw2mep
**amd-u2nw2mep** (2026-10-09-founder-rulings) — ratified the closed-span line; the diff's three sizes may reach a log, its content never — the boundary the command span's `added`/`removed`/`changed` stand on.

@history_item n=13 marker=amd-dnjkg5io
**amd-dnjkg5io** (2026-10-10-driver-cli) — `escher-session`'s stdout carries answers, never log lines; standing rule: no span or log field holds what a call supplied or a screen reads — the frame for checking 0.3.0's answers stay off stderr.

@history_item n=14 marker=amd-gftbr5vi
**amd-gftbr5vi** (2026-10-10-driver-cli) — first measured typed text absent from a sink-installing host's log at `trace` under both builds (0 sentinel, 0 ids, 0 names); the measurement method the scope's "measured as 0.2.0's were" repeats.

@history_item n=15 marker=amd-3jlpr7el
**amd-3jlpr7el** (2026-10-10-mcp-surface) — the `tool_call` span (`tool`, `ending`) ratified; `stand_mcp_tools`' recorder an exception to the silent census; `mcp_log` reads the role at `trace`; origin of the tool_call rows.

@history_item n=16 marker=amd-hk2vxiq4
**amd-hk2vxiq4** (2026-10-12-self-description) — `help` in both spans' closed sets, a `help` tool_call with no command child; fixed help text is an answer, never a log line (contract/tool-call-span/help).

@history_item n=17 marker=amd-wivka2yb
**amd-wivka2yb** (2026-10-12-headless-screenshot) — a shot's image leaves only in its answer, 0 image bytes on stderr under both builds; source of decision/shot-image-leaves-only-in-answer, and the `stand_screenshot_repeat` re-run child precedent.

@history_item n=18 marker=amd-4cbpytxz
**amd-4cbpytxz** (2026-10-14-stand-requirement-sweep) — the 0.1.0 sweep's precedent: `stand_requirement_sweep` and its new checks added to the silence census, reader counts re-counted, the host and `mcp` role re-read on the same flows; the shape this 0.3.0 sweep's amendment follows.

@history_item n=19 marker=amd-xzye3ehn
**amd-xzye3ehn** (2026-10-14-quality-gates) — `busy` driven on a real line in `stand_act_spans`; clients' stderr, the host beyond three commands and the windowed binary read by build and level, 0 needles; precedent for widening stderr readings without moving a field.

@history_item n=20 marker=amd-wggssyt7
**amd-wggssyt7** (2026-10-18-one-virtual-clock) — the `command` span's causes set to eight (`time-unavailable` retired), an accepted `advance` recording acting fields; the cause domain (contract/command-span/cause) a sweep witness of refusals reads; ratified by amd-bbjl3fhw.

@history_item n=21 marker=amd-rbvxics4
**amd-rbvxics4** (2026-10-19-session-recording) — recording opens no span and logs nothing; wire `v3`; the Recording directory row (`0700`, one `0600` check per session, call content at rest); ground for contract/recording-file/* and contract/host-wire/v3, which the masks-in-a-recording proof reads.

@history_item n=22 marker=amd-6x37n7n2
**amd-6x37n7n2** (2026-10-19-recorded-check-replay) — each kept answer stored whole, masked as on every carrier, a shot's without `png`; replay adds no span, each replayed step its own command span; ground for contract/kept-check/answer and the masks-held claim on a kept check.

@history_item n=23 marker=amd-y2gzdcnt
**amd-y2gzdcnt** (2026-10-19-test-splits) — seven_guis' shared `tests/common/mod.rs` holds the needle helpers for `host_log`/`mcp_log`, assertion messages a kind and a count only (decision/log-check-failures-name-kind-count); any new needle set this sweep adds goes there under that rule.

@history_item n=24 marker=amd-lqyenybp
**amd-lqyenybp** (2026-10-19-shared-fixtures) — the stand's third shared module `fixtures/mod.rs` silent and in the census; a new fixture the sweep adds lands in a module the census must still read 0 over.

@history_item n=25 marker=amd-vrhzfubf
**amd-vrhzfubf** (2026-10-19-visual-lint) — `lint` an instance verb inside the command span, no field; `host_log`/`mcp_log` each drive one `lint` with 0 needles; the per-verb host/role reading pattern every 0.3.0 verb followed.

@history_item n=26 marker=amd-4q2wobhh
**amd-4q2wobhh** (2026-10-20-large-screen-in-parts) — a part read records `verb` alone; `stand_act_spans` call count 35; one link of the per-mode call-count chain the sweep re-reads.

@history_item n=27 marker=amd-a6pxq634
**amd-a6pxq634** (2026-10-24-stand-requirement-sweep) — the 0.2.0 sweep: `stand_requirement_sweep`'s `WITNESSES`/`WITNESSES_020` tables and its leg check named silent, read through `include_str!`; `host_log` reads a recording session's clients at `trace`; the immediate precedent for this chunk's third table or file and its leg check (the CARRY reach gap).

@history_item n=28 marker=amd-amti4mhy
**amd-amti4mhy** (2026-10-26-codebase-health) — `stand_requirement_sweep`'s private helpers `assert_0_2_0_row`, `assert_0_2_0_witness` and `inside_stand_repeat` recorded silent, messages a row and an index only; the code CARRY `cry-omhvfewt`/`cry-ivqdcli5` widen, which must stay silent.

@history_item n=29 marker=amd-zogtm42c
**amd-zogtm42c** (2026-10-25-cold-agent-bar) — the paged part read through the host's own sink measured (`host_log`'s then-fourth test, `hl-cells`), a residual discharged; precedent for closing a stderr gap with a new `host_log` test.

@history_item n=30 marker=amd-w5hr2k4x
**amd-w5hr2k4x** (2026-10-31-engine-debts) — fixtures readers 24→28; Kept: §3's twenty-nine `session_common` readers left as written against a measured 31 — the gap CARRY `cry-4hxj6rr2` asks this chunk to re-count, its correction the wrap's amendment.

@history_item n=31 marker=amd-z52p4vmm
**amd-z52p4vmm** (2026-10-31-engine-debts) — five new non-`stand_` checks recorded silent in a §6 row only, §3 taking none; precedent for where a fixture check's silence is recorded.

@history_item n=32 marker=amd-2rlbtdpf
**amd-2rlbtdpf** (2026-10-31-engine-debts) — a `multiple` select's chosen set leaves only as form entries, snapshot value and tree nodes, never a log line; an `optgroup` label as a group name; source of decision/select-chosen-set-never-logged.

@history_item n=33 marker=amd-hv3i7j3n
**amd-hv3i7j3n** (2026-10-31-select-popup) — open select list read through the host (`hl-popup`) and the role at `trace`, 0 option needles via `option_needles`; the stderr witness of the select list the sweep names.

@history_item n=34 marker=amd-7g47dwps
**amd-7g47dwps** (2026-10-31-select-popup) — `session_common` readers was twenty-nine, now thirty-two, noting the 29 was already behind the base's 31; history of the reader count the CARRY re-counts.

@history_item n=35 marker=amd-twhohqtg
**amd-twhohqtg** (2026-10-31-select-popup) — a page's option and group labels as pixels in a shot, `size` read, the optgroup label painted, ratified widenings; source of decision/optgroup-label-crossing.

@history_item n=36 marker=amd-tjocwgx3
**amd-tjocwgx3** (2026-10-31-select-popup) — `SelectListRow.label` page text in a public read type, never logged; source of decision/select-list-label-never-logged, part of "stand layout held" (the select list).

@history_item n=37 marker=amd-qrj3ym2p
**amd-qrj3ym2p** (2026-11-07-mounteddata-buffered-render) — the mutation buffer holds page content in memory only, `Debug` its length; source of decision/mutation-buffer-memory-only.

@history_item n=38 marker=amd-wv4t22sd
**amd-wv4t22sd** (2026-11-07-ui-graph) — `graph` the ninth instance verb, `Outcome::Graphed` never logged; `session_common` readers 33 (`stand_graph`); source of decision/graph-answer-never-logged.

@history_item n=39 marker=amd-tlxklvv6
**amd-tlxklvv6** (2026-11-07-ui-graph) — the graph answer's carried set (no state value, typed sentinel 0 times) and its rest in a kept check; ratified by amd-spdvdmvv; the masks-held claim on `graph` reads against it.

@history_item n=40 marker=amd-jsqwtiq2
**amd-jsqwtiq2** (2026-11-07-ui-graph) — `graph`'s command and tool_call lines; `host_log` and `mcp_log` each drive one `graph` at `trace`, 0 needles (`graph_needles`) under both builds — `graph` is already driven by the stderr checks (closed at P3).

@history_item n=41 marker=amd-b2prlxvh
**amd-b2prlxvh** (2026-11-07-ui-graph) — `SignalRegistry` holds no state value, `Debug` counts only; source of decision/state-registry-holds-no-values.

@history_item n=42 marker=amd-zc6kgyy6
**amd-zc6kgyy6** (2026-11-08-what-can-i-do-here) — `stand_actions` silent, `session_common` readers 34; the census entry for the `actions` stand check.

@history_item n=43 marker=amd-quujnooh
**amd-quujnooh** (2026-11-08-what-can-i-do-here) — the actions answer carries ids, owners, verb and cause words, booleans only, a typed sentinel and `MASKED_VALUE` 0 times; ratified widening; the masks-held claim on `actions`.

@history_item n=44 marker=amd-k5v64vz2
**amd-k5v64vz2** (2026-11-08-what-can-i-do-here) — `actions` joins the command span's verbs (fourteen) and tool_call's tools, recording `verb` alone; source of contract/command-span/actions.

@history_item n=45 marker=amd-ei5tfcg5
**amd-ei5tfcg5** (2026-11-08-what-can-i-do-here) — the actions read opens no span and changes nothing in the document; source of decision/actions-read-changes-nothing.

@history_item n=46 marker=amd-d3eh2inx
**amd-d3eh2inx** (2026-11-08-what-can-i-do-here) — `actions_needles`; `stand_act_spans` 38 calls per mode; `host_log`/`mcp_log` each make one `actions` call, 0 needles at `trace` both builds — `actions` already driven by the stderr checks.

@history_item n=47 marker=amd-wxjukvoc
**amd-wxjukvoc** (2026-11-08-handler-coverage) — citations re-pointed; Kept: §9's enumerated `host_log` state directories left without the coverage run's, as unmeasured; a new `host_log` test the sweep adds should name its state directory.

@history_item n=48 marker=amd-u6k3oci4
**amd-u6k3oci4** (2026-11-08-handler-coverage) — the `coverage` role a further reach of the same sink install; coverage line written on stdout as an answer, a `malformed` refusal holds nothing of the path (contract/coverage-role-stdout/coverage-line); ratified by amd-qs55m2hv.

@history_item n=49 marker=amd-srw72242
**amd-srw72242** (2026-11-08-handler-coverage) — `session_common` readers 34→35 with `stand_handler_coverage`; the last recorded count before this chunk's re-count.

@history_item n=50 marker=amd-rszctrp5
**amd-rszctrp5** (2026-11-08-handler-coverage) — `host_log`'s paged-part test moved from fourth to fifth across four sections; trap: a new `host_log` test shifts the ordinals the plan states.

@history_item n=51 marker=amd-r6gvhd4l
**amd-r6gvhd4l** (2026-11-08-handler-coverage) — the coverage role's command shape, endings and `malformed` refusals; source of contract/escher-session-coverage/*; ratified by amd-iir7mv4r.

@history_item n=52 marker=amd-q624ly3m
**amd-q624ly3m** (2026-11-08-handler-coverage) — each replayed step of a kept check writes its own command span line, the coverage line never logged; source of decision/coverage-line-never-logged; ratified by amd-fznt7mdc.

@history_item n=53 marker=amd-ob2zaxca
**amd-ob2zaxca** (2026-11-08-handler-coverage) — the `coverage` module, the reached record and `stand_handler_coverage` silent; `Coverage::to_json`'s form (contract/coverage-line/shape).

@history_item n=54 marker=amd-flbwaun3
**amd-flbwaun3** (2026-11-08-handler-coverage) — `host_log`'s coverage run reads 0 id, name, coverage, sentinel, path and recorded needles at `trace` both builds (`coverage_needles`); the `coverage` role is already driven by the stderr checks.

@history_item n=55 marker=amd-coxkueit
**amd-coxkueit** (2026-11-08-handler-coverage) — the coverage line carries ids, owners, event names and two counts, a typed sentinel 0 times; ratified by amd-dhxza76a; source of contract/coverage-line/content.

@history_item n=56 marker=amd-i4np4ecm
**amd-i4np4ecm** (2026-11-12T03-36-23-rows) — the masters became rows; why CARRY `cry-4hxj6rr2`'s `.andromeda/obs-plan.md:76` coordinate must be re-found as a row in `doc:obs-plan`, not a line.

@history_item n=57 marker=amd-y7qwmdzh
**amd-y7qwmdzh** (2026-11-14-seeded-randomness) — decision/checks-silent's exception widened to a re-run child printing ids or digests; any re-run child a new witness uses must fit that exception or amend it.

@history_item n=58 marker=amd-aq6l4qkc
**amd-aq6l4qkc** (2026-11-14-auto-exploration) — + decision/exploration-adds-no-span: explore instrumented only through the command spans of the calls it drives; bears on the "explored path's check" masks proof.

@history_item n=59 marker=amd-vdqp26ky
**amd-vdqp26ky** (2026-11-14-auto-exploration) — + decision/exploration-catches-panics: a caught panic reported without its payload; constraint on any exploration witness that reads a panic.

@history_item n=60 marker=amd-csae2x3f
**amd-csae2x3f** (2026-11-14-auto-exploration) — ~ contract/escher-session-coverage/malformed: a check carrying a format other than 1 refused `malformed`; the current refusal list a coverage witness must match.

@history_item n=61 marker=amd-zbauq3ll
**amd-zbauq3ll** (2026-11-16-layout-explained) — ~ decision/checks-silent and ~ decision/headless-stand-no-sink: `stand_layout` installs its own recording subscriber; an exception the silence census for the sweep must keep.

@history_item n=62 marker=amd-vqwvafxb
**amd-vqwvafxb** (2026-11-16-layout-explained) — + decision/layout-read-opens-no-span, guard `stand_layout`'s `the_read_writes_no_escher_line_at_trace`; the in-process layout witness CARRY `cry-4cof7vvk` names.

@history_item n=63 marker=amd-b3ck4xhx
**amd-b3ck4xhx** (2026-11-16-errors-with-the-action) — ~ decision/headless-stand-no-sink, ~ decision/driver-library-installs-no-subscriber, ~ contract/telemetry-sink/rust-log: acting steps run under escher-telemetry's scoped capture, which prints nothing; the `RUST_LOG` filter reaches the sink layer alone.

@history_item n=64 marker=amd-pmdqczdc
**amd-pmdqczdc** (2026-11-16-errors-with-the-action) — ~ decision/driver-emits-no-event: the driver sets the scoped capture around an acting step, still emitting no event.

@history_item n=65 marker=amd-cazdlog4
**amd-cazdlog4** (2026-11-16-errors-with-the-action) — ~ contract/escher-sink-env/rust-log (filter on the sink layer alone) and + decision/capture-framework-targets-warn-up; what `RUST_LOG=trace` does and does not reach when measuring `caused` off stderr.

@history_item n=66 marker=amd-g62h26kx
**amd-g62h26kx** (2026-11-16-error-text-in-the-answer) — ~ decision/headless-stand-no-sink and ~ decision/checks-silent (the error-text founder check installs the sink over an in-memory writer), ~ decision/answers-echo-nothing-typed (a caused item holds what its author wrote), ~ decision/driver-library-installs-no-subscriber (chaining panic hook); the `caused` carrier the stderr and masks proofs cover.

@history_item n=67 marker=amd-z4s54auy
**amd-z4s54auy** (2026-11-16-error-text-in-the-answer) — ~ decision/driver-emits-no-event: the capture keeps each record's text and the step's panics in process, printing nothing.

@history_item n=68 marker=amd-nrqmqqqo
**amd-nrqmqqqo** (2026-11-16-error-text-in-the-answer) — ~ decision/capture-framework-targets-warn-up: engine targets admitted at warn and above; which records can reach a `caused` item.

@history_item n=69 marker=amd-fhbjzn6v
**amd-fhbjzn6v** (2026-11-16-error-text-in-the-answer) — ~ contract/telemetry-scrub-set/error and ~ contract/telemetry-scrub-set/url: records cross into `caused` under the field-name scrub; `url` kept `[redacted]` there.

@history_item n=70 marker=amd-guyxffvk
**amd-guyxffvk** (2026-11-16-error-text-in-the-answer) — ~ decision/escher-target-content-named-set: `password` added to the content-named set; the needle set for a masked control on stderr.

@history_item n=71 marker=amd-7xmwhv24
**amd-7xmwhv24** (2026-11-16-error-text-in-the-answer) — − decision/scrub-at-sink-not-call-sites → + decision/scrub-by-field-name, + decision/engine-sites-fixed-messages; the scrub the stderr-content-free reading relies on is by field name, in sink and capture.

@history_item n=72 marker=amd-vvtebi3n
**amd-vvtebi3n** (2026-11-16-error-field-in-the-answer) — ~ contract/telemetry-scrub-set/error: the capture keeps `error` as written in a caused item, cut at the text bound, so error text crosses the answer carriers; ratified by amd-mxcnbjck — a `caused` needle check must not count an `error` field on an answer as a leak, only on stderr.

@history_item n=73 marker=amd-77iqla2t
**amd-77iqla2t** (2026-11-16-error-field-in-the-answer) — ~ decision/scrub-by-field-name: the capture scrubs by the answer's own hidden set; ratified by amd-mxcnbjck.

@history_item n=74 marker=amd-5rrv4k4t
**amd-5rrv4k4t** (2026-11-16-error-field-in-the-answer) — ~ decision/answers-echo-nothing-typed except: a caused item's hidden-set field reads `[redacted]`, an `error` field as written; the exception the masks-held proof on `caused` reads; ratified by amd-mxcnbjck.

@history_item n=75 marker=amd-wjfn5pka
**amd-wjfn5pka** (2026-11-17-layout-on-the-carriers) — ~ decision/layout-read-opens-no-span: reached by the `layout` verb the read runs inside that call's command span; v030-11's carrier witnesses (CARRY `cry-4cof7vvk`).

@history_item n=76 marker=amd-ypcavwq4
**amd-ypcavwq4** (2026-11-17-layout-on-the-carriers) — ~ contract/command-span/verb (fifteen words) and + contract/command-span/layout (verb alone, cause when refused).

@history_item n=77 marker=amd-vstyxann
**amd-vstyxann** (2026-11-17-layout-on-the-carriers) — ~ contract/tool-call-span/tool (fifteen) and ~ contract/command-span/settled (layout records verb alone).

@history_item n=78 marker=amd-etvpmur5
**amd-etvpmur5** (2026-11-17-layout-on-the-carriers) — ~ decision/sink-host-read-at-trace: needle set gains the layout answer's keys; `layout` is already driven at `trace` by `host_log`/`mcp_log`.

@history_item n=79 marker=amd-w7erycve
**amd-w7erycve** (2026-11-17-layout-on-the-carriers) — + contract/layout-answer/content and + contract/layout-answer/logging (ids, closed words, numbers; never a value or typed text); ratified by amd-mxcnbjck; the masks-held claim on `layout`.

@history_item n=80 marker=amd-zbdqebxv
**amd-zbdqebxv** (2026-11-17-a-component-in-isolation) — ~ contract/layout-answer/logging: the answer crosses to the model provider on a live cold-agent run; the current carrier list of the layout answer.

@history_item n=81 marker=amd-mxcnbjck
**amd-mxcnbjck** (2026-11-17-a-component-in-isolation) — ratified the delegate confirms on the error-field and layout-answer rows; those rows stand as the founder's word, not to be reopened by the sweep.
