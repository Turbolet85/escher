# Answer card — 2026-11-21-engine-debts · Q1

## The founder's recorded answers
- F1 — a focused control that stops showing keeps focus: «Снимать фокус после вёрстки, без blur (Recommended)» — the forks file's option **A**: after each layout a focused control that no longer shows gives focus up, firing no `blur`. Relayed verbatim by the overseer, 2026-11-21, `inputs#I3` (`../additional/escher-overseer/relays/engine-debts-founder-2026-11-21.md`).
- F2 — which «stops showing» counts: «Тот же набор, что отбрасывает дерево (Recommended)» — option **A**: `display: none` on the control or any container, `visibility: hidden` and `collapse`. Same record, `inputs#I3`.
- At implement — the Tab walk: «Tab пропускает скрытое (Recommended)» — `focus_next_node` and `focus_prev_node` skip a node failing the fixup's own «shows» reading, an engine change for every document; one stated limit stays, a `:focus-within` rule that hides the focused control's own container. Relayed verbatim by the overseer, 2026-11-21, `inputs#I6`.

The relay's own line: «His answers ratify the engine changes of F1 A (with F2 A's set) as the options read.»

## The operations these answers cover (16)
- ops-architecture.md · decision/focus-fixup-without-events · established-decisions · fit: fits type 1 — a second silent clear, after each layout, of a focused node that no longer shows
- ops-architecture.md · contract/dom-events/focus-change · standard-contracts · fit: fits type 3 — the post-layout fixup dispatches no focus event
- ops-architecture.md · contract/event-dispatch/focus-change · standard-contracts · fit: fits type 3 — its closed list of silent focus changes gains the post-layout fixup
- ops-architecture.md · decision/changed-set-write-sites · cross-cutting-patterns · fit: fits type 2 — a focus given up after a layout marks its node
- ops-architecture.md · decision/hit-focus-form-engine-changes · established-decisions · fit: fits type 1 — the tally gains the post-layout fixup and the Tab skip
- ops-architecture.md · decision/anonymous-block-drop-clears-layout-parent · fit: fits type 1 — a key on a boxless control acts once re-focused
- ops-security-plan.md · decision/inert-focus-clear-silent · input-validation · fit: fits type 1 — the post-layout clear is silent too
- ops-security-plan.md · decision/programmatic-focus-events · input-validation · fit: fits type 2 — its except list names the post-layout clear
- ops-security-plan.md · decision/dropped-anonymous-block-clears-parent · error-handling · fit: fits type 4 — a key acts on a re-focused boxless control
- ops-main-design-system.md · decision/boxless-focused-select-keeps-focus · surface · fit: — (retire) — the row falls by the fixup
- ops-main-test-plan.md · decision/key-after-dropped-anonymous-block · test-scope-summary · fit: — (retire) — the row falls by the fixup
- ops-a11y-plan.md · decision/changed-set-marks · a11y-strategy · fit: fits type 1 — post-layout focus loss marks its node
- ops-a11y-plan.md · decision/tab-and-shift-tab-move-focus · keyboard-navigation · fit: fits type 1 — Tab and Shift+Tab skip a node that does not show
- ops-a11y-plan.md · decision/focus-fixup-reads-inert-alone · keyboard-navigation · fit: — (retire, heir decision/focus-fixup-gives-up-unshown-focus) — the SC 2.4.3 gap closes
- ops-a11y-plan.md · decision/focus-fixup-gives-up-unshown-focus · keyboard-navigation · fit: fits type 1 — the fixup at the flush (inert) and after each layout (unshown)
- ops-a11y-plan.md · decision/focus-within-hidden-holder-limit · keyboard-navigation · fit: fits type 1 — the one stated limit of the Tab skip

## Named beside the card, not asked (fit not_master)
- ops-architecture.md · decision/resolve-reconciles-select-list — not_master type 4: resolve's step order is the code's; the standing row stays true.
- ops-test-plan.md · decision/key-after-lost-box-acts-after-refocus — not_master type 4: an acceptance line the code and its test hold.

## The answer
Q1, answered by dialog: «Confirm all 16 (Recommended)». The relay's note, verbatim: «The founder confirmed it himself by dialog in the overseer session, 2026-11-21, his label: «Подтвердить все 16 (Recommended)»; relayed by the overseer.»
