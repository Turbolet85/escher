@carried n=1 label=security-plan/input-validation fact=fct-d2yejoup
why: the buffered render must leave every element's stable id as it reads today across each re-render, with no new panic, log site or NodeId on the path, the ids in the buffer kept in process only.
quote: a non-element, stale or detached node reads `None`, and no path panics; the id is computed on demand and escher's own code writes it to no log, DOM or vdom
@carried n=1 run=2 label=security-plan fact=fct-4jl4t3ye
quote: Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
@carried n=1 run=3 label=security-plan fact=fct-qk7naa6l
quote: Dioxus mutations — `data-dioxus-id` parsed as `usize`, unsupported attribute value types ignored, falsy boolean attributes removed, and `dangerous_inner_html` applied with `set_inner_html`.
@carried n=1 run=4 label=security-plan fact=fct-767qdy6s
quote: Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path
@carried n=2 label=security-plan/input-validation fact=fct-r5yt6pat
why: the snapshot's names must read the same after the port, and the buffer that holds the render's text must reach no log, so no name reaches stderr at any level.
quote: escher's own code logs no name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level
@carried n=3 label=security-plan/input-validation fact=fct-agnqdsgq
why: the snapshot's state readings, taken from the tree the replayed document builds, must stay the same closed words and booleans, the mask untouched, since the chunk re-asserts the snapshot and its diff unchanged.
quote: and the snapshot, which takes the three from the tree node and never parses the element a second time
@carried n=4 label=security-plan/input-validation fact=fct-7jixhrbw
why: the chunk adds no carrier, field or exit, so it must stay outside a widening under this rule, the ratified carriers' content unchanged.
quote: a new field of a closed value set inside an already ratified carrier is not a widening and does not come to him, while a field holding authored text, an open value set, a new carrier or a new exit still does
@carried n=4 run=2 label=security-plan fact=fct-n4q5kc73
quote: Driver session socket (escher-driver) — the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@carried n=4 run=3 label=security-plan fact=fct-6hwcr6nz
quote: bounded replies carry ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings, the state readings as closed words and booleans, never an attribute's text, and no `NodeId`.
@carried n=5 label=security-plan/input-validation fact=fct-v4gvn7w2
why: stand_replay, re-run among the stand_* witnesses, must still find every kept answer of the committed checks equal after the buffered render.
quote: and stops at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only; the reader also reads the two committed checks under `tests/blitz-tests/checks/` (`stand_replay`)
@carried n=5 run=2 label=security-plan fact=fct-khxrpkj3
quote: every replayed step still passes `validate` inside `Session::run`.
@binding n=1
**input-validation ↔ logging-monitoring**: the buffer holds ids, tag and attribute names, attribute values and text, so no log site, field or span added on the render path may carry them, the ids and names logged nowhere (security-plan/input-validation fct-d2yejoup) (security-plan/input-validation fct-r5yt6pat), any log line going through escher's sink (security-plan fct-ggtnnptr).
@binding n=2
**input-validation ↔ error-handling**: replaying the buffer in the original call order must keep the Dioxus mutation rules (security-plan fct-qk7naa6l) and reach `element_to_node_id`'s unwrap with no id that is not yet mapped or stale (security-plan fct-f4v2lnec), while a MountedData call during render keeps degrading through `try_doc` and `OperationFailed` with no new panic path (security-plan fct-c4rr3jfv) (security-plan fct-p5sm3bek).
@binding n=3
**input-validation ↔ data-protection**: the masked password and file-input values must read the same in the snapshot, its text, diffs and every carrier after the port (security-plan fct-767qdy6s) (security-plan/input-validation fct-agnqdsgq), and the buffer, which holds those values unmasked, stays in process and is never printed (security-plan fct-mkaeodr2).
