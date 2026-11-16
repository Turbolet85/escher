@row_add n=1 row=contract/dioxus-document/take-step-reached area=keyboard-navigation fact=F1 detector=none
boundary: Dioxus document reached-handler record
item: take_step_reached
promise: Drains the per-step list the reached-handler record keeps: each handler reached since the last drain, once, in first-reached order; empty when the record is off.
basis: fct-xxn4lhqe
because: The reached-handler record gained a per-step list and its drain, a new promise of the Dioxus document that the driver's acting step relies on.
@row_add n=2 row=contract/dioxus-document/root-error-mark area=keyboard-navigation fact=F2 detector=none
boundary: Dioxus document root error boundary
item: root_error_mark
promise: Reads the root error boundary's mark: a new mark after a handler's Err the root default boundary takes, none where the app's own error boundary catches it.
basis: fct-xxn4lhqe
because: The Dioxus document gained a read of the root error boundary's mark that tells a new handler Err from none, a new promise at its boundary.
@row_add n=3 row=contract/acting-answer/caused area=screen-reader-support fact=F3 detector=none
boundary: the acting answer
item: caused
promise: An acting call's answer carries caused only when its step caused something: one item per kind (panic, error, log) and level, with the first reached handler's id, owner and event and a count; no message, payload or error text; Dioxus framework targets counted at warn and error alone.
basis: fct-vzyy52tx, fct-qepwv2ae
because: The five acting verbs' answers gained the caused field on every carrier, a new outward promise of what an answer holds beside the snapshot's readings.
@row_add n=4 row=decision/root-boundary-error-on-screen area=screen-reader-support fact=F4 detector=none
name: Root boundary error read as screen content
rule: A handler Err the root default error boundary takes replaces the app with the boundary's rendering of the error's Display, read by the snapshot as ordinary screen content, while caused holds only an error item and no error text.
why: Ratified narrowing: the root boundary replacing the app, its rendering crosses as existing screen content, never through caused.
basis: fct-pxx4n4tk, fct-nnu5onnh
because: The chunk ruled how a root-boundary Err reaches an agent, so the screen a reader gets after such a step is a decision future work must respect.
