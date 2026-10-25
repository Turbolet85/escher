# a11y · Screen Reader Support — category read

## Carried
- (.andromeda/a11y-plan.md:5) The statement's a11y reach must name the tier-0 target exactly as the plan holds it, and claim no criterion beyond it.
  > **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
- (.andromeda/a11y-plan.md:9) The a11y reach must state that no assistive technology is given a tree by the stand binary, and that the adapter and its windowed AT witness are owed, never claimed.
  > no stand binary builds a platform adapter, so no assistive technology is given a tree by the stand binary — the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next).
- (.andromeda/a11y-plan.md:27) The tree's refresh may be stated as proven headlessly only, its windowed witness at the adapter stated as owed.
  > proven headlessly on the stand in both layout modes, its windowed witness at the platform adapter owed — owned by a `.andromeda/residuals.md` line (target: next).
- (.andromeda/a11y-plan.md:35) A limit the statement must carry: the tree never states a native checkbox or radio checked, its fix and the AT witness owed.
  > nothing else writes `toggled`, so the tree never states a native checkbox or radio checked — that fix and the windowed AT witness owned by `.andromeda/residuals.md` lines (target: next).
- (.andromeda/a11y-plan.md:37) The merged upstream reads (descriptions and the new closed values) must be stated as reaching the platform adapter alone, with what an AT reads of them unmeasured.
  > the description and the new closed values leave only to the platform adapter, which no stand binary builds, so what an assistive technology reads of them is not measured.
- (.andromeda/a11y-plan-detail/screen-reader-support.md:17) The a11y reach may claim the stand's named interactive roles only as far as this assertion reaches — the four lean tasks, both layout modes, the nine roles listed.
  > every interactive-role tree node of the four lean tasks reads a non-empty name — asserted in both layout modes over the nine roles of `INTERACTIVE_ROLES`, Button, TextInput, Slider, ListBox, ListBoxOption, CheckBox, RadioButton, ComboBox and Link
- (.andromeda/a11y-plan-detail/screen-reader-support.md:18) The headless half of the adapter crossing is what the statement may claim as proven: the tree handed the adapter equals the harness's tree.
  > the tree the shell hands the adapter, `Document::accessibility_tree` through `&dyn blitz_dom::Document` (the call `AccessibilityState::update_tree` makes), equals `harness.doc.accessibility_tree()` by id, node for node, with the same focus and root, every `author_id` on it the node's `element_id`, none empty, all digits or holding `0x`
- (.andromeda/a11y-plan.md:188) The stable-id capability, as stated to a consumer, must match this: every element node carries its id as `author_id`, no text-run, root or window node one.
  > In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one; on the stand every interactive-role node of the four lean tasks reads a non-empty name and the tree the shell hands the adapter equals the harness's tree node for node; the roles and names the checks expect are stated once, in the shared module's tables — never a second role or name mapping.
- (.andromeda/a11y-plan.md:190) The statement's description of the snapshot must source each reading as the plan does — role, name, focus and the three states from the tree; `enabled` and `value` from the element; `checked` by its stated rule.
  > The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element, and `enabled` (the presence of `disabled`) and `value` from the element, `checked` from a native checkbox's or radio's state, else a checkable role's `toggled`; its text form, parts, diff, MCP results, node-key help and `screenshot` read that model and nothing else — never a second role or name mapping.
- (.andromeda/a11y-plan-detail/screen-reader-support.md:24) The boolean model's two limits are among the stated limits the statement gathers, in these words' sense.
  > with the two limits of the boolean model: a mixed toggle reads `pressed` false and a grammar or spelling error reads invalid, the node keeping the full value
- (.andromeda/a11y-plan-detail/screen-reader-support.md:31) The statement's limits must agree with the tool's own served help, which already states the boolean model's two limits.
  > the node keys' help, which `help` serves — each key's reading stated by the tool, the boolean model's two limits among them (a mixed toggle reads `pressed` false, a grammar or spelling error reads `invalid` true)
- (.andromeda/a11y-plan-detail/screen-reader-support.md:25) A limit to state: `aria-disabled` moves no snapshot reading, `enabled` reading the element's `disabled` presence alone.
  > so the snapshot reads those three from the element — never the tree's disabled, so `aria-disabled` moves no snapshot reading
