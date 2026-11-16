@row_retire n=1 row=decision/caused-answer-crossing seen=1 heir=decision/caused-text-crossing detector=none
why: Caused no longer carries closed words and a count only: each item now carries its message, location, cause chain and fields, so the rule and its why that left the text to a follow-up are both false.
basis: fct-wed52rrj
because: The closed-words-only caused shape is retired and the text crosses, so the row is replaced by one stating the widened crossing.
@row_retire n=2 row=decision/root-boundary-err-as-screen-content seen=1 heir=decision/root-boundary-err-crossing detector=none
why: A root-boundary Err no longer crosses as screen content only: its text and chain now cross in caused, so the row's name, rule and narrowing why are false.
basis: fct-smxs2rkb
because: The root Err's text now crosses in caused, so the row is replaced by one stating both crossings.
@row_add n=1 row=decision/caused-text-crossing area=threat-model-summary detector=none
name: Caused text crossing
rule: Each acting verb's answer carries caused on every ratified carrier only when the step caused something: one item per panic, boundary error and kept record, holding its message, location, boundary, cause chain and fields, a content-named field read as redacted.
why: A boundary widening to the error text, ratified.
basis: fct-z2b5kqi7
because: Caused now carries each record's text from every source, scrubbed by field name, replacing the retired closed-words crossing.
@row_add n=2 row=decision/caused-answer-bounds area=threat-model-summary fact=F1 detector=none
name: Caused answer bounds
rule: The caused answer keeps at most 16 items, 8 chain links and 8 fields per item, each text cut at 1024 characters with its cut count, and counts the rest in caused_past by kind and source.
basis: fct-z2b5kqi7
because: The crossing of error text is bounded by fixed limits, a contract every carrier of caused relies on.
@row_add n=3 row=decision/root-boundary-err-crossing area=threat-model-summary detector=none
name: Root-boundary Err crossing
rule: A handler Err the app's root error boundary takes crosses as screen content, through Dioxus's default boundary rendering of its Display, and in caused as one error item holding its message and cause chain under the root boundary.
basis: fct-gvjd2ehp
because: The root Err's text and three-link chain now cross in caused, so its crossing is restated beside the screen-content one.
@row_add n=4 row=decision/capture-scrub-by-field-name area=logging-monitoring fact=F2 detector=none
name: Scoped capture scrubs by field name
rule: The scoped capture scrubs every source by field name alone: a content-named field reads [redacted] while messages and other fields are kept, an engine record's message included, which escher's sink still redacts on its own lines.
basis: fct-xz256kkb
because: The capture now keeps text from app, framework and engine sources, and its one scrub rule is the control on what of it crosses.
@row_add n=5 row=decision/capture-panic-hook-chaining area=logging-monitoring fact=F3 detector=none
name: Capture's chaining panic hook
rule: The scoped capture installs once per process a chaining panic hook that keeps a panic's payload and location on the capture open on the panicking thread, prints, logs and fields nothing, and always runs the hook it replaced.
basis: fct-z2b5kqi7
because: A process-wide hook now holds panic payloads in process for caused, and what it does not print or log is a logging control.
@row_add n=6 row=decision/engine-log-values-in-fields area=logging-monitoring fact=F4 detector=none
name: Engine log values in named fields
rule: Engine warn and error sites carry their values in named fields under fixed messages; no site interpolates a value into its message.
basis: fct-g5qt4zja
because: The capture keeps an engine record's message whole and scrubs by field name only, so the engine's values must stand in fields to be redacted.
@row_set n=1 row=decision/escher-target-content-scrub cell=rule seen=1 fact=F5 detector=none
value: An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
basis: fct-rzllzxpg
because: password joined the content-named field set, so the listed set the row holds must name it.
@row_set n=2 row=decision/escher-target-content-fields-withheld cell=rule seen=1 detector=none
value: In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
basis: fct-wbcflx2v
because: The content-named field set moved from 11 to 12 names, a count the row's rule holds.
@row_set n=3 row=decision/escher-telemetry-allowlist-sink cell=except seen=1 detector=none
value: The scrubbing reaches that sink and escher's scoped capture only: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
basis: fct-xz256kkb
because: The scoped capture now scrubs the text it keeps by the same content-named set, so the scrubbing no longer reaches the sink alone.
@row_set n=4 row=decision/mask-holds-on-every-carrier cell=except seen=1 detector=none
value: A caused item's message holds what its author wrote; only a content-named field in it reads as redacted.
basis: fct-s5bltahf
because: Caused now carries authored message text, where the mask holds only as far as its author and the field-name scrub keep it.
@row_set n=5 row=decision/socket-carries-screen-reading cell=rule seen=1 detector=none
value: An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, and an acting step's caused text scrubbed by field name, never a NodeId or an attribute value as typed.
basis: fct-z2b5kqi7
because: An acting step's answer now carries the caused error and log text, a class the row's list of what an answer carries lacks.
@row_none n=1 fact=F6 type=9
what: The boundary registry's unbounded growth is a deviation the wrap directive dispositions to an owner, a CARRY or a residual row; the route holds it.
