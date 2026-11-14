@task_prose part=chunk
Bundles a run's fonts: every run path (the stand's boot, the driver session's served tasks, the shared fixture pages, a harness-built document naming no font context) shapes and measures text from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off.
The generic families a run's styles name (sans-serif, serif, monospace, an unknown family) reach the bundled set, not the host.
The fonts-unavailable refusal is kept: an app whose font context holds no family it can paint text with is still refused before anything is painted or read, its text unchanged.
Proves that every CI OS (Linux, macOS, Windows test runners) reads one state of a run the same way, compared by its readings (snapshot text, bounds, text extents, lint findings), never by a shot's bytes.
Holds the bundled set where a later canvas painter can reach it (the document's font context) and names canvas text as a future reader of it; builds no canvas painting.
@task_file n=1 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis stand's boot and its shared fixture pages, which build the stand's single-font context from DejaVu Sans — edits: the stand's font context build
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document's construction, which turns an absent font context into the host's system font collection — edits: BaseDocument::new's font context fallback
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the test harness's document options, whose font context defaults to none — edits: HarnessOptions font_ctx default
@task_file n=4 path=examples/seven_guis/assets/DejaVuSans.woff2
what: the bundled font asset, the stand's DejaVu Sans, a file already in the tree
@task_file n=5 path=packages/escher-driver/src/refusal.rs
what: the driver's refusal causes, home of the fonts-unavailable refusal kept unchanged
@task_file n=6 path=packages/escher-driver/src/screenshot.rs
what: the driver's screenshot, which refuses with fonts-unavailable before anything is painted
@task_item n=1 part=needs
The snapshot's text bounds and text extents are now measured from the bundled font set on every run path; the snapshot text form's parts and node bounds must read the same on every CI OS.
@task_item n=2 part=needs
The accessibility tree's node bounds, which the snapshot reads, follow text shaped from the bundled set rather than the host's fonts.
@task_item n=3 part=needs
The lint's contrast finding (its bounds, its ratio, its required value) is one of the readings compared across OSes; it must read the same from the bundled set.
@task_item n=4 part=needs
Open select list rows (one row height, an open list's row bounds, a closed select's option bounds, a multiple select's row bounds) are laid out from text the bundled set shapes, and must read the same across OSes.
@task_item n=5 part=needs
Hidpi or zoom changes reshape text: the reshaped text must still come from the bundled set, never fall back to host fonts.
@task_item n=6 part=needs
The stand's checks that read text from the shaped page (the stand reads no lint finding at boot, SC 1.4.3 contrast asserted and measured from computed styles) must keep holding with the bundled set on every run path.
@task_item n=7 part=needs
The driver's screenshot: no shot is compared by bytes across OSes; a screenshot still changes nothing and an element or row shot is unchanged in what it answers.
@task_item n=8 part=needs
The driver's refusal causes gain nothing: fonts-unavailable stays the refusal for an app with no paintable font family, its text unchanged, and no new cause, field, verb, argument or carrier is added.
@task_item n=9 part=needs
The snapshot model stays the single source the readers share; the cross-OS comparison reads the snapshot and the lint's findings through that model, adding no new reading.
@task_item n=10 part=needs
The obs log format gains no field and no span: no new log field or span is added by the bundling.
@task_item n=11 part=needs
Text runs label their parent element and the open list's option labels leave only as pixels: the text those readings take is shaped from the bundled set, their meaning unchanged.
@task_item n=12 part=needs
No change to which font a remote page or the browser app reads, unless it is brought to the founder as an engine-change fork.
@task_standing n=1
convention: each new check of the accessibility or snapshot readings is a Rust test over the AccessKit tree the document builds
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand check runs within the stand's repeated rounds, in the accessibility test leg of the slow tier
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: expected stand readings are stated once in the shared expectation tables, never restated per test
home: 7-screen-reader-support · screen-reader-support
@task_item n=1 part=not_needed
Keyboard navigation: Tab order, focus moves, keyboard activation, range input keys and select list keys; the chunk changes no key or focus behaviour.
@task_item n=2 part=not_needed
ARIA role mapping and ARIA state and property reads (checked, pressed, selected, expanded, invalid, level, described-by); fonts change no role or state.
@task_item n=3 part=not_needed
Accessible name precedence and label binding (aria-label, aria-labelledby, the label element); fonts change no name.
@task_item n=4 part=not_needed
Hidden and inert content exclusion from the tree and from hit-testing; fonts change what is hidden or inert in no way.
@task_item n=5 part=not_needed
The platform accessibility adapter, the accessibility tier and the accessibility feature's defaults; no adapter or feature changes.
@task_item n=6 part=not_needed
Cognitive accessibility: invalid state beyond colour or class; the chunk touches no form state.
@task_item n=7 part=not_needed
UI graph state registration and the Dioxus reached-handler record; the chunk registers no state and records no handler.
@task_item n=8 part=not_needed
Password masking and the password caret by bullets; no value or caret reading changes.
