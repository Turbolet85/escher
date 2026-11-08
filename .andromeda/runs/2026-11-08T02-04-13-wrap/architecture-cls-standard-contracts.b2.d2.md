@answer n=1 entity=standard-contracts#6.6 v=no
why: the session changes no blitz-dom file (packages/blitz-dom is untouched and its git diff gate is green), so the style-data and resolve-context claim holds
@answer n=2 entity=standard-contracts#7.1 v=no
why: the session changes no blitz-dom layout file, so the LayoutPassState and Taffy trait claims still hold
@answer n=3 entity=standard-contracts#8.1 v=no
why: the session changes no blitz-dom node file, so the stacking-context mix-blend-mode reader stands
@answer n=4 entity=standard-contracts#9.1 v=no
why: the session changes no blitz-dom node or widget file, so the NodeData, SpecialElementData and Widget inventory stands
@answer n=5 entity=standard-contracts#10.1 v=no
why: the session only reads ElementData::is_disabled for the snapshot's clickable and changes it nowhere, and its blitz-dom file is unreached, so the shared-reading claim holds
@answer n=6 entity=standard-contracts#10.2 v=no
why: the session changes no blitz-dom select_list file, so row_disabled stands
@answer n=7 entity=standard-contracts#10.3 v=no
why: the session changes no blitz-dom select file, so the crate-private select module's readers stand
@answer n=8 entity=standard-contracts#10.4 v=no
why: the session changes no blitz-dom select file, so the select cursor and write-path claims stand
@answer n=9 entity=standard-contracts#10.5 v=no
why: the session changes no blitz-dom select_list file or crate root, so the public list read types and colour constants stand
@answer n=10 entity=standard-contracts#10.6 v=no
why: the session changes no blitz-dom select_list file, so the fresh-list claim and its named test stand
@answer n=11 entity=standard-contracts#10.7 v=no
why: the session changes no blitz-dom select_list file, so the MAX_ROWS and placement claims stand
@answer n=12 entity=standard-contracts#10.8 v=no
why: the session changes no blitz-dom node file, so the out_of_flow flag and its readers stand
@answer n=13 entity=standard-contracts#10.9 v=no
why: the session changes no blitz-dom node or construct file, so the inline-with-block-children flag stands
@answer n=14 entity=standard-contracts#10.10 v=no
why: the session changes no blitz-dom element or mutator file, so the classes field and its sync stand
@answer n=15 entity=standard-contracts#10.11 v=no
why: the session changes no blitz-dom element or mutator file, so the open-state flush and its callers stand
@answer n=16 entity=standard-contracts#11.1 v=no
why: the session changes no blitz-traits provider file, so the NetProvider, ShellProvider and Viewport contracts stand
@answer n=17 entity=standard-contracts#12.1 v=no
why: the session changes no blitz-html or blitz-net file, so the HTML and networking crate contracts stand
@answer n=18 entity=standard-contracts#13.1 v=no
why: the session changes no blitz-paint or blitz-shell file, so the paint and shell contracts stand
@answer n=19 entity=standard-contracts#13.2 v=no
why: the session changes no blitz-paint select_list render file, so paint_select_list stands
@answer n=20 entity=standard-contracts#13.3 v=no
why: the session changes no blitz-paint text file, so stroke_text and RunPaint stand
@answer n=21 entity=standard-contracts#13.4 v=no
why: the session changes no blitz-paint blend or render file, so the mix-blend-mode and isolation paint claims stand
@answer n=22 entity=standard-contracts#13.5 v=no
why: the session changes no blitz-paint layers or blend file, so the blend's lack of a frame witness stands
@answer n=23 entity=standard-contracts#14.1 v=no
why: the session changes no blitz-shell file, so BlitzShellProvider's redraw path stands
@answer n=24 entity=standard-contracts#15.1 v=no
why: the session changes no blitz-test-harness crate file (the reached tests/blitz-tests common module is test code, not the harness), so the harness exports and helpers stand
@answer n=25 entity=standard-contracts#16.1 v=no
why: the session moves no harness clock or time method, so the VirtualClock and pump claims stand
