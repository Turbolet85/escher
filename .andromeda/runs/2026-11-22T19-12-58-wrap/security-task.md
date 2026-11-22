@task_prose part=session
A behaviour-preserving structural corrective: the driver's command validation module had its inline unit tests moved whole into a child test module file, its production items unchanged.
Three upstream engine functions (the accessibility node builder, the keyboard event handler and the presentational hints computation) had escher's own additions moved out into private functions of escher's own, one call each in upstream's body; every read, arm, gate and order is kept, each ARIA attribute read once.
The lockfile moved url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3, closing advisory RUSTSEC-2026-0592 with no audit ignore added; the audit leg reads advisories ok, licenses ok and CI went green.
The bindgen and cssparser-macros syn edges were held at 3.0.6 against cargo's own re-resolution on the operator's ruling, because the lock cargo update -p url writes does not build (E0277 in bindgen); the workspace builds locked on the lock carried.
Measured: escher-driver line coverage 97.97 to 97.80 percent against its floor 97; no public symbol, verb, contract, schema or config changed.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: the driver's single command validation gate, the closed driver verb schema checked call by call with refusal by the first broken rule; edited, its inline unit tests replaced by a path to a child test module
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: the unit tests of the driver command validation gate (admitted calls, unknown verbs, malformed calls, refusal order, verb levels); new, moved whole from the gate's module
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder feeding the accessibility adapter, reading ARIA control states (expanded, selected, toggled, invalid) and the accessible name, aria-label trimmed once; edited, escher's reads moved into its own private functions
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine keyboard event handling, with Tab closing an open select list, range input key actions, single list box and drop-down select keys, multiple select keys and Enter activation of the focused control; edited, escher's arms moved into its own private functions
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hint attributes read by a typed parse, including a select's size hint rows and a single list box's overflow; edited, the select size arm moved into a private function
@task_file n=6 path=Cargo.lock
what: the lockfile read by locked builds and by the cargo-deny audit leg on every push; edited, url and form_urlencoded bumped, syn edges of bindgen and cssparser-macros held
@task_item n=1 part=touches
The dependency audit CI gate and cargo-deny as the audit tool: advisory RUSTSEC-2026-0592 closed by the url 2.5.9 bump, the audit leg green with advisories ok, licenses ok.
@task_item n=2 part=touches
Advisory ignores by ID and audit ignores with a written reason: no ignore added; the advisory was closed by a version move instead.
@task_item n=3 part=touches
Locked builds: the lock carried is one cargo's own re-resolution does not produce (syn edges of bindgen and cssparser-macros held at 3.0.6), yet the workspace builds and reads metadata under --locked with the lock unchanged.
@task_item n=4 part=touches
Engine dependency pins, upstream sync taking the dependency set whole and workspace-inherited dependency versions: a lock-only bump, no manifest touched, and two transitive syn edges held against resolution.
@task_item n=5 part=touches
The absence of automated dependency update tooling: the residual hazard that a later url update re-breaks the build until bindgen and prettyplease agree on syn stands beside it.
@task_item n=6 part=touches
The dependency-security overview's lockfile advisory state read on every push: the advisory state of the lock moved (one advisory closed).
@task_item n=7 part=touches
Per-crate line coverage floors: escher-driver's reading moved 97.97 to 97.80 percent; its floor 97 holds and is unchanged.
@task_item n=8 part=touches
Single command validation gate, closed driver verb schema, fixed order of target refusals and the driver refusal causes: the gate's module was restructured (tests moved out), its validation and refusal behaviour unchanged.
@task_item n=9 part=touches
One trimmed aria-label read, aria-labelledby outranking label and aria-label, toggle state read per role, closed parse of expanded, disabled and level, and control states crossing as closed values: the reads moved into escher's own builder functions, each attribute read once, values unchanged.
@task_item n=10 part=touches
Select expansion from engine state only and where a select's choice leaves: the expanded and selected state reads moved into escher's own functions, unchanged.
@task_item n=11 part=touches
Content classes on the accessibility adapter crossing and ratified exits of accessible names: the name reader moved, the crossing unchanged.
@task_item n=12 part=touches
Range input key actions, select keys passing a closed match, single select list keys, multiple select keys and clicks, and Enter activating the focused control: the five-arm key chain moved into one function of escher's own in its former order and gates.
@task_item n=13 part=touches
Tab chooses and closes an open select list and what closes an open select list: the Tab close moved into its own function, called before upstream's Tab handling as before.
@task_item n=14 part=touches
Select size hint capped, multiple select size sets its rows and presentational hints read by a typed parse: the select size arm moved into its own function; it now iterates every size attribute of the select, as the loop's arm did.
@task_fact n=1 slug=dependency-security
fact: The lockfile holds bindgen's and cssparser-macros' syn edges at 3.0.6 against cargo's own re-resolution (operator ruling), since bindgen 0.73.2 fails to build against syn 2.0.119; a later url update re-breaks the build until bindgen and prettyplease agree on syn.
@task_fact n=2 slug=dependency-security
fact: A lock move is proven by the locked workspace build as well as the audit leg; the audit leg alone proves no build.
@task_item n=1 part=untouched
Authentication and authorization: no CI token, publish guard, secret or application auth changed.
@task_item n=2 part=untouched
Data protection: kept check files, recordings, history, HTTP cache, cookies and TLS are untouched.
@task_item n=3 part=untouched
API security: the session socket, MCP server over stdio, outbound fetch limits, load waits and host idle expiry are unchanged.
@task_item n=4 part=untouched
Secret management and environment variables: no variable read or secret handling changed.
@task_item n=5 part=untouched
Logging and monitoring: span lines, the telemetry sink, target allowlists and log redaction are unchanged.
@task_item n=6 part=untouched
Stable element id, snapshot text, MCP protocol error, session socket reply and session error contracts: no carrier or answer changed shape.
@task_item n=7 part=untouched
CI workflows, actions pinning, the WPT runner and post-results workflow, and the licence gate's policy: none edited.
@task_item n=8 part=untouched
Script and resource fetching, URL scheme limits, iframe and import nesting limits and HTML parsing: not reached.
