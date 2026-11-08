@answer n=1 entity=visual-design-verification#8.6 v=yes mode=stale
why: the session reaches tests/blitz-tests/tests/stand_cells_served.rs, which this entity cites at lines 216-232, and a reached file is reached whole, so the citation may have moved
@answer n=2 entity=visual-design-verification#8.7 v=no
why: the session reaches no select popup test file and no paint or hit-test path, changing only the Dioxus event handler dispatch
@answer n=3 entity=visual-design-verification#8.8 v=no
why: the session reaches no paint or blend code, and render_element is not in any reached file
@answer n=4 entity=visual-design-verification#8.9 v=no
why: the session does not reach packages/blitz-traits/src/devtools.rs, which holds the devtools settings
