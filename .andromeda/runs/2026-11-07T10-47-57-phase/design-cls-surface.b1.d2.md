@answer n=1 entity=surface#1.1 v=no
why: the UA sheet is added in DocumentConfig::default(), a unit the chunk does not edit, and no re-asserted check names it
@answer n=2 entity=surface#1.2 v=no
why: select default CSS rules; no select is touched and no named check reads them
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row layout in the UA sheet; the chunk touches no select and names no check over it
@answer n=4 entity=surface#1.4 v=no
why: multiple-select chosen-row colour; a colour the chunk does not change
@answer n=5 entity=surface#1.5 v=no
why: open select list colour constants on SelectListView; no select list is touched
@answer n=6 entity=surface#1.6 v=no
why: single-select group layout in the UA sheet; the chunk touches no select
@answer n=7 entity=surface#1.7 v=no
why: select list opening and painting; the chunk touches no select list and names no check over it
@answer n=8 entity=surface#1.8 v=no
why: select list overlay painting through paint_scene; the chunk does not touch the overlay
@answer n=9 entity=surface#1.9 v=no
why: select list highlight and key behaviour; no select list is touched and the cited select_popup tests are not re-run by the chunk
@answer n=10 entity=surface#1.10 v=no
why: select choice events; the chunk changes no select and its cited tests are not the chunk's checks
@answer n=11 entity=surface#1.11 v=no
why: select list closing on Escape, click or page scroll; the chunk leaves select event handling as it is and touches no select list
@answer n=12 entity=surface#1.12 v=no
why: disabled and hidden select opening rules; no select is touched
@answer n=13 entity=surface#1.13 v=no
why: focused select surviving a dropped container; a key-on-select rule the chunk does not touch and whose cited test is not a chunk check
@answer n=14 entity=surface#1.14 v=no
why: select list group header reading; no select list is touched
@answer n=15 entity=surface#1.15 v=no
why: highlighted row absence from snapshot and tree for an open select; the cited checks are select_popup tests, not checks the chunk re-runs
@answer n=16 entity=surface#1.16 v=yes
why: names stand_act_popup.rs, a stand_* check the chunk re-runs, and states the open-list row bounds in the snapshot that check reads
@answer n=17 entity=surface#1.17 v=no
why: the initial window theme; the chunk passes no theme and does not change the windowed theme default
@answer n=18 entity=surface#1.18 v=yes
why: the ported test re-measures get_client_rect, and the default viewport size, scale and zoom are the input that measured geometry reads
@answer n=19 entity=surface#1.19 v=no
why: JS innerWidth and devicePixelRatio in the vibey-script runtime; the chunk does not touch that runtime
@answer n=20 entity=surface#2.1 v=no
why: macOS browser tabstrip padding; the chunk changes no browser UI component
@answer n=21 entity=surface#2.2 v=no
why: shell provider window controls; the chunk neither calls nor changes them
@answer n=22 entity=surface#3.1 v=no
why: custom GPU widget opacity in an example; the chunk does not touch the example
@answer n=23 entity=surface#4.1 v=no
why: devtools overlay drawing; the chunk does not touch devtools
@answer n=24 entity=surface#5.1 v=yes
why: the windowed launch renders through the buffer, and the shell's title rule needs the document's title node set before the window exists, which the buffered replay must keep
@answer n=25 entity=surface#5.2 v=no
why: rdme window title text and Ctrl/Cmd+T theme key are app-level rules the chunk does not change, and no named check reads them
