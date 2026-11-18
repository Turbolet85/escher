@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing guarded; CI legs locked) — `--locked` made to cover every ci.yml cargo leg (`examples/wasm_hello` excepted), the origin of `decision/locked-builds`; any `--test` this chunk adds to the `a11y` leg or `stand-repeat` in `.github/scripts/ci-leg.sh` must keep `--locked`.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — brought in the audit, a11y and coverage legs, all under `--locked`, and made ci.yml's token read-only and SHA-pinned; the `a11y` leg this chunk extends is one of these locked legs, so a new witness joins its `--test` list and adds no unlocked cargo call.

@history_item n=3 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (`disabled` row) — split `disabled` into a parse for focus and presence for state and clicks, after `disabled="false"` was measured matching `:disabled`; this is the first step of `decision/disabled-read-by-presence`, which the SC 2.1.1/2.4.3 re-assertion reads when a control turns disabled.

@history_item n=4 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the HTML `id` row: an author key only when non-empty, `/`-free and first in pre-order, else a path; no panic, no engine id. It underlies the stable ids that stand_keyboard's focus sequences and the contrast pairs name.

@history_item n=5 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — a row's id is a model-assigned key, never an index, pointer, hash, clock or process-local value (`contract/stable-element-id/stability`); an SC 2.4.3 sequence the chunk states by id keeps holding across re-renders only because of this.

@history_item n=6 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stable id leaves through the platform accessibility adapter as `author_id`, and accessible names come from `aria-label`/`<label for>`. It set the CARRY on "Stand a11y assertions" (never a NodeId/pointer in `author_id`) and the standing rule that any further consumer of the id reopens the row; a re-asserting check reads, never adds, a consumer.

@history_item n=7 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot is an in-process reader of ids and names that adds no crossing, with a trap: the first wire form is a crossing. The a11y checks read the snapshot in process; that stays inside this ruling.

@history_item n=8 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the password value masked in the snapshot; the snapshot's `enabled` follows the presence reading; a falsy boolean attribute is removed (`decision/falsy-boolean-attribute-removed`). The snapshot `disabled`/`enabled` readings the re-assertion uses depend on this.

@history_item n=9 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — one text form, `Snapshot::to_text`: ids and names quoted in Debug form (`contract/snapshot-text/id-field`, `name-field`), and the file input masked too. Any snapshot-text reading a new check asserts is bound by this single form.

@history_item n=10 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the platform tree is rebuilt on change, and the diff is an in-process reader with masked values. Its windowed witness was owed from here, which is the line that leads to residual `res-llkgqygv` and that this chunk's scope keeps out.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (socket: threat vector, access, input rows) — the session's one Unix-domain socket, `0700`/`0600`, no TCP/UDP, a closed grammar, bounds. It is the origin of `decision/session-socket-unix-only`, `decision/driver-session-unix-socket-only` and the socket bounds the served-page checks run through.

@history_item n=12 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — measured ids and names reaching a host's stderr at debug/trace. Its trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. Any check this chunk adds that touches logging must run against a sink-installing host.

@history_item n=13 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records are dropped whole, so a sink-installing host logs no id or name at any level. This is the source of `decision/accessible-names-not-logged` and `decision/stable-id-not-logged`; the re-assertion adds no log site and must leave these readings at 0.

@history_item n=14 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — every driver call is validated against a closed table before anything runs, and a refusal holds nothing a call supplied (`decision/single-validation-gate`, `decision/refusal-holds-no-call-input`). The scope adds no verb or argument, so these stay untouched.

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — an id is looked up anew on every call, never by index, and an unknown id is `not-found` with nothing run (`decision/id-lookup-anew-each-call`, `decision/unknown-target-refused`). Keyboard and focus checks driving the served pages by id rest on this.

@history_item n=16 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the fixed refusal order disabled, off-screen, covered, with the snapshot and both focus readings unchanged, and `scroll` reporting `in_view` (`decision/target-refusal-order`). A control turning disabled or hidden in the SC 2.4.3 sweep reads through this order.

@history_item n=17 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's span record class, with the standing rule that a count of a diff's nodes may reach a log but no node, id, name or value of it may. Any new check's log reading stays inside this.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries calls and answers) — ratified the widening `decision/session-socket-carries-calls`: an id and typed text in; ids, names, masked values, the snapshot text and the diff out; no NodeId. The served-page re-assertion runs over this carrier and must stay inside its set.

@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer) — names and ids leave on stdout and the socket but reach no log; a typed sentinel measured at 0 in the host's stderr at trace. These are the readings any served-page check must keep.

