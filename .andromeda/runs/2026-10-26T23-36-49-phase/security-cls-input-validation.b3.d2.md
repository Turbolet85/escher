# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · no · the `<ol start>` parse is engine list numbering; the lint's six defects read bounds, text extents and contrast, not list numbers, and the chunk adds no such read
input-validation#1.52 · no · `attr_parsed` is the engine's attribute-parse helper behind `tabindex`; the chunk's helpers parse no attribute
input-validation#1.53 · no · `disabled` by presence is engine focus, pointer and key gating, which the task lists as not needed; the lint's helpers read no disabled state, dispatch no click and leave `Session::run`'s refusals alone
input-validation#1.54 · no · `autofocus` moves focus at mount in the engine; the lint reads no focus and the chunk dispatches nothing that would move it
input-validation#1.55 · no · the `dir` match is an engine parse; the lint reads bounds and text extents, and the chunk adds no direction read
input-validation#1.56 · no · the canvas `src` u64 parse is an engine texture read; the chunk's three functions reach no canvas
input-validation#1.57 · yes · `Finding` carries the stable id, so every helper split out of `findings` must carry only ids, closed words and numbers, and `Session::lint` feeds the private id record from its own snapshot
input-validation#1.58 · no · standing S3 · escher's own crates log only through tracing with fixed words, bools and counts, never an accessible name
input-validation#1.59 · no · an accessible description reaches only the platform adapter; the snapshot and the lint read no description and the chunk touches neither
input-validation#1.60 · no · `aria-expanded`, `aria-disabled` and `aria-level` are engine tree reads, which the task lists as not needed; the snapshot's `enabled` keeps its presence reading and the lint reads no state
input-validation#1.61 · no · the ARIA control states are engine tree and snapshot reads, which the task lists as not needed; the lint's six defects read bounds, text extents and contrast, not states
input-validation#1.62 · no · `inert` is an engine hit, focus and tree rule, which the task lists as not needed; the chunk's walk reads the snapshot's nodes as given and adds no `inert` read
input-validation#1.63 · no · the range model and its key, pointer and click defaults are engine rules, which the task lists as not needed; the lint reads no range value and dispatches no key or pointer
input-validation#1.64 · no · Enter and Space activation and click focus are engine default actions, which the task lists as not needed; the chunk dispatches no key or click and the lint runs no step
