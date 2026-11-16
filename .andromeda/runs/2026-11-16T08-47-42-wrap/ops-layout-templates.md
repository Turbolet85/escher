@row_add n=1 row=contract/escher-session-acting-answer/caused area=surface-cli fact=F1 detector=none
boundary: escher-session acting answer
item: caused
promise: On click, type, press, advance and scroll, written last and only when non-empty: one item per kind (panic, error, log) and level (error, warn, info, debug, trace) with the first reached handler's id, owner and event where known and a count; no message, payload or Err text crosses.
basis: fct-vzyy52tx
because: The five acting verbs' answers gain the caused field with a closed item shape and stated limits, a promise outward that no row of the CLI surface yet carries.
@row_add n=2 row=decision/framework-log-targets-warn-and-error area=surface-cli fact=F2 detector=none
name: Framework log targets counted at warn and error
rule: Log records from the Dioxus framework targets (dioxus, generational_box, warnings) count in an acting answer's caused only at warn and error; every other target outside the engine and escher prefixes counts at every level.
why: The Dioxus framework traces on every re-render and signal write, so counting its every level would put caused on every click and move the committed checks.
guard: stand_act_errors
basis: fct-noe4u5rr
because: The capture's counting of framework targets was ruled at warn and error alone, a decision future work on caused must respect.
@row_add n=3 row=decision/root-boundary-err-crosses-as-screen area=surface-desktop-native fact=F3 detector=none
name: Root boundary error crosses as screen content
rule: A handler Err the app's own root default boundary takes replaces the whole app on screen with the boundary's rendering of the error text, which crosses as screen content and never through an acting answer's caused.
why: Ratified: the root boundary replacing the app is an existing screen-content crossing, and caused holds no byte of the error text.
guard: stand_act_errors
basis: fct-nnu5onnh
because: The root boundary's replacement of the app was measured and the Err text's crossing narrowed to the screen by a ruling, which no row of the surface states.
@row_add n=4 row=decision/acting-step-panic-caught area=surface-cli fact=F4 detector=none
name: Acting step panic caught
rule: A handler panic during an acting step is caught: the session answers that call with a panic item in caused and keeps answering later calls.
guard: stand_act_errors
basis: fct-itglb53b
because: An acting step now runs under a panic catch so a panicking handler no longer ends the session, a behaviour callers of the CLI rely on.