@history_item n=20 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings; the closed-field rule) — selected, pressed and invalid added as closed words and booleans (`decision/snapshot-state-readings`, `contract/snapshot-text/state-words`), plus the rule that a closed-value field inside a ratified carrier is no widening (`decision/closed-value-field-no-widening`, `decision/boundary-widening-rule`). This is the test for whether an a11y fix crosses a boundary.

@history_item n=21 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (control-states Markup row; the adapter's third class) — `aria-selected`/`pressed`/`invalid` are matched exactly in lower case (`decision/aria-state-lowercase-exact-match`) and cross only as closed values (`decision/control-states-closed-values-only`). The adapter-side witness is owed, which is the AT residual this scope keeps out.

@history_item n=22 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and crossing) — MCP results became a third exit for ids, names, control states and masked values. The served pages' readings over MCP stay inside this ratified set.

@history_item n=23 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and fixed text) — a password typed over MCP measured masked by `stand_mcp_masked`, now a guard of `decision/password-file-value-masked`. The rule that program-fixed text is no widening while page text is one also governs any help text a fix might touch.

@history_item n=24 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (pixels in band) — `decision/screenshot-in-band`/`screen-pixels-in-band`: the PNG crosses as base64 or an MCP image under the unchanged bound. A contrast check that samples pixels reads inside this ratified crossing.

@history_item n=25 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the engine's password mask, `png`) — a password painted as bullets in every document (`decision/password-painted-masked`, guarded by `password_painted_masked`, `stand_act_screenshot`), and blitz-test-harness taking blitz-paint and anyrender with no lock change. Contrast sampling of a password field reads bullets, never text.

@history_item n=26 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness takes `color` (already in the graph through blitz-dom) as the contrast reader's luminance source, with no lock change and no `tracing` or `style` dependency. This is the source of `decision/color-crate-luminance-source`, `decision/color-is-contrast-luminance-source` and `decision/test-harness-no-tracing-no-style`. The SC 1.4.3 re-read must stay on `color`; the scope adds no crate dependency.

@history_item n=27 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input row (`decision/range-keyboard-actions`, `range-value-exits`, `range-value-sanitized`, `range-current-value-apart`): arrows, Home and End act only with no `disabled`. SC 2.1.1 on a range control re-asserts exactly these keys.

@history_item n=28 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation (Enter keydown, Space keyup), a button click focusing the button, and `disabled` by presence, all ratified widenings (`decision/enter-activates-focused-control`, `space-keyup-activates-focused-control`, `button-click-focuses`). It recorded the known `<div disabled tabindex=0>` limit. This chunk re-asserts stand_keyboard on these arms and may not widen them.

@history_item n=29 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the tree the shell hands the adapter was proven headlessly in both layout modes, with `author_id` never a NodeId or pointer form. What an AT reads was re-homed to a residual, the founder's F1 «Пока нет, перенести». This is the last assertion the chunk re-reads, and the AT witness stays out of scope.

@history_item n=30 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the password click caret was a known limit, and the `disabled` narrowing and the overlay fade were placed on residuals (C10, C15). These are prior a11y-adjacent limits; the chunk should know which were later closed (see the 2026-10-18 engine-debts items).

@history_item n=31 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the whole-workspace audit and licence gate; the `coverage_floor.py` row (`decision/per-crate-coverage-floors`); lockfile verification lists the `stand-repeat` leg. A witness joining `stand-repeat` or the `a11y` leg stays under these locked legs and the coverage floors.

@history_item n=32 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the range and keyboard-activation rows cite named functions (`press_range_input_key`, `press_range_input`, `click_button`). Any citation in an a11y-plan guard that names these arms should use those names.

@history_item n=33 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (merged ARIA reads) — `aria-labelledby` names; descriptions exit only to the adapter; `aria-expanded`, `aria-disabled` and `aria-level` are closed values, and `aria-disabled` moves no snapshot reading (`decision/merged-aria-states-closed-parse`, guarded by `accessibility_merged_states`). One of the `accessibility_*` fixtures the scope re-reads.

@history_item n=34 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`inert` read and toggle per role) — `inert` by presence: not focusable, no hit, out of the tree and the snapshot (`decision/inert-subtree`), with its focus-fixup limit placed on a residual. Toggle read per role (`decision/toggle-read-per-role`). This is the "`inert` (ratified 2026-10-18)" the scope names as changed since the last assertion.

@history_item n=35 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls; password caret) — `disabled` by presence on a form control alone through `is_disabled` (`decision/disabled-read-by-presence`), so a `disabled` on a `summary`/`a` gates no key. The password caret is laid out by the bullets. This is the "`disabled` by presence on a form control alone" the scope re-asserts against.

