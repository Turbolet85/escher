@carried n=1 label=a11y-plan/a11y-strategy fact=fct-p7eqwwyx
why: the probe must re-verify the hazard on this same poll path and read the tree through the trait method so that DioxusDocument's override, which carries the stable ids, is what it reads.
quote: The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: on a poll that
  reported work the shell's `View::poll` takes the document's changed set (`BaseDocument::take_changed_nodes` — outside
  the `accessibility` cfg, so a shell built without the feature still drains it) and, under `accessibility`, rebuilds
  the platform tree when the set it took was non-empty; both paths go through the `Document::accessibility_tree` trait
  method, so a wrapper's override (DioxusDocument's stable ids) reaches the platform tree
@carried n=1 run=2 label=a11y-plan fact=fct-zyrpeqlm
quote: a poll that takes a non-empty changed set rebuilds the platform tree, both paths through the
  `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches it; proven
  headlessly on the stand in both layout modes, its windowed witness at the platform adapter owed
@carried n=2 label=a11y-plan/a11y-strategy fact=fct-3hktsq24
why: link (4) uses the changed set as its witness, so the probe must drain the set before the step it reads, and must not count node creation, hover or layout as state reaching a node.
quote: A harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the set, so a step
  settled through it leaves the refresh above, and a later diff, exactly what the step marked; and the set is non-empty
  on a freshly booted harness document — the boot's own in-document mutations mark it and nothing has drained it — so a
  check that reads `has_changes()` as "this step changed something" drains first
@carried n=2 run=2 label=a11y-plan fact=fct-xhpyuh5z
quote: `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: an
  in-document mutation, a focus or checked change, a text control's input, a select's changed choice and a single
  select's list opening or closing mark it — a page scroll or a move that closes a list marks its select through
  `reconcile_select_list` at the next resolve, a wheel elsewhere through `close_select_list`; node creation, hover,
  active, scroll and layout do not, save that scroll or move, and a highlight or a wheel move inside a list marks
  nothing, so a pointer move alone reaches no refresh
@carried n=3 label=a11y-plan/a11y-strategy fact=fct-d5bs73pt
why: the probe reads through the override, the snapshot and unkeyed_actionable, so its code must be built with dioxus-native-dom's accessibility feature, which tests/blitz-tests already names, and must add no feature or crate to get them.
quote: dioxus-native-dom's feature also gates its own `accessibility_tree` override, its snapshot model
  (`DioxusDocument::snapshot`), that model's text form (`Snapshot::to_text`) and its parts (`Snapshot::text_parts`), its
  diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all five of which its
  crate doc names
@carried n=3 run=2 label=a11y-plan/a11y-scope-summary fact=fct-ip333e77
quote: **blitz-tests** — `blitz-dom` and `dioxus-native-dom` are built with the `accessibility` feature and `accesskit`
  is a dev-dependency
