# Founder forks — 2026-10-20-select-by-id (phase P4)

Chunk: Select by id (v020-08) — escher-0.2.0/chunks/2026-10-20-select-by-id/scope.md. Gathered per the operator's
directive (inputs#I1): every fork that widens a boundary (a new driver command, verb, argument, exit or log field, an
engine behaviour change for every document) or turns the project's direction, each with its options in order, the
recommendation marked and the most conservative option marked. Playbook patterns each touches: «Boundary widening»
(verdict: escalate) for a new verb or argument; «An engine behaviour change for every document» (routine only on the
founder's own word at this fork; a delegate's answer leaves it PROVISIONAL, «Provisional discharge»).

What the measurement found (research.md §Measured, evidence/select-probe-readings.txt — both layout modes alike): a
`select` today lays out as nothing. It computes `display: inline` with every `option` `display: none`, reads no client
rect, and so is left out of the snapshot; `click`, `type` and `screenshot` on its id are refused `not-found`. The tree
holds it as an empty `ComboBox` (no options, no value); a `<label>` names nothing but an `input`, while `aria-label`
names it. It takes its Tab stop in document order and takes no key. So v020-08 needs four things the engine lacks: a
box, a chosen option it holds, a value the snapshot reads, and a way for an agent to choose.

Decided without a fork (leans an artifact settles, stated here so the whole picture is on one page):
- The chosen option is held by the engine apart from the markup, seeded from the `selected` attribute (else the first
  option that is not disabled), re-seeded when the app writes an option's `selected` or adds or removes options — the
  range model's shape (2026-10-13-range-input-interaction), cited to the WHATWG `select` element.
- The snapshot's `value` for a select is HTML's `select.value` — the chosen option's `value` attribute, else its text —
  in the existing `value` key, inside the control-value crossing the founder ratified on 2026-10-10 (a text input's
  author-written `value` attribute already crosses there). No new key.
- Single-choice selects only; a `multiple` select and `optgroup` stay a stated limit (scope §Boundaries). A disabled
  option is never chosen by a key. A disabled select takes no key and no click (`ElementData::is_disabled`).

## F1 — How an agent chooses an option

The engine has no select keys today. Every option below makes a click on a select focus it, as a click on a button does
(today a click on any other element clears focus), in every document.

1. **Keys and typeahead, through the verbs there are — (Recommended)**
   On a focused select, in every document: ArrowDown / ArrowUp choose the next / previous option that is not disabled,
   Home / End the first / last; a typed character chooses the first option whose label starts with the text typed so
   far (case-insensitive, as browsers do; the typed text starts afresh at a click on the select and after 1 s of the
   document's clock). An agent chooses with `type --id <select> --text <label>` in one call — `type` already clicks its
   target and then types — or with `click --id` then `press arrow-down`. Each change fires the app's events (F4). No new
   verb, argument, key name or exit; `type`'s help gains one fixed sentence for a select.
   Why: one call by the select's own id, the way an agent already sets a text field; keyboard users get the browser's
   keys (SC 2.1.1); nothing new crosses the driver's surface.
2. **Keys only, through `press` — (Most conservative)**
   As 1 without typeahead: an agent focuses the select by `click --id` and steps with `press arrow-down` / `arrow-up` /
   `home` / `end`, one option per call, reading the value back each time. No change to `type`.
   Trade-off: choosing the fifth option takes five calls, and `type` on a select types nothing useful.
3. **A new instance verb `choose --id <select> --option <label>`**
   The twelfth verb, on every carrier (socket, command line, MCP, `help`), recorded in a kept check; it chooses the
   option by its label directly, and is refused `not-found` (an existing cause) when no option has that label. The
   engine still gains the held choice; keys are left out unless also chosen.
   Trade-off: a new verb and a new argument kind widen the validated surface (as `scroll` and `screenshot` did) and
   move every count site of the verb table.

## F2 — The select's box

1. **The engine lays a select out as an inline block showing its chosen option's label — (Recommended)**
   The user-agent stylesheet gives `select` `display: inline-block` with a border and padding like `input`'s, and
   layout gives it the chosen option's label as its content, so every select on every page — remote ones included —
   takes a size, reads in the snapshot and paints its value (no arrow glyph, no popup).
   Why: a real app's unstyled select becomes visible to the agent and on screen, as in a browser.
2. **The engine gives a select a box but paints no label**
   `display: inline-block` and a minimum size in the stylesheet only; the select is clickable and read in the snapshot,
   but its value shows nowhere on screen.
3. **No engine layout change: the fixture sizes its select in its own CSS — (Most conservative)**
   The fixture page sets the select's display, width and height; an unstyled select on a real page still has no box and
   stays out of the snapshot — a stated limit until a later chunk.

## F3 — The options in the tree and the snapshot

1. **Each option read as the select's child, named by its label, the chosen one `selected` — (Recommended)**
   The accessibility builder keeps a select's options (today dropped as `display: none`) as `ListBoxOption` children,
   in every document; the snapshot lists them under the select by their own stable ids, each named by its label, the
   chosen one `selected`, its bounds the select's box. An option's label crosses as a name, inside the ratified name
   crossing; `selected` is the existing closed reading. An option is not a Tab stop and is not clicked (a click on one
   is refused as any element under another).
   Why: an agent sees the choices it can make before it makes one.
2. **The select's value alone — no option read — (Most conservative)**
   The snapshot reads the select and its chosen value; the other options are not read, so an agent learns them only by
   stepping through with keys.

## F4 — What else a real app reads from a select (choose any)

1. **A `change` event after `input` — (Recommended)**
   Each change fires `input` then `change`, as HTML does: blitz-traits gains a `Change` event kind and the Dioxus bridge
   hands it to `onchange`, in every document. Without it an app's `onchange` on a select never runs (today only
   `oninput` can be reached, as the range model's change is).
2. **A `<label>` names a select (HTML's labelable elements) — (Recommended)**
   `label_bound_input_element` binds a `<label>` to every labelable element HTML names (`button`, `input` not hidden,
   `meter`, `output`, `progress`, `select`, `textarea`), in every document; the label then names it in the tree, and a
   click on the label focuses it (the same function serves `handle_click`).
3. **Neither — `input` alone, and a select named by `aria-label` / `aria-labelledby` — (Most conservative)**

## Answers (the operator, by question dialog, read 2026-10-20T04:00Z — delegate operator, PROVISIONAL)

- F1: «Keys + typeahead (Recommended)»
- F2: «Engine shows its label (Recommended)»
- F3: «Options as children (Recommended)»
- F4: «`change` event (Recommended)», «`<label>` names it (Recommended)»
- The operator's note, verbatim: «Delegate answers while the founder is away, PROVISIONAL until his word in
  founder-forks.md (an engine behaviour change for every document, as the :focus-within take): no new verb, argument or
  carrier — option labels inside the ratified name crossing.»
- Authority: delegate operator (the founder away). Every answer is an engine behaviour change for every document,
  PROVISIONAL until the founder's own word on this file, discharged in the Epoch 4 boundary batch («Provisional
  discharge»); built now, not gated — as the carried `:focus-within` take was.
