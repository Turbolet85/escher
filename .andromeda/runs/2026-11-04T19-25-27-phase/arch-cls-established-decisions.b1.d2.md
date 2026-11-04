@answer n=1 entity=established-decisions#1.1 v=no
why: the chunk adds no dependency and moves no pin (needs 4); the non-Dioxus pins, the idna_adapter pin and the commented patch blocks are not units the probe touches
@answer n=2 entity=established-decisions#2.1 v=no
why: build profiles are excluded by the task (not_needed 8); the probe adds no profile
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning is excluded by the task (not_needed 8); the probe ships nothing
@answer n=4 entity=established-decisions#4.1 v=no
why: renderer backends and paint_bench, screenshot and wpt rendering are excluded (not_needed 5); the probe paints nothing
@answer n=5 entity=established-decisions#5.1 v=yes
why: dioxus-native-dom's default `accessibility` feature gates the snapshot modules the probe's check reads read-only (needs 17)
@answer n=6 entity=established-decisions#6.1 v=yes
why: incremental layout defaults to true, one of the two layout modes both verdicts run in (needs 8); the headless Light colour scheme is the stand's pinned Light viewport (needs 5)
@answer n=7 entity=established-decisions#7.1 v=no
why: render-blocking `<head>` stylesheets and `<link>` loads are not named by the scope; the stand boots offline with inline const CSS, so no link or resolve check is reached
@answer n=8 entity=established-decisions#8.1 v=no
why: CSS layout approximations are excluded (not_needed 9) and no link of the UI graph reads geometry (not_needed 14)
@answer n=9 entity=established-decisions#9.1 v=no
why: the unsupported CSS features (Shadow DOM, container queries, 3D transforms, :focus-visible and the rest) touch none of the four links or the Counter and CRUD path
@answer n=10 entity=established-decisions#10.1 v=yes
why: keyboard activation (a released Space or pressed Enter dispatches a queued Click, so app handlers run before the default action) and click focus through handle_click bear on link (2) and link (3) and on the deferred click-listener reading; the scroll, range and bounds parts are not reached
@answer n=11 entity=established-decisions#11.1 v=yes
why: `inert` and `disabled` on a form control decide whether a click or key reaches its listener, which the link (2) runs measure; the password-click, hit-stop and absolute_position parts are not reached
@answer n=12 entity=established-decisions#11.2 v=yes
why: the focus fixup and the tree's focus falling back to the window decide which node a key press reaches, the path the MountedData hazard runs on (needs 9)
@answer n=13 entity=established-decisions#11.3 v=yes
why: a key press with focus on a control whose anonymous block was dropped must not panic, and the probe pumps key input through poll(None) on the hazard path (needs 7, 9)
@answer n=14 entity=established-decisions#11.4 v=yes
why: the document's clock goes through DocumentConfig.clock, which is the harness VirtualClock headlessly, the one clock only tick and advance_time move (needs 7)
@answer n=15 entity=established-decisions#11.5 v=no
why: the single-select keys, typeahead and select model are excluded (not_needed 1); the probe reads no select and the stand has none
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list popup is excluded (not_needed 1); the probe reads no select
@answer n=17 entity=established-decisions#11.7 v=no
why: closing an open select list is excluded (not_needed 1); the probe reads no select
@answer n=18 entity=established-decisions#11.8 v=no
why: Tab and the rows of an open select list are excluded (not_needed 1); the probe reads no select
@answer n=19 entity=established-decisions#11.9 v=no
why: the rejected select-list designs concern the select only (not_needed 1); the probe reads no select
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list's standing limits concern the select only (not_needed 1); the probe reads no select
@answer n=21 entity=established-decisions#11.11 v=no
why: the select list's merge surface is not reached, the probe merges nothing (not_needed 10) and no reached file is a merge or conflict surface
@answer n=22 entity=established-decisions#11.12 v=no
why: the multiple-select keys are excluded (not_needed 1); the probe reads no select
@answer n=23 entity=established-decisions#11.13 v=no
why: the multiple-select pointer is excluded (not_needed 1); the probe reads no select
@answer n=24 entity=established-decisions#11.14 v=yes
why: the windowed shell's redraw route is one of the upstream behaviour changes the probe reads from code on the windowed path (needs 9, 10)
@answer n=25 entity=established-decisions#11.15 v=no
why: the twelve upstream commits name no file or path the probe reads and state no behaviour the chunk must act on; no boundary is widened in them
