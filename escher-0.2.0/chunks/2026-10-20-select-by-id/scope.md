# Scope — 2026-10-20-select-by-id · Select by id

Working entry (escher-0.2.0/working-route.md, Epoch 4 — Visual lint and real-app readiness), verbatim:
«Select by id — a `select` driven by id like any control, read in the snapshot with role and name, Tab order kept;
fixture-proven (v020-08)»

Capability: v020-08 (requirement: a select is driven by id like any control and read in the snapshot, proven on a
fixture — intent §6 «Ready for a real app»; residual: no `select` interaction model, the founder 2026-10-09,
«Остаток до первой нужды», `.andromeda/residuals.md` line 11, absorbed:v020-08). Matrix acceptance as routed: on a
fixture holding a select, an agent chooses an option by the control's stable id through the driver, and the snapshot
reads the select with its role, its name and the chosen value, the focus order unchanged.

Freight on the entry: none (no CARRY / PREREQ / CONTEXT / WATCH / BLOCKED-ON at an annotation position).

## What it builds
- A `select` on a page an agent drives — the control addressed by its stable element id, as `click`, `type` and
  `scroll` address any control — so that the agent chooses one of its options through the driver.
- The snapshot reads the select: its role, its accessible name and its chosen value.
- The focus order is unchanged: the select takes its place in Tab order as a form control, and a page holding one
  keeps the rest of its Tab sequence.
- Proof on a fixture page holding a select (fixture-proven), in both layout modes.

## Closed at P3 (research.md)
- [premise-corrected: measured in both layout modes, research.md §Measured — a select has no box, so the snapshot
  and every driver verb miss it] The engine has no `select` model, and worse than the residual's hypothesis: a
  `select` computes `display: inline` with every `option` `display: none`, lays out 0×0 and reads no client rect, so
  the snapshot leaves it out and `click`, `type` and `screenshot` on its id are refused `not-found`. The tree holds it as
  an empty `ComboBox` (no options, no value); a `<label>` names nothing but an `input`, `aria-label` names it. It is
  focusable and takes its Tab stop in document order, and it takes no key. Before an agent can drive it, it needs a
  box (or the fixture must size it), a chosen option held by the engine, a value the snapshot reads and a way to
  choose.
- How the agent chooses an option — `type` (typeahead), `press` (arrows · `home` · `end`), a new verb, or a pair — and
  whether the engine gains a select's box, key, typeahead or label default actions for every document, are the
  founder's forks, not facts research can settle: each widens the boundary or changes the engine for every document.
  They go into the run dir's `founder-forks.md`, the most conservative option marked, and are asked of the operator by
  dialog (inputs#I1).
- [premise-corrected: the snapshot reads no value for a `select` (`value()`, snapshot.rs:238-262, matches `textarea`
  and `input` alone) and the tree holds no option, so no option reads `selected`] The value the snapshot reads for a
  select is new: the chosen option's `value` (HTML's `select.value`: its `value` attribute, else its text), or its
  label, is a founder's fork beside whether the options themselves are read.
- [premise-corrected: the module holds a page only when two checks read it (testing.md §Fixtures; test-plan §3)] The
  select fixture page lives in its one check's file unless a second check reads it.
- The only form event a select change can reach an app by is `input` (no `change` kind in blitz-traits), as the range
  model's change does.

## Decided at P4 (inputs#I2)
The operator answered the four founder forks as delegate, PROVISIONAL until the founder's own word (discharged in the
Epoch 4 boundary batch), built now and not gated: F1 «Keys + typeahead» — on a focused select the arrows, Home and End
and a typed prefix choose, and a click on a select focuses it, in every document, an agent choosing through `type` or
`click` + `press` with no new verb, argument or carrier; F2 «Engine shows its label» — a select lays out as an
inline block holding its chosen option's label; F3 «Options as children» — the options are read in the tree and the
snapshot, named by their labels, the chosen one `selected`; F4 «`change` event» and «`<label>` names it» — a choice
fires `input` then `change`, and a `<label>` names every labelable element HTML lists.

## Boundaries
- A single-choice `select` (no `multiple`), the case the requirement names; a `multiple` select or an `optgroup` is
  in only where it costs nothing extra, else a stated limit.
- No dropdown popup is painted or windowed; the proof is headless.
- No stand task gains a `select` unless the plan needs one; the 7GUIs stand stays as built.
- Load wait is done (2026-10-20-load-wait); the large screen in parts is the next entry, not this chunk.

## CI read at take-up (Setup 5a)
- 2f0997354972 (2026-10-20-load-wait, the last wrap's flip and HEAD): CI#39365557179 in progress — verdict not yet
  available (17/17 checks, 12 running, the oldest `Coverage report` 26 s, at 2026-10-20T03:47Z). Not folded, not read
  as green.
