# Classified — arch · standard-contracts.b4

## Answers
standard-contracts#19.1 · yes · a lint step's kept answer and its FNV-1a 64 digest, taken over `Outcome::Linted`'s `to_json()`, must stay byte-identical as the Needs name, and `Session::run`'s `command` span stays unchanged
standard-contracts#19.2 · yes · replay compares a kept lint answer's line with the live one, so the byte-identical `{"findings":[…]}` must keep matching on replay, the match the Needs name; replay's own code is not reached
standard-contracts#19.3 · yes · the visual lint itself: `Session::lint` and `findings` return the six defects in document then `Defect::ALL` order, refuse `fonts-unavailable` before any read, read the snapshot (recording its ids as `snapshot` does) and the text extent and contrast read-only, and run no step, settle, pump or time move
standard-contracts#19.4 · no · the load wait (`settle_waiting`, `LOAD_WAIT`, the acting verbs' settle in session.rs and execute.rs) is not reached, and lint's no-wait clause is the no-step property 19.3 already carries
standard-contracts#20.1 · yes · the flight booker's one `overlap` (`flight-type-one-way` × `flight-type-return`) and the zero readings of the other lean tasks are pinned by `stand_lint` in both layout modes and must hold through the refactored walk, the defect itself staying open
standard-contracts#20.2 · no · the load page's `load-fetch` click waiting out `LOAD_WAIT` is the load wait's witness in stand_load_page.rs, a click path the chunk does not touch; the load page's 0-findings boot reading is a stand_lint pin, not a statement of this entity
standard-contracts#21.1 · no · blitz-vibey-script's JavaScript runtime and `ScriptDocument` are not reached, the lint walk, the listing test and the sweep touching no script
standard-contracts#22.1 · no · the high-level launch entry points (blitz `launch_url` and `launch_static_html`, dioxus-native `launch`) are not reached, the stand booting headless through the harness and the chunk changing no launch path