@carried n=3 run=3 label=a11y-plan/a11y-scope-summary fact=fct-lszvr7oi
quote: under the `accessibility` feature, `BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM
@carried n=3 run=4 label=a11y-plan fact=fct-f4s2gocd
quote: The test harness builds blitz-dom with the accessibility feature
@carried n=4 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
why: any probe code kept in examples/seven_guis reaches the accessibility modules only through escher-driver's edge, and it must build no platform adapter, so the windowed and assistive-technology readings stay read and not run.
quote: the session library `escher-driver`, which seven_guis depends on since 2026-10-07-driver-session, names
  `accessibility` on `dioxus-native-dom` since 2026-10-07-act-by-id — unconditionally, with no `[features]` table of its
  own: its executor reads the snapshot, which stands behind that feature — so the feature reaches seven_guis through
  that edge although seven_guis' own manifest still names none, and both its binaries, the windowed `seven_guis_native`
  and the `escher-session` host, now compile dioxus-native-dom's accessibility modules (the `accessibility_tree`
  override that carries `author_id`, the snapshot model, its text and its diff); neither builds a platform adapter
@carried n=4 run=2 label=a11y-plan fact=fct-kbjv4usk
quote: it reaches seven_guis through `escher-driver`, which names it unconditionally, while seven_guis' own manifest
  names none, and no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand
  binary
@carried n=5 label=a11y-plan/a11y-scope-summary fact=fct-lxyhqmtz
why: the served task the probe runs beside the lean one must be one of these two pages, booted through its stand entry and served by escher-session, its nodes already read keyed so they can be addressed by author_id.
quote: two pages the stand serves stand beside the four lean tasks, Cells (`seven_guis::stand::boot_cells`, mounting
  `Task::Cells` in `TaskShell`) and the load page (`seven_guis::stand::boot_load`, its `load-fetch` Button and
  `load-done` Paragraph), both served by `escher-session`; neither is a `LeanTask`; each holds a stated Tab sequence
  (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's
  `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`, and
  their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0
@carried n=6 label=a11y-plan/a11y-scope-summary fact=fct-fichvpza
why: if the plan adds a probe check to the a11y leg, the capability statement's count of that leg's files must be changed by hand in that document, since no pin reads it; and the statement of what the accessibility layer reaches must not change.
quote: `docs/capabilities.md` («What Escher can do») publishes the accessibility reach under `## Accessibility reach`:
  tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion, the `a11y` leg's fourteen files with no required status
  check, what an assistive technology reads unmeasured, and the named limits (the focus fixup firing no `blur`, a single
  select's `size` unread, `aria-activedescendant` unread); its pin `.github/scripts/test_capability_statement.py` reads
  none of these claims, so a change to the reach is made in that document by hand
@carried n=7 label=a11y-plan/a11y-strategy fact=fct-ngdl2hcv
why: when the probe measures links (3) and (4) on the CRUD task, a row's selection is read in the tree as `selected` from its `aria-selected`, so that is the value the reading nodes carry, and the probe must read it from the tree and not from the attribute.
quote: each value trimmed of ASCII whitespace and compared exactly, in lower case: `aria-selected` `true` / `false`
  sets `selected`, anything else nothing
@carried n=7 run=2 label=a11y-plan/aria-patterns-roles fact=fct-wcjja6re
quote: `aria-selected` → `selected`, `aria-pressed` → `toggled` (a toggle button; `mixed` kept as Mixed),
  `aria-invalid` → `invalid` (True / Grammar / Spelling; a value outside the list reads True), read by value on any
  element
@carried n=7 run=3 label=a11y-plan fact=fct-cixbakze
quote: The builder writes three states from ARIA state attributes, trimmed and matched in lower case: `aria-selected` →
  `selected`
@carried n=7 run=4 label=a11y-plan fact=fct-v47qtadu
quote: `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` map to `selected`, `toggled` and `invalid` as
  §2 Role derivation states
@carried n=8 label=a11y-plan fact=fct-zvvhy2hs
why: the probe must drive the hazard's key handler, and the Counter or CRUD handlers, headlessly through the harness's `press` / `press_with`, the same path the stand's keyboard checks drive, and add no key path of its own.
quote: a Tab and Shift+Tab key-press focus check exists on the stand, driven through the test harness's `press` /
  `press_with` (tests/blitz-tests/tests/stand_snapshot_state.rs:331-367), and since 2026-10-13-stand-keyboard-harness
  `stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks
  through the same `press` / `press_with`, reading the focused node back per step
@carried n=9 label=a11y-plan fact=fct-4abpljjc
why: the probe must print, log or put in a field nothing it reads, and it must leave this allowlist as it is, adding no target, field or span for it to judge.
quote: a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` /
  `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's
  own targets (`escher_*`) and drops a record from any other target whole, at every level
@binding n=1
**a11y-strategy tree refresh ↔ a11y-strategy changed set ↔ the scope's MountedData hazard**: the poll that takes a non-empty changed set and rebuilds the tree through `Document::accessibility_tree` (a11y-plan/a11y-strategy fct-p7eqwwyx) is the same `poll` in which the key-spawned task is first polled inside `render_immediate`. So the probe's link-(4) reading of the changed set, drained first (a11y-plan/a11y-strategy fct-3hktsq24), and its re-verification of the hazard must both run on that path, headlessly. The windowed shell's copy of those lines is read, not run.
@binding n=2
**a11y-scope-summary feature reach ↔ a11y-strategy feature gating ↔ the probe's code home**: probe code in `tests/blitz-tests` reads the override, the snapshot and `unkeyed_actionable` under the feature that crate already names (a11y-plan/a11y-scope-summary fct-ip333e77; a11y-plan/a11y-strategy fct-d5bs73pt). Probe code in `examples/seven_guis` reaches them only through `escher-driver`'s unconditional edge (a11y-plan/a11y-scope-summary fct-r6zswm2d). Neither home may add a feature, a crate or a platform adapter.
@binding n=3
**a11y-scope-summary served pages ↔ screen-reader-support actionable-key rule**: the served task the probe runs beside the lean one (a11y-plan/a11y-scope-summary fct-lxyhqmtz) must address its nodes by the author key that `unkeyed_actionable` already finds on every stand task (a11y-plan fct-77ewqepn), and the probe must add no new key.
@binding n=4
**a11y-scope-summary capability statement ↔ ci-integration a11y leg**: a probe check the plan adds to the `a11y` leg must join that leg's named file list and count (a11y-plan fct-6kcxmj7u), and the same count must change by hand in `docs/capabilities.md` (a11y-plan/a11y-scope-summary fct-fichvpza).
@binding n=5
**aria-patterns-roles ARIA states ↔ a11y-strategy changed set**: on the CRUD task, a row's click or the list's arrow key rewrites a row's `aria-selected`, which the tree reads as `selected` (a11y-plan/a11y-strategy fct-ngdl2hcv; a11y-plan/aria-patterns-roles fct-wcjja6re). That rewrite is an in-document mutation that marks the changed set (a11y-plan fct-xhpyuh5z), so it is the handler-written state that links (3) and (4) must witness.
@binding n=6
**the a11y body's keyboard harness ↔ a11y-strategy tree refresh**: the key handler that spawns the hazard's task must be reached through `press` / `press_with` (a11y-plan fct-zvvhy2hs), and the poll that follows is the refresh path (a11y-plan/a11y-strategy fct-p7eqwwyx) where the panic is expected.
@binding n=7
**the a11y body's log allowlist ↔ the scope's «Nothing crosses» boundary**: escher's redaction for `escher_*` targets (a11y-plan fct-4abpljjc) must stay as it is. The probe must give it nothing to judge: no target, field or span, and no value it reads printed.
