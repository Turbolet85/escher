@answer n=1 entity=visual-design-verification#3.8 v=no
why: focus-ring paint (2px #4D90FE outline on inputs) is not-needed item 12; the probe reads no outline or colour pair and reads focus off the tree.

@answer n=2 entity=visual-design-verification#3.9 v=no
why: forced-colors focus outline in the google fixture is a user-preference token (not-needed item 12) on a page the probe does not load.

@answer n=3 entity=visual-design-verification#3.10 v=no
why: caret and password-bullet paint is a paint fact; the seven_guis stand has no password input and the probe paints nothing.

@answer n=4 entity=visual-design-verification#4.1 v=no
why: prefers-reduced-motion queries in the BBC fixture are motion tokens (not-needed item 12) in a fixture the probe does not run.

@answer n=5 entity=visual-design-verification#5.1 v=no
why: absence of reduced-motion and colour-scheme rules across the s02 files is a motion and colour-scheme token fact (not-needed item 12).

@answer n=6 entity=visual-design-verification#5.2 v=no
why: no-motion and no-advance read is pinned for an open select's list, which the probe does not read (not-needed item 2); the probe's post-step reads are of Counter and CRUD only.

@answer n=7 entity=visual-design-verification#6.1 v=no
why: 0.35 opacity on disabled browser-chrome icon buttons is paint of a state the probe never reaches (not-needed items 1 and 6).

@answer n=8 entity=visual-design-verification#6.2 v=no
why: author styles keyed on :open and the open attribute's reads are not the probe's; it reads no :open and adds no attribute path (not-needed item 8).

@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is not an :open state; that is the select list (not-needed item 2), which the probe does not read.

@answer n=10 entity=visual-design-verification#6.4 v=no
why: status word beside colour on WPT runner result lines; the probe touches no WPT runner output.

@answer n=11 entity=visual-design-verification#6.5 v=no
why: synthesized bold on a group header in an open select list is the select list (not-needed item 2).

@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in examples/form.rs about an input's accent colour; nothing the probe reads, writes or prints.

@answer n=13 entity=visual-design-verification#7.1 v=no
why: viewport colour scheme and document zoom are user-preference tokens (not-needed item 12); the probe reads no viewport setting.

@answer n=14 entity=visual-design-verification#7.2 v=no
why: colour-scheme device rebuild on the stylist is a user-preference token (not-needed item 12).

@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale invalidating text shaping is paint behind a scale the headless stand probe never changes (not-needed item 12).

@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom is a windowed-shell shortcut; the probe is headless and dispatches no zoom key (not-needed item 10).

@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme tokens and backgrounds (not-needed item 12); rdme is not the stand.

@answer n=18 entity=visual-design-verification#7.6 v=no
why: colour-scheme declarations on the TodoMVC and reference pages (not-needed item 12); the probe loads neither page.

@answer n=19 entity=visual-design-verification#7.7 v=no
why: un-inverting images under inverted-colors is a user-preference token (not-needed item 12).

@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action per-axis panning is a pointer path the probe does not drive (not-needed item 10).

@answer n=21 entity=visual-design-verification#8.1 v=yes
why: accessibility_tree is the probe's sole read surface and it excludes display none, visibility hidden and inert nodes with their descendants, so a link read through the tree must count such a node absent (needs items 1 and 3).

@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility-hidden hit-test exclusion is a hit rule; the probe's CRUD row clicks target visible rows and it adds no hit path (not-needed item 7).

@answer n=23 entity=visual-design-verification#8.3 v=no
why: a non-invertible transform takes no hit; the stand has no transform and the probe adds no hit path (not-needed item 7).

@answer n=24 entity=visual-design-verification#8.4 v=no
why: pointer-events none makes an element transparent to hits; the probe hit-tests no pointer-events element (not-needed item 7).

@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none suppresses overlay scrollbar paint; the probe reads no scrollbar.
