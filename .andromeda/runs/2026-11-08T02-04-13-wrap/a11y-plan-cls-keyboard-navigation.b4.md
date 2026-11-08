@answer n=1 entity=keyboard-navigation#5.8 v=yes mode=stale
why: the session reached cells.rs, and the arrow clamp and synchronous scroll_to are not in the report's unchanged keyboard statement, which covers only Cells' Tab order
@answer n=2 entity=keyboard-navigation#5.9 v=yes mode=stale
why: the session reached cells.rs, and the Cells editor's Enter commit, focus return and click early-return are not covered by the report's Enter and Space activation statement, which concerns buttons
@answer n=3 entity=keyboard-navigation#5.10 v=no
why: the report keeps the CRUD list keys, its Tab stop, the unread aria-activedescendant and the People name unchanged (no key path changed; untouched items 4 and 7; names as before)
@answer n=4 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC Preact page at examples/preact/index.html is not among the session's reached files
@answer n=5 entity=keyboard-navigation#5.12 v=no
why: the Android Activity files are not among the session's reached files
@answer n=6 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture under examples/assets is not among the session's reached files
@answer n=7 entity=keyboard-navigation#6.2 v=no
why: the google fixture under examples/assets is not among the session's reached files
@answer n=8 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction bindings sit in examples/assets, which the session did not reach
@answer n=9 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible styling sits in examples/assets, which the session did not reach
@answer n=10 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture under examples/assets is not among the session's reached files
@answer n=11 entity=keyboard-navigation#6.6 v=no
why: servo.css under examples/assets is not among the session's reached files
@answer n=12 entity=keyboard-navigation#6.7 v=no
why: servo.html under examples/assets is not among the session's reached files
@answer n=13 entity=keyboard-navigation#6.8 v=no
why: examples/form.rs is not among the session's reached files
@answer n=14 entity=keyboard-navigation#7.1 v=no
why: the report keeps focusability and the accessibility tree lifecycle unchanged, and the stand focus check's file stand_snapshot_state.rs is not reached
@answer n=15 entity=keyboard-navigation#7.2 v=yes mode=stale
why: the session reached cells.rs, and the Cells arrow, clamp, scroll-into-view and editor clauses of this keyboard walk are not in the report's unchanged statements, though its other clauses hold
@answer n=16 entity=keyboard-navigation#7.3 v=no
why: the report keeps the load page and Cells Tab sequences unchanged (Tab orders re-read and unmoved) in the reached common/mod.rs
@answer n=17 entity=keyboard-navigation#7.4 v=no
why: the report keeps range-input key paths unchanged and the Timer slider's role, name and focus sequence as before, and the range test files are not reached
@answer n=18 entity=keyboard-navigation#7.5 v=no
why: the text and range no-change event test file is not reached and the report keeps key paths unchanged
@answer n=19 entity=keyboard-navigation#7.6 v=no
why: the report keeps single and multiple select key paths unchanged, and the a11y and stand-repeat leg membership of these files is unaffected by the new stand_graph file
@answer n=20 entity=keyboard-navigation#7.7 v=no
why: the select popup, select list and log test files are not among the session's reached files
@answer n=21 entity=keyboard-navigation#7.8 v=no
why: the report keeps the flight booker's select Tab stop, its name and its arrow keys unchanged, and stand_keyboard is not reached
@answer n=22 entity=keyboard-navigation#7.9 v=no
why: the key-press container test file is not reached and it runs in neither leg the session changed
@answer n=23 entity=keyboard-navigation#8.1 v=no
why: the report keeps focusability and Tab orders unchanged, so the session added no tabindex to the searched files
@answer n=24 entity=keyboard-navigation#8.2 v=no
why: the report keeps focusability and Tab orders unchanged, so the session added no tabindex to the searched files
@answer n=25 entity=keyboard-navigation#8.3 v=no
why: the session adds no focus or keyboard handling, and its event-path change records a dispatch while keeping key behaviour as before
