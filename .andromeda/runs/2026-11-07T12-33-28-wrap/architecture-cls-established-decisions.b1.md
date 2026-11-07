@answer n=1 entity=established-decisions#1.1 v=no
why: the session moves no manifest, lock or dependency, so the pinned versions it cites stand
@answer n=2 entity=established-decisions#2.1 v=no
why: the build profiles in Cargo.toml are untouched by a render-path change in dioxus-native-dom
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning and the workspace version are unaffected by the bridge's buffered render
@answer n=4 entity=established-decisions#4.1 v=no
why: renderer backend selection in the examples, screenshot and wpt is untouched by the bridge change
@answer n=5 entity=established-decisions#5.1 v=no
why: dioxus-native-dom's default feature list lives in its manifest, which the session left alone, and the new module sits outside every cfg gate
@answer n=6 entity=established-decisions#6.1 v=no
why: the document defaults in blitz-dom are unchanged, the bridge's mutation buffer being no document default
@answer n=7 entity=established-decisions#7.1 v=no
why: resource loading in blitz-dom is untouched, the session changing only the Dioxus bridge's render path
@answer n=8 entity=established-decisions#8.1 v=no
why: the CSS approximations in blitz-dom and stylo_taffy are unchanged by the bridge's buffered render
@answer n=9 entity=established-decisions#9.1 v=no
why: the buffered render's limit is a bridge behaviour recorded as its own residual row, and no engine CSS or layout feature in this list changed
@answer n=10 entity=established-decisions#10.1 v=no
why: the scrolling and input rules sit in blitz-dom files the session did not reach, and the bridge change alters none of them
@answer n=11 entity=established-decisions#11.1 v=yes mode=stale
why: it names residual res-3v42legm as its owner, which the session re-said as absorbed into v030-03 and owned through CARRY cry-mjwr45xr
@answer n=12 entity=established-decisions#11.2 v=no
why: its merge-surface owner line is unchanged and its visible_region, focus and select rules sit in blitz-dom files the session did not reach
@answer n=13 entity=established-decisions#11.3 v=no
why: the layout_parent rule sits in blitz-dom's document.rs, which the session did not reach, and its owner line is unchanged
@answer n=14 entity=established-decisions#11.4 v=yes mode=stale
why: it names residual res-3v42legm as its owner, which the session re-said as absorbed into v030-03 and owned through CARRY cry-mjwr45xr
@answer n=15 entity=established-decisions#11.5 v=yes mode=stale
why: it names residual res-3v42legm as its owner, which the session re-said as absorbed into v030-03 and owned through CARRY cry-mjwr45xr
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list's painting and rules are blitz-dom behaviour the bridge change does not touch
@answer n=17 entity=established-decisions#11.7 v=no
why: the rules for closing an open select list are blitz-dom behaviour the bridge change does not touch
@answer n=18 entity=established-decisions#11.8 v=no
why: the Tab, Enter and move-key rules of an open select list are blitz-dom behaviour the bridge change does not touch
@answer n=19 entity=established-decisions#11.9 v=no
why: it records rejected and unbuilt select list approaches, which the session does not build
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list's standing limits are documented in docs/capabilities.md and blitz-dom, neither reached by the session
@answer n=21 entity=established-decisions#11.11 v=no
why: its merge-surface owner line is unchanged by the session and its sixteen files are blitz-dom files the session did not reach
@answer n=22 entity=established-decisions#11.12 v=no
why: the multiple select's keys are blitz-dom behaviour the bridge change does not touch
@answer n=23 entity=established-decisions#11.13 v=no
why: the multiple select's pointer rules are blitz-dom behaviour the bridge change does not touch
@answer n=24 entity=established-decisions#11.14 v=no
why: the six upstream behaviour changes it lists are blitz-dom and engine behaviour the session did not reach
@answer n=25 entity=established-decisions#11.15 v=no
why: the twelve upstream behaviour changes it lists are blitz-dom and engine behaviour the session did not reach
