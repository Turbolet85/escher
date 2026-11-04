@answer n=1 entity=established-decisions#1.1 v=no
why: pins of markup5ever, stylo, taffy, parley, winit and idna_adapter are not moved; the probe adds no dependency and the Dioxus pin is need 4 alone
@answer n=2 entity=established-decisions#2.1 v=no
why: build profiles are not_needed item 8; the probe changes no profile
@answer n=3 entity=established-decisions#3.1 v=no
why: release versioning is not_needed item 8; the probe adds no workspace crate or release
@answer n=4 entity=established-decisions#4.1 v=no
why: renderer backends and screenshot choices are not_needed item 5; the probe picks no backend or paint path
@answer n=5 entity=established-decisions#5.1 v=yes
why: the accessibility-gated snapshot read needs dioxus-native-dom's default accessibility, and the offline stand's one bundled font meets its system-fonts default
@answer n=6 entity=established-decisions#6.1 v=yes
why: the stand's pinned Light colour scheme and the incremental-layout default bound the two layout modes the probe runs
@answer n=7 entity=established-decisions#7.1 v=no
why: the stand boots offline with inline CSS and no head stylesheet, link, iframe or image load for resolve to wait on
@answer n=8 entity=established-decisions#8.1 v=no
why: CSS and layout approximations are not_needed item 9; the probe reads no approximated geometry
@answer n=9 entity=established-decisions#9.1 v=no
why: unsupported CSS features are not_needed item 9; no link of the UI graph reads a feature listed here
@answer n=10 entity=established-decisions#10.1 v=yes
why: keyboard activation queues a Click on a focused button so app handlers run before the default action, which link 2 and the listener map read
@answer n=11 entity=established-decisions#11.1 v=yes
why: inert and disabled gate whether a click or key reaches a control's handler, the verb the reacts-to-a-click reading measures
@answer n=12 entity=established-decisions#11.2 v=yes
why: the focus fixup and the window focus fallback decide which focused node a probe key press reaches; the visible_region, multiple select and optgroup parts do not bind
@answer n=13 entity=established-decisions#11.3 v=yes
why: a key press with focus on a control must not panic after its anonymous block is dropped, the key verb the probe drives
@answer n=14 entity=established-decisions#11.4 v=yes
why: engine time reads go through the document clock that the Harness VirtualClock moves, which the probe's poll(None) measurement relies on
@answer n=15 entity=established-decisions#11.5 v=no
why: the select model is not_needed item 1; the probe reads no select
@answer n=16 entity=established-decisions#11.6 v=no
why: the select list is not_needed item 1; the probe reads no select
@answer n=17 entity=established-decisions#11.7 v=no
why: select list closing is not_needed item 1; the probe reads no select
@answer n=18 entity=established-decisions#11.8 v=no
why: Tab and the move keys on an open select list are not_needed item 1
@answer n=19 entity=established-decisions#11.9 v=no
why: rejected select-list designs are not_needed item 1
@answer n=20 entity=established-decisions#11.10 v=no
why: the select list's standing limits are not_needed item 1
@answer n=21 entity=established-decisions#11.11 v=no
why: the select list's merge surface is upstream-sync work, not_needed items 1 and 10
@answer n=22 entity=established-decisions#11.12 v=no
why: multiple-select keys are not_needed item 1; the probe reads no select
@answer n=23 entity=established-decisions#11.13 v=no
why: multiple-select pointer handling is not_needed item 1; the probe reads no select
@answer n=24 entity=established-decisions#11.14 v=yes
why: the windowed shell's redraw route is the shell loop that need 10 has the probe read; the JavaScript, accessor and :open items do not bind
@answer n=25 entity=established-decisions#11.15 v=no
why: the twelve upstream commits are recorded by hash only, and HEAD is measured directly by needs 1 and 5
