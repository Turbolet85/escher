@answer n=1 entity=visual-design-verification#8.6 v=no
why: the session reaches only the Dioxus bridge mutation buffer, document and probe files, and the scrollbar hit-test in blitz-dom pointer.rs and stand_cells_served.rs stay unchanged
@answer n=2 entity=visual-design-verification#8.7 v=no
why: the open select list's paint and covered-click rules are in select_popup_painted.rs, select_popup.rs and stand_act_popup.rs, none of which the session reaches
@answer n=3 entity=visual-design-verification#8.8 v=no
why: the blend-layer paint order from render_element and the select_popup_painted pass are outside the MountedData buffered render, which touches no paint code
@answer n=4 entity=visual-design-verification#8.9 v=no
why: the devtools layout outline and highlight settings in blitz-traits devtools.rs are not in any file the session reaches