@history_item n=36 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (PROVISIONAL batch ratified) — the founder ratified the `disabled`, toggle, `inert`, activation-gate and caret marks plus the eight-cause set. The engine behaviours the re-assertion reads are standing, not provisional.

@history_item n=37 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set became eight (`time-unavailable` retired), `advance` is accepted on every session, and the overlay fade reads the document's clock, so two shots of one state read the same pixels. A contrast check over shots is deterministic only under this; the set is `decision/closed-refusal-cause-set`.

@history_item n=38 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a masked `type` is kept as `MASKED_STAND_IN` (`contract/masked-input-value/masked-stand-in`) and kept checks are owner-only. Any kept check a new witness records holds the mask.

@history_item n=39 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (in-process lint; text extent reader) — `BaseDocument::text_extent` is read-only and answers `None` when unresolved (`decision/text-extent-read-only-none-when-unresolved`). The lint's contrast/truncation readers are the prior art for reading text on the stand.

@history_item n=40 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the single-choice `select` row (`decision/single-select-selectedness`, `select-choice-exits`, `select-typeahead-private`), labelable-element label binding, and a click on a select focusing it. These are the keyboard and focus semantics of selects that SC 2.1.1/2.4.3 re-assert.

@history_item n=41 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `part` is a whole number 1-1000 (`contract/driver-argument-kinds/part`) and a part's text reaches no log. Snapshot readings of large served pages (Cells) page through it.

@history_item n=42 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the windowed AT witness's owner set to the residual re-carried past 0.2.0, and the cause set restated as eight. Confirms the AT witness is not this version's to close.

@history_item n=43 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`open` by presence, `:open`) — `open` read by presence on `details`/`dialog`, `:open` matching. Later scoped so `open` on a select steers nothing (`decision/select-expanded-from-engine-state`); a focus check on `details`/`summary` reads this.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk action at its two CI sites) — `Stand checks repeated` frees disk, a step not shown to be the cure for its out-of-disk red. Relevant if new `stand-repeat` witnesses grow the job; see the 2026-11-17 move of that row.

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms; `off-screen`) — the visible region read through transforms, and under rotation the bounding box (`contract/refusal-causes/off-screen`). Off-screen focus or refusal readings in the re-assertion carry this limit.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`type` on a multiple select; select readers) — `focus_multiple_select` focuses with no click, and the readers go through `get_node`. This precedes `decision/driver-type-on-select`, a focus path SC 2.4.3 meets.

@history_item n=47 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkable's checked state in the tree) — a native checkbox's or radio's checked state reaches the tree under a checkable role only, pinned by `accessibility_states`. This is one of the "engine debts" the scope lists, read by the `accessibility_*` fixtures.

@history_item n=48 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus an inert subtree loses; the tree's `focus`) — focus is cleared when it comes under `inert` with no blur or focusout (`decision/inert-focus-clear-silent`), and the tree's focus names only built nodes (`decision/accessibility-focus-built-nodes-only`), pinned by `inert_subtree`. Here `disabled` and `hidden` were left unmoved: the root of the CARRY that a focused control that no longer shows keeps focus. That is the SC 2.4.3 gap this chunk pins and does not remove.

@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the engine's `change` on a checkbox and a radio) — `decision/engine-change-event-checkable`, guarded by `checkable_change_event`, is one of the engine debts the scope lists. Keyboard activation of a checkable fires it.

@history_item n=50 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `select` row's rules, limits and pins) — optgroup options listed and `multiple` settling; at that point no popup, and `size` unread. The pins (`select_interaction` 18, `stand_act_select` 10) are among the a11y fixtures re-read.

@history_item n=51 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys, clicks and chosen set) — the closed key match `press_multiple_select_key`, and disabled rows never chosen (`decision/multiple-select-keys-and-clicks`, `multiple-select-set-exits`); Ctrl+A is not modelled. These are the "`multiple` select's keys" SC 2.4.3 re-asserts focus after.

