@row_none n=1 rec=1 type=4
relied: Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state.
@row_none n=2 rec=2 type=4
relied: An open select's list is no `:open` state, so no author style keys on a list standing open.
@row_none n=3 rec=3 type=4
@row_none n=4 rec=4 type=4
@row_none n=5 rec=5 type=0
what: a note on a code comment in an example about an input's color
@row_none n=6 rec=6 type=4
relied: The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed.
@row_none n=7 rec=7 type=4
relied: A color-scheme change on the viewport is a device change that rebuilds the stylist device.
@row_none n=8 rec=8 type=4
relied: A total scale change from hidpi or zoom invalidates text shaping.
@row_none n=9 rec=9 type=4
@row_none n=10 rec=10 type=4
@row_none n=11 rec=11 type=5
@row_none n=12 rec=12 type=4
@row_none n=13 rec=13 type=4
@row_add n=1 row=decision/hidden-content-excluded-from-a11y-tree area=visual-design-verification rec=14
name: Hidden content excluded from the accessibility tree
rule: Nodes with `display: none`, `visibility: hidden` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot.
except: A select's options are excepted from `display: none`, so they stand under the select in the tree and the snapshot, directly or through an `optgroup`.
@row_add n=2 row=decision/inert-element-takes-no-hit area=visual-design-verification rec=14
name: An inert element takes no hit
rule: A hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit.
why: The `inert` read was ratified.
guard: inert_subtree
@row_none n=14 rec=15 type=4
@row_none n=15 rec=16 type=4
@row_none n=16 rec=17 type=4
@row_none n=17 rec=18 type=4
@row_none n=18 rec=19 type=4
relied: Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first.
@row_add n=3 row=contract/driver-click-refusals/covered area=visual-design-verification rec=20
boundary: Driver click refusals
item: covered
promise: A driver click on an element under an open select list is refused `covered`.
@row_none n=19 rec=21 type=4
@row_none n=20 rec=22 type=5
@row_none n=21 rec=23 type=8
@row_add n=4 row=decision/a11y-ci-gate area=bootstrap-phases-derive-for-route-setup-project rec=24
name: Accessibility checks gate CI
rule: Accessibility checks run in CI as the `a11y` job.
guard: a11y
@row_none n=22 rec=25 type=S
