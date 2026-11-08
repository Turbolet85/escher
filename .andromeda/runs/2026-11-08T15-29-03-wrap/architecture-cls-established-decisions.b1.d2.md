@answer n=1 entity=established-decisions#1.1 v=no
why: the session moves no dependency, manifest or lockfile, so the pinned versions hold.
@answer n=2 entity=established-decisions#2.1 v=no
why: no manifest or build profile moved, and the session adds no crate or setting.
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning and the workspace version are untouched by the session.
@answer n=4 entity=established-decisions#4.1 v=no
why: the renderer backends of paint_bench, screenshot and blitz-test-harness are untouched; the driver's screenshot arm only gains target checks.
@answer n=5 entity=established-decisions#5.1 v=no
why: no feature list moved, and the new dioxus-native-dom method adds no feature.
@answer n=6 entity=established-decisions#6.1 v=no
why: document defaults and Stylo prefs are engine code the session does not change.
@answer n=7 entity=established-decisions#7.1 v=no
why: resource loading and stylesheet blocking are engine behaviour, and no engine file changed.
@answer n=8 entity=established-decisions#8.1 v=no
why: CSS approximations live in blitz-dom and stylo_taffy, which the session does not touch.
@answer n=9 entity=established-decisions#9.1 v=no
why: unsupported features are engine behaviour the session leaves alone.
@answer n=10 entity=established-decisions#10.1 v=no
why: scrolling and input rules are engine behaviour, and the actions read scrolls nothing and changes no engine file.
@answer n=11 entity=established-decisions#11.1 v=no
why: the session changes no engine file, so the direct engine changes and the covered meaning stand as recorded.
@answer n=12 entity=established-decisions#11.2 v=no
why: visible_region, focus and the select changes are engine code the session does not reach.
@answer n=13 entity=established-decisions#11.3 v=no
why: deallocate_anonymous_block is engine code the session leaves alone.
@answer n=14 entity=established-decisions#11.4 v=no
why: the document clock is engine code, and no time read moved.
@answer n=15 entity=established-decisions#11.5 v=no
why: the select model is engine behaviour, and the actions read changes no select state.
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list's opening and painting are engine behaviour, and the actions read leaves an open list open as this decision holds.
@answer n=17 entity=established-decisions#11.7 v=no
why: the list's close rules are engine behaviour, and the session changes no close path.
@answer n=18 entity=established-decisions#11.8 v=no
why: Tab, Enter and row choice are engine behaviour the session does not change.
@answer n=19 entity=established-decisions#11.9 v=no
why: the rejected and unbuilt alternatives for the select list are unaffected by the session.
@answer n=20 entity=established-decisions#11.10 v=yes mode=stale
why: docs/capabilities.md is reached whole and gains two stated limits, so its cited lines 245-247 and 248-250 may have moved while the select limits hold.
@answer n=21 entity=established-decisions#11.11 v=no
why: the select list's merge surface is upstream-sync bookkeeping, and the session reaches none of its sixteen files.
@answer n=22 entity=established-decisions#11.12 v=no
why: the multiple-select keys are engine behaviour in keyboard.rs, which the session does not touch.
@answer n=23 entity=established-decisions#11.13 v=no
why: the multiple-select pointer is engine behaviour in pointer.rs, which the session does not touch.
@answer n=24 entity=established-decisions#11.14 v=no
why: the upstream-taken behaviour changes are engine code the session's driver and doc changes do not reach.
@answer n=25 entity=established-decisions#11.15 v=no
why: the listed upstream commits are engine behaviour the session does not reach.