@history_item n=52 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup` and its `label`) — a group's options listed and its label its tree name (`decision/optgroup-options-and-label`), the `option_disabled` single rule (`decision/option-disabled-one-rule`). One of the engine debts listed.

@history_item n=53 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — a subtree under such a transform takes no hit, which also gates the select list's opening (`decision/select-list-open-gate`).

@history_item n=54 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' `cells-grid` keys) — `decision/cells-grid-keyboard`: `tabindex` 0, a closed key match, no role; Enter opens the editor; the editor's Enter returns focus to the grid. This is the Cells keyboard model SC 2.1.1/2.4.3 re-assert on the served Cells page.

@history_item n=55 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause; select verbs' help) — an option of a closed select stays refused `covered` (`contract/refusal-causes/covered`), with the restated help as fixed text. A keyboard check reaching options must open the list first.

@history_item n=56 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (readers and close add no panic) — a select made inert, disabled, hidden or removed closes its list with no panic (`decision/select-list-closes-when-select-unfit`), and a row with no styles reads `None` (`decision/select-row-layout-none-without-styles`). These are the "open select list closing" focus moments SC 2.4.3 re-asserts.

@history_item n=57 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key-press panic after a hidden ancestor) — recorded the panic when the focused control's anonymous block was dropped, a remote-reachable panic; `select_popup` skipped that key press. Retired at 2026-11-01. It is the history behind the hidden-control focus trap.

@history_item n=58 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a shot of an open list holds labels as pixels) — `decision/open-list-shot-pixels`, guarded by `cli_commands`, a ratified widening. Contrast sampling of an open list reads page text as pixels inside this crossing.

@history_item n=59 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses) — a row's own `bounds` only, no `expanded` node key, and the highlighted row crosses no structured carrier, which the lint does not read for contrast. The SC 1.4.3 re-assertion of the list's highlight must read pixels, not a structured field, and adds no key.

@history_item n=60 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — `size_rows` capped at 65534; `0`/non-numeric gives no hint (`decision/multiple-select-size-rows`, `select-size-hint-cap`). This is the "`size` on a `multiple` select" the scope lists.

@history_item n=61 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded`; an author's `open` and `aria-expanded`) — `decision/select-expanded-from-engine-state`: a single select's expanded state is engine state only.

@history_item n=62 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a label's click on a bound single select) — the label click focuses the select and opens nothing. A focus-order check through labels reads this.

@history_item n=63 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (which select opens a list; what a row takes; hostile markup) — `decision/select-list-open-gate`, `select-list-row-choice` and `select-list-twenty-rows`, each a ratified engine change; their guards are named tests in `select_popup`.

@history_item n=64 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (Enter, Space and a click open the list) — `decision/select-list-keys-and-painted-labels`: Enter and Space open the single select's list, Space only with no typed text. This is the SC 2.1.1 path for selects.

@history_item n=65 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's state, keys, pointer and close) — `press_open_list_key`'s closed match; Tab chooses and closes before focus moves in either direction (`decision/select-list-tab-chooses-and-closes`); what closes the list (`decision/select-list-closing`); pointer release chooses. These are exactly the focus-after-close moves SC 2.4.3 re-asserts.

@history_item n=66 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`optgroup`'s `label` painted as a header) — a group label is painted in the open list and leaves as pixels. Header text is a contrast pair SC 1.4.3 must read.

@history_item n=67 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click and no DOM focus events, and closes any list (`decision/driver-type-on-select`). A focus-order check must not count a `type`'s focus as a DOM focus event.

@history_item n=68 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select readers and the list's public surface) — the list's surface is read-only but for `close_select_list`; `SelectListRow.label` is page text, never logged (`decision/select-list-label-never-logged`). A check reading rows uses these readers and adds no writer.

@history_item n=69 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — the visible region of a list row is the list's scrollport (`decision/visible-region-open-list-row`); the list is painted over the page (`decision/select-list-painted-over-page`); border, highlight and disabled-text colours are engine constants, background and ordinary text the select's own page styles (`decision/select-list-engine-colours`). This is the colour source SC 1.4.3 reads, and a failing pair in those constants is an engine change for every document.

@history_item n=70 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list in the log) — `host_log`/`mcp_log` open and choose in a list with 0 option needles at trace; the list holds no log site. Any new list-driving check must keep that silence.

@history_item n=71 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's markup reaches the select list) — the list is an engine overlay on every document, remote pages included. A contrast or keyboard fix to it is an engine change for every document, needing a ruling (the scope's P4 boundary).

@history_item n=72 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (his words on the widening) — the founder's F1-F10 and Q1 labels the list's ratified behaviour stands on (F6 «Как у select» for the colours, F5 «Без анимации»). The re-assertion must find any fix inside these words, or else bring the founder a fork.

