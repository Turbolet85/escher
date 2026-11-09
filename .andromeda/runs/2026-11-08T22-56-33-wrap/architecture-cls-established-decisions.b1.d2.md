@answer n=1 entity=established-decisions#1.1 v=no
why: its rule lives in Cargo.toml and the lockfile, which the session does not reach (manifests and lockfile stay byte-identical)
@answer n=2 entity=established-decisions#2.1 v=no
why: its rule lives in the Cargo.toml profile tables, which the session does not reach
@answer n=3 entity=established-decisions#3.1 v=no
why: its rule lives in the workspace manifest and publish workflow, which the session does not reach, and no crate is added
@answer n=4 entity=established-decisions#4.1 v=no
why: its rule lives in the example, harness and renderer files, none of which the session reaches
@answer n=5 entity=established-decisions#5.1 v=no
why: its rule lives in the per-crate Cargo.toml feature lists, which the session does not reach, and no feature is added
@answer n=6 entity=established-decisions#6.1 v=no
why: its rule lives in blitz-dom's document, config and stylo files, which the session does not reach
@answer n=7 entity=established-decisions#7.1 v=no
why: its rule lives in blitz-dom's resource-loading files and their tests, which the session does not reach
@answer n=8 entity=established-decisions#8.1 v=no
why: its rule lives in blitz-dom and stylo_taffy, which the session does not reach
@answer n=9 entity=established-decisions#9.1 v=no
why: its rule lives in blitz-dom, stylo_taffy and their tests, which the session does not reach
@answer n=10 entity=established-decisions#10.1 v=no
why: its scrolling, pointer and keyboard rules live in blitz-dom, which the session does not reach, and the new reached record leaves engine behaviour unchanged
@answer n=11 entity=established-decisions#11.1 v=no
why: its seven engine changes live in blitz-dom's pointer, accessibility, document and clock files, which the session does not reach
@answer n=12 entity=established-decisions#11.2 v=no
why: its visible_region and focus-fixup rules live in blitz-dom, which the session does not reach
@answer n=13 entity=established-decisions#11.3 v=no
why: its deallocate_anonymous_block rule lives in blitz-dom's document.rs, which the session does not reach
@answer n=14 entity=established-decisions#11.4 v=no
why: its document-clock rule lives in blitz-dom's config and clock files, which the session does not reach
@answer n=15 entity=established-decisions#11.5 v=no
why: its select-model rules live in blitz-dom's select, keyboard and pointer files, which the session does not reach
@answer n=16 entity=established-decisions#11.6 v=no
why: its select-list rules live in blitz-dom's document.rs and blitz-paint, which the session does not reach
@answer n=17 entity=established-decisions#11.7 v=no
why: its select-list closing rules live in blitz-dom, which the session does not reach
@answer n=18 entity=established-decisions#11.8 v=no
why: its open-list key and pointer rules live in blitz-dom's keyboard and pointer files, which the session does not reach
@answer n=19 entity=established-decisions#11.9 v=no
why: it records rejected and unbuilt select-list alternatives, which the session does not touch
@answer n=20 entity=established-decisions#11.10 v=yes mode=stale
why: the session reaches docs/capabilities.md whole and adds three stated limits to it, so the cited lines 259-261 and 262-264 may have moved
@answer n=21 entity=established-decisions#11.11 v=no
why: its merge surface is a set of blitz-dom files, none of which the session reaches, so the sixteen-file count stands
@answer n=22 entity=established-decisions#11.12 v=no
why: its multiple-select key rules live in blitz-dom's keyboard.rs, which the session does not reach
@answer n=23 entity=established-decisions#11.13 v=no
why: its click_select_row rule lives in blitz-dom's pointer.rs, which the session does not reach
@answer n=24 entity=established-decisions#11.14 v=no
why: its upstream-taken behaviour changes live in blitz-dom, the shell and the script crossing, none of which the session reaches
@answer n=25 entity=established-decisions#11.15 v=no
why: its twelve upstream-taken commits live in blitz-dom and the script crossing, which the session does not reach
