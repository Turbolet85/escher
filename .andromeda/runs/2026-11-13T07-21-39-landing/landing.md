# The landing of rows proposal 2026-11-12T03-36-23

## The head of dry-run.md
```
rows-convert · proposal 2026-11-12T03-36-23 · word f6c83639c6bf74e32eb17a739fa9a0c0874388faf977a4847f6074c44a3f72ff · of 5becb89730a5 · head f84ccd9f58bc · pin 492

re-check 1 · the masters' text pin (492, the proposal's 499): ok
re-check 2 · the store is the list: ok
re-check 3 · the files a row's verdict stands on (0 row(s) moved since edd0cfaea7f0): ok
re-check 4 · the word: ok
re-check 5 · the proposal's recorded tool (rec v1.32): ok
re-check 6 · the install reads rows: ok
re-check 7 · no edge from outside the family: ok
re-check 8 · the strict ground: ok

gone: keep 5 · 5 line(s)
guards: stands 1 · absent 1 · 2 line(s)
divergences: code 31 · holds 1 · rule 2 · 34 line(s)
stale: code 5 · 5 line(s)
rekey: no fate · 0 line(s)
relied: cut 72 · keep 120 · 192 line(s)
moved: no fate · 0 line(s)
```

## The TX
`landed tx 543 · 3473 row(s) of 7 master(s) · 6248 node(s) · 10663 edge(s) · 2026-11-13T07-23-49-2026-11-13T07-21-39-landing.ndjson`

## The read-backs
- `rec.py check --root .` → `check: clean`.
- `rec.py rows-form --root .` → `masters: rows · landing 2026-11-13T07-21-39 uncommitted`, exit 3, as owed before the commit.
- `select master, sheet, count(*) from master_row group by all` against the table of `dry-run.md`: every count equal (architecture 753 · 720 · 19 · 36 · 2; security-plan 347 · 195 · 19 · 1; design-system 49 · 84 · 3 · 1; layout-templates 21 · 99 · 1; test-plan 272 · 326 · 2 · 1; obs-plan 95 · 179 · 5 · 2; a11y-plan 129 · 109 · 3). Each sheet with 0 rows reads no row.
- `rec.py tree --root . doc:{doc}` for the seven masters: architecture 1530 rows in 11 areas · security-plan 562 in 13 · design-system 137 in 17 · layout-templates 121 in 5 · test-plan 601 in 23 · obs-plan 281 in 20 · a11y-plan 241 in 21, each at tx 543. No difference.

## The leaves
`cascade.py leaves --all`, fired once: 15 groups, 25 leaves, 723249 B, `unmapped 1` (`.claude/rules/host-linux.md`, the host leaf). The comparers' one batch `leaf`: 15 of 15 pass, `[PATH_CHECK] PASS`. Applied: `total · blocks 55 · applied 41 · held 14`, 0 by hand. Every decision is in `cascade-dispositions.md` `## Leaves`. Their outputs landed: `landed tx 560 · 95 record(s) of 15 payload(s)`.

## No row
None. No divergence and no stale line is fated `none`.

