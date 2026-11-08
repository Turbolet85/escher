@answer n=1 entity=established-decisions#1.1 v=no
why: its pins live in Cargo.toml, Cargo.lock and deny.toml, which the session leaves unchanged with no dependency added or removed
@answer n=2 entity=established-decisions#2.1 v=no
why: its profile rule lives in Cargo.toml, which the session does not reach, and its cited baseline is a dated measurement
@answer n=3 entity=established-decisions#3.1 v=no
why: its rule lives in the publish workflow and the workspace Cargo.toml, neither reached by the session
@answer n=4 entity=established-decisions#4.1 v=no
why: its renderer backends live in examples, wpt, rdme, blitz-test-harness and dioxus-native, none reached and dioxus-native unedited
@answer n=5 entity=established-decisions#5.1 v=no
why: its default feature lists live in manifests the session leaves unchanged, and the new graph module sits under the accessibility default already listed
@answer n=6 entity=established-decisions#6.1 v=no
why: its document defaults live in blitz-dom, which the session does not reach and whose git diff gate is green
@answer n=7 entity=established-decisions#7.1 v=no
why: its resource-loading rule lives in blitz-dom and its cited tests, none of them reached by the session
@answer n=8 entity=established-decisions#8.1 v=no
why: its CSS approximations live in blitz-dom and stylo_taffy, neither reached by the session
@answer n=9 entity=established-decisions#9.1 v=no
why: its unsupported-feature rules live in blitz-dom and stylo_taffy, neither reached, and the session adds no CSS support
@answer n=10 entity=established-decisions#10.1 v=no
why: its scrolling and input rules live in blitz-dom, which the session does not reach, and the bridge dispatch change sits outside the engine event driver
@answer n=11 entity=established-decisions#11.1 v=no
why: its direct engine changes live in blitz-dom files the session does not reach
@answer n=12 entity=established-decisions#11.2 v=no
why: its visible_region, focus and select changes live in blitz-dom files the session does not reach
@answer n=13 entity=established-decisions#11.3 v=no
why: its deallocate_anonymous_block change lives in blitz-dom document.rs, which the session does not reach
@answer n=14 entity=established-decisions#11.4 v=no
why: its document clock lives in blitz-dom config, clock and scrolling files, none reached by the session
@answer n=15 entity=established-decisions#11.5 v=no
why: its select model lives in blitz-dom keyboard, pointer and select files, none reached by the session
@answer n=16 entity=established-decisions#11.6 v=no
why: its select list lives in blitz-dom and blitz-paint, and its cited select_popup test is not among the reached files
@answer n=17 entity=established-decisions#11.7 v=no
why: its close rules live in blitz-dom document and pointer files, none reached, and the cited select_popup test is not reached
@answer n=18 entity=established-decisions#11.8 v=no
why: its Tab and row rules live in blitz-dom keyboard and pointer files, none reached by the session
@answer n=19 entity=established-decisions#11.9 v=no
why: its rejected alternatives concern blitz-dom and blitz-paint design, which the session does not reach
@answer n=20 entity=established-decisions#11.10 v=yes mode=stale
why: its limits are cited from docs/capabilities.md:230-235, a reached file the session edits, so those citations can have moved
@answer n=21 entity=established-decisions#11.11 v=no
why: its merge surface is blitz-dom and blitz-traits files, none reached, and no upstream take occurs in this chunk
@answer n=22 entity=established-decisions#11.12 v=no
why: its multiple select keys live in blitz-dom keyboard.rs, which the session does not reach
@answer n=23 entity=established-decisions#11.13 v=no
why: its click_select_row lives in blitz-dom pointer.rs, which the session does not reach
@answer n=24 entity=established-decisions#11.14 v=no
why: its upstream behaviour changes live in blitz-dom and the windowed shell, neither reached by the session
@answer n=25 entity=established-decisions#11.15 v=no
why: its upstream commits are blitz-dom and blitz-traits behaviour changes, and the session brings no upstream take