- (.andromeda/a11y-plan.md:192) What an MCP client carries onward must state the masks as the plan does — a password's and a file input's value as `MASKED_VALUE`, the typed text in no snapshot field, tree node or painted frame.
  > A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame (blitz-paint paints bullets, and a click places the caret by them), and a file input with a non-empty `value` reads the same marker; the driver refuses a `click` or a `type` as `disabled` where the snapshot's `enabled` reads `Some(false)`, nothing dispatched.
- (.andromeda/a11y-plan-detail/screen-reader-support.md:28) The painted-mask claim stands headlessly; the statement must not claim the windowed paint as run.
  > the windowed paint is not run here
- (.andromeda/a11y-plan-detail/screen-reader-support.md:30) The MCP result's content, as the statement names it, is a reader of the snapshot model and of nothing else, in the two forms this line states.
  > an MCP tool result is a further reader of that model and of nothing else — an accepted `snapshot` result is `to_text`'s output itself for a whole screen and, for a screen read in parts, its JSON line `{"text","part","parts"}` holding that part's text
- (.andromeda/a11y-plan-detail/screen-reader-support.md:11) The select capability, as stated, must match the tree and snapshot readings: options kept as `ListBoxOption` nodes under `display: none`, the snapshot's `value` the chosen option's value.
  > A `select` (role `ComboBox`, `ListBox` for `multiple`) keeps its `option` children in the tree as `ListBoxOption` nodes, under `display: none` too, each `selected` true or false from its chosen state and named by its non-empty `label` attribute, else its text; the snapshot reads a `select`'s `value` as `BaseDocument::select_value` (the chosen option's value, nothing with none chosen) and its options as its children, each with its select's client rect as its `bounds`
- (.andromeda/a11y-plan-detail/screen-reader-support.md:44) A stated limit of the a11y reach: the CRUD rows are not focusable and the tree's focus is the list, the missing `aria-activedescendant` kept unread and pinned.
  > one gap stands open — the rows are not focusable and the tree's focus is the list, never the option: since 2026-10-13-stand-keyboard-harness the focused list's keys move the selection (§5 Per-surface keyboard shortcuts), and the missing `aria-activedescendant` is kept unread on the founder's F2 «Оставить записанным (Рекомендую)» (2026-10-14), pinned by `accessibility_active_descendant_unread` (ratified by the founder, 2026-10-18)
- (.andromeda/a11y-plan-detail/screen-reader-support.md:38) A stated limit: no snapshot field says an element reacts to a click, that reading owed to a residuals line.
  > no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading
- (.andromeda/a11y-plan-detail/screen-reader-support.md:65) The a11y reach must state screen-reader exposure as gated by the `accessibility` feature, as the crates state it.
  > Screen-reader integration is behind blitz's `accessibility` feature via blitz-shell (packages/blitz/Cargo.toml:16); the Dioxus crates state screen-reader exposure only as accesskit support behind the `accessibility` feature
- (.andromeda/a11y-plan.md:213) Unmeasured screen-reader ground the statement must not claim as reach.
  > NOT YET MEASURED — live regions, heading-hierarchy handling and per-surface screen-reader test specs
- (.andromeda/a11y-plan.md:235) The a11y leg the statement cites as proof must match its stated file count.
  > `cargo test --workspace --locked` over fourteen files

## Bindings
- **Masked readings ↔ security-plan §Surfaces**: the masks the statement states for what an MCP client carries onward (a password's and a file input's value as `MASKED_VALUE`, a shot painting a password masked) must agree with the crossing security-plan ratifies; this side (.andromeda/a11y-plan.md:192), (.andromeda/a11y-plan-detail/screen-reader-support.md:30).
- **Boolean model's limits ↔ escher-driver schema help (`help`'s node keys)**: the statement's limits list must match the served help texts the scope closes completeness against at P3; (.andromeda/a11y-plan-detail/screen-reader-support.md:31).
- **AT witness, native checked state, click-reaction field ↔ `.andromeda/residuals.md`**: each limit the statement states as owed names the residuals line that owns it (target: next); (.andromeda/a11y-plan.md:9), (.andromeda/a11y-plan.md:35), (.andromeda/a11y-plan-detail/screen-reader-support.md:38).
- **`aria-activedescendant` gap ↔ a11y-plan §5 Per-surface keyboard shortcuts (keyboard-navigation category)**: one gap stated once, both sides agreeing it is pinned unread; (.andromeda/a11y-plan-detail/screen-reader-support.md:44).
- **a11y leg's fourteen files ↔ a11y-plan §9 and test-plan §9**: the count the statement cites as the a11y reach's proof; (.andromeda/a11y-plan.md:235).