## Refused rows
refused rows: 3 — a row the fix round refused lands nothing, under any fate; its records are answered by its line alone where no landed row names them
- refused row · architecture · decision/changed-set-write-sites · [ROW_FORM] over — rule: 304 chars, 300 at the most · fct-wqtlmbkv@1 named by decision/removed-node-interaction-state · decision/damage-propagation · decision/geometry-memo-generation · decision/select-list-changed-set · decision/platform-tree-rebuild; fct-ksdng7dt@1 named by decision/interaction-state-cleared-on-removal · decision/dirty-descendant-invariants · decision/anonymous-blocks-in-layout-children-only · decision/dom-mutation-reconstructs-box · decision/animations-advance-by-resolve · decision/color-scheme-change-recascades · decision/device-read-flushes-pending · decision/changed-set-write-sites · decision/changed-set-engine-touched · decision/a11y-rebuild-on-nonempty-changed-set · decision/color-scheme-follows-window-theme · decision/one-incremental-pipeline · decision/paint-tree-topology-only
- refused row · architecture · decision/no-base64-crate · [TRACE] trace — its records hold no base64-encoded · fct-ovyord5g@1 named by decision/image-raster-decoding · decision/serde-json-driver-reader · decision/png-shot-encoder; fct-t7zq5feb@5 named by decision/validated-verb-set-widened-by-ratification · decision/double-click-window · decision/screenshot-reads-session-clock · decision/refusal-causes-closed-at-eight · decision/one-self-description-for-cli-and-mcp · decision/type-replaces-control-content · decision/no-node-id-crosses-the-socket · decision/screenshot-png-in-band · decision/tool-call-span-tool-and-ending · decision/mcp-current-revision-not-taken · decision/seen-ids-record-bounded · decision/refused-call-records-no-ids · decision/measured-limits-stated-in-the-tool · decision/advanced-ms-is-the-clock-move · contract/session-errors/already-running · contract/session-errors/no-session · contract/session-errors/dead · contract/session-errors/host-exited · contract/session-errors/timeout · contract/session-errors/protocol · contract/session-errors/invalid-label · contract/session-errors/state-dir-too-long · contract/session-errors/state-dir-not-private · contract/session-errors/unsupported · contract/session-errors/io · contract/session-errors/not-settled · contract/session-errors/answer-too-large · contract/session-errors/record-dir-not-private · contract/session-errors/recording-failed · contract/session-error-json/shape · contract/cli-exit-statuses/0 · contract/cli-exit-statuses/1 · contract/cli-exit-statuses/2 · contract/cli-exit-statuses/3 · contract/refusal-causes/unknown-verb · contract/refusal-causes/malformed · contract/refusal-causes/not-found · contract/refusal-causes/stale · contract/refusal-causes/disabled · contract/refusal-causes/covered · contract/refusal-causes/off-screen · contract/refusal-causes/fonts-unavailable · contract/refusal-json/shape · contract/refusal-faults/missing · contract/refusal-faults/unnamed · contract/refusal-faults/repeated · contract/refusal-faults/wrong-kind · contract/refusal-faults/out-of-bound · contract/validation-order/first-failure · contract/driver-verbs/snapshot · contract/driver-verbs/click · contract/driver-verbs/type · contract/driver-verbs/press · contract/driver-verbs/advance · contract/driver-verbs/scroll · contract/driver-verbs/screenshot · contract/driver-verbs/lint · contract/driver-verbs/graph · contract/driver-verbs/actions · contract/driver-verbs/start · contract/driver-verbs/status · contract/driver-verbs/stop · contract/driver-verbs/help · contract/acting-verb-results/settled · contract/acting-verb-results/busy · contract/acting-verb-results/added · contract/acting-verb-results/changed · contract/acting-verb-results/removed · contract/busy-classes/render · contract/busy-classes/layout · contract/busy-classes/loads · contract/node-keys/id · contract/node-keys/parent · contract/node-keys/role · contract/node-keys/name · contract/node-keys/enabled · contract/node-keys/checked · contract/node-keys/value · contract/node-keys/selected · contract/node-keys/pressed · contract/node-keys/invalid · contract/node-keys/focused · contract/node-keys/clickable · contract/node-keys/bounds · contract/node-keys/differs · contract/argument-kinds/id · contract/argument-kinds/text · contract/argument-kinds/key · contract/argument-kinds/flag · contract/argument-kinds/milliseconds · contract/argument-kinds/name · contract/argument-kinds/part · contract/key-names/tab · contract/key-names/enter · contract/key-names/escape · contract/key-names/backspace · contract/key-names/delete · contract/key-names/space · contract/key-names/arrow-up · contract/key-names/arrow-down · contract/key-names/arrow-left · contract/key-names/arrow-right · contract/key-names/home · contract/key-names/end · contract/session-wire/version · contract/session-wire/hello-request · contract/session-wire/stop-request · contract/session-wire/call-request · contract/session-wire/refused-version · contract/session-wire/refused-malformed · contract/session-wire/reply-bound · contract/session-wire/ok-accepted · contract/session-wire/ok-refused · contract/session-wire/ok-oversize · contract/session-wire/ok-recording-failed · contract/session-host/connections · contract/session-host/idle-expiry · contract/session-host/stale-socket · contract/session-client/call-answer-read-bound · contract/session-client/start · contract/answer-json/outcome · contract/answer-json/node · contract/answer-json/screenshot · contract/answer-json/lint-finding · contract/answer-json/actions · contract/answer-json/paged-snapshot · contract/answer-json/string-escaping · contract/lint-reason-keys/overlap · contract/lint-reason-keys/clip · contract/lint-reason-keys/truncation · contract/lint-reason-keys/contrast · contract/lint-reason-keys/off-screen · contract/lint-reason-keys/zero-size · contract/command-line/serve · contract/command-line/mcp · contract/command-line/help · contract/command-line/start · contract/command-line/verb-flags · contract/command-line/closed-pipe · contract/mcp-server/protocol-revisions · contract/mcp-server/initialize · contract/mcp-server/ping · contract/mcp-server/tools-list · contract/mcp-server/tools-call · contract/mcp-server/other-methods · contract/mcp-server/request-line-bound · contract/mcp-server/id-echo · contract/mcp-server/result-bound · contract/mcp-server/protocol-error · contract/mcp-server/tool-result · contract/mcp-server/idle-expiry · contract/command-span/verb · contract/command-span/cause · contract/command-span/settled · contract/command-span/busy · contract/command-span/passes · contract/command-span/added · contract/command-span/removed · contract/command-span/changed · contract/command-span/never-fielded · absence/settle-verb-on-socket · contract/tool-call-span/tool · contract/tool-call-span/ending · decision/advance-moves-session-clock · decision/no-base64-crate · decision/answers-written-by-own-json-writer
- refused row · test-plan · decision/per-crate-coverage-floors · [ROW_FORM] taken — another row stands under this handle · fct-na5rpohe@1 named by contract/coverage-floor-exit-codes/0 · contract/coverage-floor-exit-codes/1 · contract/coverage-floor-exit-codes/2 · decision/per-crate-coverage-floors

No record of the three is `named by no landed row`.

## Twins
twins: 31 equal lead · 26 twin section — the landing merges none
- twin sections, numbered and unnumbered forms of one heading: obs-plan §2 · §4 · §5 · §6 · §7 · §8 · §9; design-system Motion · Surface: desktop-native; a11y-plan §1 · §2 · §4 · §5 · §6 · §7 · §8 · §9; test-plan §1 · §2 · §4 · §5 · §6 · §7 · §8 · §9 · §10.
- equal lead: 16 in architecture, 12 in test-plan, 3 in security-plan, each pair or triple of handles under one lead (the lines whole in `dry-run.md`).

## Route owed
architecture · decision/actions-pinned-to-commit-sha · Pin every action of wpt.yml, publish-browser.yml and wpt-post-results.yml to a 40-hex commit SHA, its ref a trailing comment, and widen the pinning check past ci.yml to every workflow.
security-plan · decision/ci-actions-pinned-to-sha · Pin every action of wpt.yml, publish-browser.yml and wpt-post-results.yml to a 40-hex commit SHA, its ref a trailing comment, and widen the pinning check past ci.yml to every workflow.

## The reading's model
The proposal's reading batches (`rows` · `rowsfix` · `verify` · `reverify`) ran on the session's own model; `proposal.json` holds `model: null` for them.
