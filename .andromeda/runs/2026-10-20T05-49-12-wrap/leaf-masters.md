group: masters
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: retired
  leaf-text: «the in-process visual lint (PROVISIONAL: in process only; no verb»
  source: .andromeda/architecture.md:202 «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to»
  replace-with: «the in-process visual lint (in process only — its `lint` verb, on every carrier and recorded, owed to Visual lint's gated part; no verb»

- leaf: CLAUDE.md:81   class: retired
  leaf-text: «(`Session::lint`, PROVISIONAL; unit tests inline)»
  source: .andromeda/architecture.md:202 «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to»
  replace-with: «(`Session::lint`, in process only; unit tests inline)»

- leaf: CLAUDE.md:110   class: retired
  leaf-text: «and the visual lint (built in process only, PROVISIONAL —»
  source: .andromeda/architecture.md:202 «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to»
  replace-with: «and the visual lint (built in process only, its `lint` verb on every carrier and recorded owed to Visual lint's gated part —»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «`aria-invalid`, read by value on every element;»
  source: .andromeda/architecture.md:86 «from `aria-selected`, `aria-pressed` and `aria-invalid`, read by value (a select's option excepted, its `selected` its chosen state, [DOM semantics])»
  replace-with: «`aria-invalid`, read by value on every element but a select's option, whose `selected` is its chosen state with no `aria-selected` read and which stays in the tree though it computes `display: none`;»

- leaf: CLAUDE.md:22   class: stale
  leaf-text: «typed text and a range input's moved value (`apply_range_move`, nothing for a move that changes nothing), not by node creation, hover or layout)»
  source: .andromeda/architecture.md:294 «and `BaseDocument::apply_select_choice`, which marks a select where a key or typeahead changed its chosen option and nothing for a choice of the chosen option»
  replace-with: «typed text, a range input's moved value (`apply_range_move`, nothing for a move that changes nothing) and a select's changed choice (`apply_select_choice`, nothing for a choice of the chosen option), not by node creation, hover or layout)»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «ratified by the founder, 2026-10-13), keyboard activation and a button click's focus»
  source: .andromeda/architecture.md:92 «- The select model stands beside them, a direct engine change for every document, remote pages included: a focused single-choice `select` (no `multiple`) that is no disabled form control takes»
  replace-with: «ratified by the founder, 2026-10-13), a single-choice select's model (its rules in the crate-private `node/select.rs`: the choice held as its options' `CHECKED` state, seeded from `selected` and settled by HTML's selectedness setting algorithm; focused and no disabled form control, `ArrowDown` · `ArrowUp` · `Home` · `End` take it to the next · previous · first · last enabled option and a character with no Ctrl, Alt or Meta is typeahead, its buffer starting afresh after 1 s of the document's clock; a click focuses it; each change goes through `apply_select_choice`, one `input` then one `change` (`DomEventKind::Change`), a choice of the chosen option firing none; `BaseDocument::select_value` its public reader; no popup painted, an option not clicked, a `multiple` select and an `optgroup`'s options not modelled — a direct engine change for every document), keyboard activation and a button click's focus»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «a range input's its current value `range_input_value()`»
  source: .andromeda/architecture.md:214 «the snapshot's `value` reader takes the document and the node and reads a `select` through `BaseDocument::select_value`, no mask applying»
  replace-with: «a select's its chosen option's value (`BaseDocument::select_value`, no mask applying), its options read as its children with its client rect as their `bounds`, named by their labels, a range input's its current value `range_input_value()`»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`type` replacing what the control holds (an empty text clears it)»
  source: .andromeda/architecture.md:196 «`type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it»
  replace-with: «`type` replacing what the control holds (an empty text clears it; on a select the text chooses the first option whose label starts with it)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «`type` replaces what its control holds»
  source: .andromeda/architecture.md:196 «`type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it»
  replace-with: «`type` replaces what its control holds (on a select it chooses the first option whose label starts with the text)»

- leaf: CLAUDE.md:75   class: omission
  leaf-text: «range input `packages/blitz-dom/src/node/range.rs` (unit tests inline)»
  source: .andromeda/architecture.md:179 «- The crate-private module `node::select` holds the select's rules — `OptionReading`, `SelectStep` (`Next` · `Previous` · `First` · `Last`), `Typeahead` and `TYPEAHEAD_TIMEOUT` (1 s)»
  replace-with: «range input `packages/blitz-dom/src/node/range.rs` (unit tests inline) · the select's rules `packages/blitz-dom/src/node/select.rs` (unit tests inline; checks `tests/blitz-tests/tests/{select_interaction,stand_act_select}.rs`)»
