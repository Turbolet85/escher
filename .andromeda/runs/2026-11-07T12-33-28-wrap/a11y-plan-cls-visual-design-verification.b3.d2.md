@answer n=1 entity=visual-design-verification#8.6 v=no
why: The session changes only the dioxus-native-dom bridge (mutation_buffer, dioxus_document, lib) and the probe, and the scrollbar hit rule in blitz-dom events/pointer.rs and stand_cells_served.rs are not among them.
@answer n=2 entity=visual-design-verification#8.7 v=no
why: The session moves no paint, select-popup or covered-click code, so the open-list rules in select_popup_painted.rs and select_popup.rs stand on unchanged files.
@answer n=3 entity=visual-design-verification#8.8 v=no
why: The session moves no blend or render_element paint code, and the layered order it describes lies outside the four reached files.
@answer n=4 entity=visual-design-verification#8.9 v=no
why: The devtools layout outline and highlight settings in blitz-traits/src/devtools.rs are outside the four reached files, and the session changes no devtools code.
