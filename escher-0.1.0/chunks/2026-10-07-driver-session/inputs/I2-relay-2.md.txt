# Relay 2 — the P5 review of the plan for 2026-10-07-driver-session

Received at /andromeda-phase P5, 2026-10-07, in reply to the prompt `Apply? (yes / review / cancel)`. It arrived as
pasted text and was the whole of the message; the paste does not state its author.

## The review (verbatim)
review: two changes. (1) The library must not depend on the example. A crate under packages/ that depends on examples/seven_guis inverts the workspace layering and welds the driver to one app. Make Session and serve take the app boot from the caller (a closure or a small trait that returns the harness), with no seven_guis dependency in the library; the stand binding lives only with the escher-session binary. Put that binary where depending on seven_guis does not invert the layering, tell me where and what it costs, and keep "one headless stand instance" true for what the checks boot. (2) Lean 4 stands for this chunk, but record a CARRY for the Driver CLI entry: a host whose client died without stop keeps running; the CLI needs an answer (an idle expiry, or a list-and-stop command). The other leans stand.
