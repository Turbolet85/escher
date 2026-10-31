# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · standing S5 · Formatting and lints: fmt, clippy and rustdoc gates through ci-leg.sh, which every change owes
conventions#2.1 · no · the chunk edits no manifest and adds no dependency or workspace crate
conventions#3.1 · yes · the focus and key code implements SC 2.1.1, 2.4.3 and 1.4.3, so spec URLs sit beside those implementations; any new public reader carries `///`; the layer cap constant states its rationale
conventions#4.1 · yes · the scope names the unbuilt pseudo-element `content: attr()` and non-string content gap, which bears on how the group label gets painted, so any path left unbuilt is marked `TODO` or `FIXME`
conventions#5.1 · yes · the scope names the choice's writers as crate-private, `node::select` holding the rules with no public type, and any new reader staying read-only
conventions#6.1 · yes · execute's hand-built `command` span has no cfg gate and the snapshot's accessibility gate sits in units the chunk edits; any engine tracing call it adds is gated per call site
conventions#7.1 · no · the chunk writes no unsafe block and the scope names no borrow conflict that the take-out-and-restore pattern must solve
conventions#8.1 · yes · the four stand tables each take a new row, a new scenario runs in both layout modes after its fixture assertion (S2), and a table two checks share lives once in tests/common/mod.rs
conventions#9.1 · no · standing S1 · unit tests in a `#[path]` child file; the chunk adds no unit-test module in its own file
conventions#9.2 · no · standing S3 · fixture pages in fixtures/mod.rs; the chunk adds no fixture page
conventions#9.3 · no · the chunk edits neither mcp_tests.rs nor stand_requirement_sweep.rs and adds no MCP verb or sweep row
conventions#10.1 · no · the chunk adds no binary, app or example, and flight_booker.rs is reached only as the stand's popup with no edit named
conventions#11.1 · no · the chunk adds no HTML fixture, and Not needed names the google and graphite fixture forms
