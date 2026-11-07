@answer n=1 entity=established-decisions#1.1 v=no
why: no manifest, lock or dependency moved, and its Cargo.toml citations are outside the reached files
@answer n=2 entity=established-decisions#2.1 v=no
why: the session changes no build profile and reaches no file that holds one
@answer n=3 entity=established-decisions#3.1 v=no
why: the workspace version and the release tooling are untouched by the session
@answer n=4 entity=established-decisions#4.1 v=no
why: the renderer backends and their feature wiring are unchanged, and the dioxus-native file it cites is not reached
@answer n=5 entity=established-decisions#5.1 v=no
why: no default feature list moves, and the new private module adds no feature or manifest line
@answer n=6 entity=established-decisions#6.1 v=no
why: blitz-dom's document defaults and Stylo prefs are unchanged, and the session reaches no blitz-dom file
@answer n=7 entity=established-decisions#7.1 v=no
why: resource loading in blitz-dom and its cited tests is unchanged, and none of those files is reached
@answer n=8 entity=established-decisions#8.1 v=no
why: the CSS approximations in blitz-dom and stylo_taffy are unchanged, and the session reaches no file that holds them
@answer n=9 entity=established-decisions#9.1 v=no
why: the unsupported-feature list is unchanged, the session adds no CSS or DOM capability to blitz-dom
@answer n=10 entity=established-decisions#10.1 v=no
why: blitz-dom's scroll and input rules are unchanged, and the bridge's MountedData scroll calls are not what this entity states
@answer n=11 entity=established-decisions#11.1 v=yes mode=stale
why: the session absorbs residual res-3v42legm into v030-03, so the open-owner reading this entity gives it moves
@answer n=12 entity=established-decisions#11.2 v=no
why: its merge-surface owner is the Upstream sync line and names no residual, and the session reaches none of its files
@answer n=13 entity=established-decisions#11.3 v=no
why: the layout_parent rule in blitz-dom's document is unchanged, and the session reaches only dioxus-native-dom files and the probe test
@answer n=14 entity=established-decisions#11.4 v=yes mode=stale
why: the session absorbs residual res-3v42legm into v030-03, moving the open-owner reading this clock entity names for its merge surface
@answer n=15 entity=established-decisions#11.5 v=yes mode=stale
why: the session absorbs residual res-3v42legm into v030-03, moving the open-owner reading this select-model entity names for its merge surface
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list's painting and its named files are unchanged and not reached by the session
@answer n=17 entity=established-decisions#11.7 v=no
why: the select list's closing rules are unchanged by the session
@answer n=18 entity=established-decisions#11.8 v=no
why: the select list's Tab and row rules are unchanged by the session
@answer n=19 entity=established-decisions#11.9 v=no
why: the rejected select-list alternatives are unaffected by a Dioxus bridge change
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list's standing limits are unchanged, and none of them reads a reached file
@answer n=21 entity=established-decisions#11.11 v=no
why: its merge-surface line is owned by the Upstream sync line and names no residual, and its sixteen files are not reached
@answer n=22 entity=established-decisions#11.12 v=no
why: the multiple-select key rules are unchanged by the session
@answer n=23 entity=established-decisions#11.13 v=no
why: the multiple-select pointer rules are unchanged by the session
@answer n=24 entity=established-decisions#11.14 v=no
why: the list of upstream behaviour changes is unchanged, since the 286874cc1 port is a dioxus-native-dom bridge change, not a blitz-dom take
@answer n=25 entity=established-decisions#11.15 v=no
why: the twelve upstream commits listed are unchanged, since the 286874cc1 port is a dioxus-native-dom bridge change outside this list