@history_item n=73 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` leg, locked) — a locked `select_popup_painted` leg with `blitz-paint/font-embolden`, last in `FAST_LEGS`. A painted-list contrast witness interacts with this leg's place in `ci-leg.sh` and its pins in `test_ci_workflows.py`.

@history_item n=74 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears the children's parent (`decision/dropped-anonymous-block-clears-parent`, guarded by `key_press_after_container_lost_its_box`). A focused control hidden by `hidden`, `display:none` or CSS keeps focus and takes keys, which is the F1 A limit. "This plan writes no row on the limit"; the answer card placed it in a11y-plan §5. This is the CARRY (`cry-c2zq6evs`) the chunk pins as a known SC 2.4.3 gap and must not remove.

@history_item n=75 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — the windowed AT witness's owner sentences re-pointed to residual `res-llkgqygv`, the residual the scope names as out of scope.

@history_item n=76 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the windowed AT witness's residual) — `res-llkgqygv` re-said as re-carried at 0.3.0 (target: next) at its three sites. It confirms the AT witness stays residual during this chunk.

@history_item n=77 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `NodeState.clickable`, a closed bool true where listeners hold `click` and the element is no disabled form control (`contract/snapshot-text/clickable`, `contract/diff-node/clickable`). A further reader of the presence reading the SC 2.1.1 sweep can use to find actionable controls.

@history_item n=78 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — the single crate-private `Readings` (focusable, interactive role, or listeners other than `mounted`) read by `unkeyed_actionable` and `actionable_nodes` (`decision/one-actionable-rule`). This is the ready inventory of "every control an agent or a person can act on" for SC 2.1.1.

@history_item n=79 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's validation and answer lines) — `actions` takes no argument and answers `{"nodes":[…]}` with ids, owner ids, `keyed`, verb and cause words, never a value (`contract/driver-verbs/actions`, `contract/actions-answer/nodes`). A served-page SC 2.1.1 check reading it stays inside this ratified crossing.

@history_item n=80 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged actions answer's size) — Cells' answer is 106,088 bytes under the 1,048,576 bound (`decision/actions-answer-unpaged`, guarded by `the_whole_cells_answer_is_within_the_answer_bound`), with its limit on `res-7cc5bi4k`.

@history_item n=81 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (Input Validation) — the PROVISIONAL mark dropped from `~ contract/actions-answer/keyed`; the field now stands.

@history_item n=82 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `~ decision/driver-session-unix-socket-only`: the socket is unix only while in-process sessions run on linux, windows and macOS. This is the bundled-fonts change the scope says may have moved glyph metrics. Served-task checks run in process on every OS leg, and socket-based ones on unix only.

@history_item n=83 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Error Handling) — `+ decision/reach-error-closed-set`: reach errors are fixed texts holding nothing supplied. Relevant if a check starts a served page in a fixture state.

@history_item n=84 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `+ decision/session-layout-read-in-process`, and `~ decision/no-screen-content-in-logs` naming the layout answer. A layout reading used to place focus or text runs no step and logs nothing.

@history_item n=85 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Input Validation) — `~ decision/mask-holds-on-every-carrier` (except: a caused message holds what its author wrote) and `~ decision/socket-carries-screen-reading` (caused text). These are the current exceptions to the mask a served-page check must expect.

@history_item n=86 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (Input Validation) — `~ decision/mask-holds-on-every-carrier` and `~ decision/socket-carries-screen-reading`: a caused item's `error` field is kept as written. This is the current wording of both picked rows.

@history_item n=87 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Input Validation) — `+ contract/driver-verbs/layout`, `~ decision/session-layout-read-in-process`, `~ decision/socket-carries-screen-reading` naming layout readings. The layout verb is available on every carrier for served-page focus and placement checks with no new verb.

@history_item n=88 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Threat Model Summary) — `~ decision/select-list-crossing`: a list row also crosses as a layout answer and its `select-list` placing word. This is the current crossing of the select list the re-assertion reads.

@history_item n=89 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Input Validation) — `~ contract/driver-session-reply/no-attribute-text`: a reply never carries attribute text except an author's text in a record's message or a caused item's `error`.

@history_item n=90 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (the delegate confirms ratified) — the `error`-in-caused and `layout --id` rows now stand by the founder's own word. The rows above are standing, not provisional.

@history_item n=91 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Dependency Security) — `− decision/free-disk-only-on-stand-repeat`: free-disk now on two jobs, the rule's home being the test plan's CI integration. If the chunk grows `stand-repeat`, the disk rule lives in test-plan, not here.

@history_item n=92 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `~ decision/mask-holds-on-every-carrier` guard gains `stand_masks_on_new_answers`. This is the immediate predecessor: every 0.3.0 requirement names its witness inside `stand-repeat`, pinned by function name, the pattern this chunk follows for its stand-facing witnesses.
