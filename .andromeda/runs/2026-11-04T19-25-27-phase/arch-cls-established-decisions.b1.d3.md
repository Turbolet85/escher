@answer n=1 entity=established-decisions#1.1 v=no
why: the pins name non-Dioxus crates the probe does not touch, and Dioxus stays pinned under task item 4
@answer n=2 entity=established-decisions#2.1 v=no
why: dev and test profile settings shape build time, and the probe changes no profile
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning and publish bumps do not touch a probe that adds no crate or release
@answer n=4 entity=established-decisions#4.1 v=no
why: renderer backend and screenshot choices are excluded by task not_needed item 5, and the stand manifest already enables a renderer
@answer n=5 entity=established-decisions#5.1 v=yes
why: the stand boots dioxus-native and dioxus-native-dom under their default features, where accessibility gates the read-only snapshot and system-fonts meets the one bundled font
@answer n=6 entity=established-decisions#6.1 v=yes
why: incremental layout defaults to true, the first of the two layout modes the verdicts run in, and headless documents use ColorScheme Light for the pinned 800x600 Light viewport
@answer n=7 entity=established-decisions#7.1 v=no
why: render-blocking and resource loading act on linked stylesheets, images and iframes, and the offline stand loads none of them with its CSS inline
@answer n=8 entity=established-decisions#8.1 v=no
why: CSS and layout approximations are excluded by task not_needed item 9, and no link of the UI graph reads geometry
@answer n=9 entity=established-decisions#9.1 v=no
why: unsupported CSS features are excluded by task not_needed item 9, and the probe reads no style
@answer n=10 entity=established-decisions#10.1 v=yes
why: keyboard activation dispatches a Click that runs app handlers before the default action and the click focus rule covers a button, which decide what links (2) and (3) show
@answer n=11 entity=established-decisions#11.1 v=no
why: the inert and disabled click gates, password caret, toggle writes, client rect and hit centre meet no Counter or CRUD click or snapshot read the scope names
@answer n=12 entity=established-decisions#11.2 v=yes
why: the change event on checkbox and radio and the focus fallback are DOM events the bridge listener map and event names read, and the served task may carry them
@answer n=13 entity=established-decisions#11.3 v=no
why: the dropped anonymous block key-press fix is a merge surface the probe does not take, and the lean stands hide no control
@answer n=14 entity=established-decisions#11.4 v=yes
why: every engine time read goes through DocumentConfig.clock, which the harness shared VirtualClock drives under tick and advance_time only
@answer n=15 entity=established-decisions#11.5 v=no
why: the select model is excluded by task not_needed item 1, and the probe reads no select
@answer n=16 entity=established-decisions#11.6 v=no
why: the open select list is excluded by task not_needed item 1, and the probe reads no select
@answer n=17 entity=established-decisions#11.7 v=no
why: closing an open select list is excluded by task not_needed item 1, and the probe reads no select
@answer n=18 entity=established-decisions#11.8 v=no
why: Tab and Enter on an open select list are excluded by task not_needed item 1, and the probe reads no select
@answer n=19 entity=established-decisions#11.9 v=no
why: the rejected select list designs are excluded by task not_needed item 1, and the probe reads no select
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list standing limits are excluded by task not_needed item 1, and the probe reads no select
@answer n=21 entity=established-decisions#11.11 v=no
why: the select list merge surface is excluded by task not_needed item 1, and the probe merges nothing
@answer n=22 entity=established-decisions#11.12 v=no
why: the multiple select keys are excluded by task not_needed item 1, and the probe reads no select
@answer n=23 entity=established-decisions#11.13 v=no
why: the multiple select pointer is excluded by task not_needed item 1, and the probe reads no select
@answer n=24 entity=established-decisions#11.14 v=yes
why: the windowed shell redraw route is the shell loop the windowed hazard reading covers under task item 10, and the other changes it lists are not needed
@answer n=25 entity=established-decisions#11.15 v=yes
why: the twelve taken upstream commits, e42ee1f25 among them, are the HEAD state the verdicts re-read after the 2026-11-01 sync, and no boundary is widened in them
