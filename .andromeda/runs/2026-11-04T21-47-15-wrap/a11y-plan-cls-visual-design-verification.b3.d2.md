@answer n=1 entity=visual-design-verification#8.6 v=no
why: the session reaches only the probe test, Cargo.lock (the pulldown-cmark bump) and deny.toml, and the scrollbar pointerdown rule lives in packages/blitz-dom/src/events/pointer.rs and tests/blitz-tests/tests/stand_cells_served.rs, neither reached.
@answer n=2 entity=visual-design-verification#8.7 v=no
why: the open-list paint and covered-click rule lives in select_popup_painted.rs, select_popup.rs and stand_act_popup.rs, none of which the session reached, and the Cargo.lock change is only the pulldown-cmark bump.
@answer n=3 entity=visual-design-verification#8.8 v=no
why: the paint order past blend layers rests on render_element and select_popup_painted.rs, neither reached by the session, and the Cargo.lock change is only the pulldown-cmark bump.
@answer n=4 entity=visual-design-verification#8.9 v=no
why: the devtools overlay settings at packages/blitz-traits/src/devtools.rs:5-34 are outside every file the session reached, and the Cargo.lock change is only the pulldown-cmark bump.
