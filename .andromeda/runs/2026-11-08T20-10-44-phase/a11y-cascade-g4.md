@carried n=1 label=a11y-plan/visual-design-verification fact=fct-anrtdftj
why: The coverage record must count a handler exercised only where a replayed step dispatched to it, so a click swallowed as a thumb drag in the shown scrollbar band leaves its target's handlers unexercised.
quote: Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`hit_with_scrollbar` while `scrollbar_opacity > 0`, `packages/blitz-dom/src/events/pointer.rs`)
@carried n=2 label=a11y-plan/visual-design-verification fact=fct-h2b6iawo
why: A replayed driver click refused `covered` reaches no handler, so the coverage list must keep the covered element's handler among the unexercised ones rather than keying exercise on the step's named target.
quote: a driver `click` on an element under the list is refused `covered`
@binding n=1
**visual-visibility ↔ screen-reader-support (the driver's disabled refusal)**: a step that reaches no handler — swallowed in the scrollbar band (a11y-plan/visual-design-verification fct-anrtdftj), refused `covered` (a11y-plan/visual-design-verification fct-h2b6iawo) or refused `disabled` (a11y-plan fct-2hrkecxm) — must read as exercising nothing, the coverage record keyed on the (graph node id, event) pairs actually dispatched, never on the steps issued.
