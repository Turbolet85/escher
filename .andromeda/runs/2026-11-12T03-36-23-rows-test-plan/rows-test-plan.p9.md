@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=4
relied: A task polled during render acts on the document as the previous render left it, so a node created in that same render cannot yet be focused.
@row_none n=3 rec=3 type=4
relied: The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it.
@row_none n=4 rec=4 type=9
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=4
relied: The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute.
@row_none n=9 rec=9 type=5
@row_none n=10 rec=10 type=5
@row_none n=11 rec=11 type=5
@row_none n=12 rec=12 type=5
@row_add n=1 row=decision/password-caret-placed-by-mask area=test-scope-summary rec=13
name: Password caret placed by the mask
rule: A click inside a password field holding a value places the caret by the painted mask bullets, so one click at one offset into two values of one length lands on one index whatever their characters, and a double click selects the whole value.
why: Ratified.
guard: password_caret_click
@row_none n=13 rec=14 type=9
@row_none n=14 rec=15 type=5
@row_none n=15 rec=16 type=4
relied: A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good.
@row_none n=16 rec=17 type=5
@row_add n=2 row=decision/eight-refusal-causes-in-order area=test-scope-summary rec=18
name: Eight refusal causes in a fixed order
rule: The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy.
why: Ratified.
guard: the_eight_causes_are_named_in_order
@row_add n=3 row=decision/driver-instance-verbs-tested-by-stand area=test-scope-summary rec=18
name: Driver verbs over a held instance tested by integration
rule: The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts.
why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
@row_none n=17 rec=19 type=5
@row_none n=18 rec=20 type=5
@row_none n=19 rec=21 type=5
@row_add n=4 row=decision/buffered-render-on-dioxus-07-bridge area=test-scope-summary rec=22
name: Buffered render on the Dioxus 0.7 bridge
rule: The Dioxus 0.7 bridge renders into a mutation buffer with the document unborrowed, so a task polled during render can access the document.
why: Upstream's buffered render ported onto the Dioxus 0.7 bridge.
guard: dioxus_runtime_probe
@row_add n=5 row=decision/upstream-buffered-render-test-kept-as-written area=test-scope-summary rec=22
name: Upstream buffered-render test kept as written
rule: Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7.
why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for.
guard: dioxus_runtime_probe
@row_add n=6 row=decision/upstream-mounted-flush-test-left-out area=test-scope-summary rec=23
name: Upstream mounted-flush test left out
rule: Upstream's test of an element removed before its mounted event is flushed is left out of the fork until the Dioxus 0.8 move, so the upstream guard on queued mounted events has no test of its own.
why: On Dioxus 0.7 it reads 1 mounted event where it asserts 0, from Dioxus 0.7's scheduling, not the borrow.
@row_none n=20 rec=24 type=7
