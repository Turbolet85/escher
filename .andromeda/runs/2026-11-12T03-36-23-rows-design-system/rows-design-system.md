@row_none n=1 rec=fct-rnr6ebdp@1 type=S
@row_none n=2 rec=fct-jbi2ij6e@1 type=S
@row_none n=3 rec=fct-xfodco37@1 type=S
@row_add n=1 row=decision/select-list-colours-engine-constants area=color-palette rec=fct-nqadbrsf@1
name: Select list colours as engine constants
rule: The open select list's colours are engine constants, not design tokens and not in the default stylesheet; its highlight is the same pair as a multiple select's chosen row.
why: The engine holds no design tokens.
@row_none n=4 rec=fct-dgxjrhfr@1 type=S
@row_none n=5 rec=fct-iawtqlzw@1 type=S
@row_none n=6 rec=fct-oolgokdo@1 type=S
@row_none n=7 rec=fct-r74vd227@1 type=S
@row_add n=2 row=decision/screenshot-never-skips-text area=typography rec=fct-kjaf2qti@1
name: Screenshot never paints with text skipped
rule: A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
@row_add n=3 row=contract/driver-screenshot-refusals/fonts-unavailable area=typography rec=fct-kjaf2qti@1
boundary: driver screenshot refusals
item: fonts-unavailable
promise: A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
@row_add n=4 row=decision/test-engine-with-system-fonts area=typography rec=fct-6tyrmhic@1
name: Test engine built with system fonts
rule: The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_add n=5 row=decision/wasm-bundled-font-context area=typography rec=fct-v7kb2nps@1
name: Bundled font context on WASM
rule: On WASM a font context with bundled fonts must be provided, a single-font context for one font.
@row_none n=8 rec=fct-a5ni2awv@1 type=S
@row_none n=9 rec=fct-tqd5dldd@1 type=S
@row_none n=10 rec=fct-3tbpfog4@1 type=S
@row_none n=11 rec=fct-uwbv2oqg@1 type=S
@row_none n=12 rec=fct-435ixehz@1 type=S
@row_none n=13 rec=fct-d35vvhfz@1 type=S
@row_none n=14 rec=fct-c6md4fhl@1 type=S
@row_none n=15 rec=fct-ii7p3j24@1 type=S
@row_none n=16 rec=fct-q73in4ta@1 type=9
@row_none n=17 rec=fct-atzpfc54@1 type=S
@row_none n=18 rec=fct-woyfbmnb@1 type=S
@row_add n=6 row=decision/hit-testing-stops-at-paint-clip area=depth-strategy rec=fct-zdoz4c52@1
name: Hit-testing stops at a paint clip
rule: Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box.
except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
@row_none n=19 rec=fct-tsm6w3f3@1 type=S
@row_none n=20 rec=fct-bn2nrqdn@1 type=S
@row_none n=21 rec=fct-iqz2qfcv@1 type=S
@row_none n=22 rec=fct-b37lobif@1 type=9
@row_none n=23 rec=fct-fdzevwkk@1 type=S
@row_none n=24 rec=fct-drur5box@1 type=S
@row_none n=25 rec=fct-pe4pylgg@1 type=S
@row_none n=26 rec=fct-4cmypdo2@1 type=S
@row_none n=27 rec=fct-vt4vwm5c@1 type=9
@row_none n=28 rec=fct-w6dpo7qk@1 type=S
@row_add n=7 row=decision/into-view-scroll-instant-in-nested-boxes area=motion-calibrated-to-expression-level-unrecorded rec=fct-tnwqvola@1
name: Into-view scroll instant in nested boxes
rule: A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone.
except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.
@row_none n=29 rec=fct-buuw5o72@1 type=S
@row_add n=8 row=decision/settle-does-not-wait-on-animation area=motion-calibrated-to-expression-level-unrecorded rec=fct-vjxch6lk@1
name: Settle does not wait on animation
rule: A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
@row_add n=9 row=decision/animation-driven-by-virtual-clock area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
name: Animation driven by the virtual clock
rule: The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
@row_add n=10 row=contract/driver-verbs-and-virtual-clock/advance area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: advance
promise: The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
@row_add n=11 row=contract/driver-verbs-and-virtual-clock/click area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: click
promise: The driver's click leaves the VirtualClock where it was.
@row_add n=12 row=contract/driver-verbs-and-virtual-clock/type area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: type
promise: The driver's type leaves the VirtualClock where it was.
@row_add n=13 row=contract/driver-verbs-and-virtual-clock/press area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: press
promise: The driver's press leaves the VirtualClock where it was.
@row_add n=14 row=decision/load-wait-moves-no-clock area=motion-calibrated-to-expression-level-unrecorded rec=fct-rxggiikl@1
name: Load wait moves no clock
rule: While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
@row_none n=30 rec=fct-ibq7wjp3@1 type=S
@row_none n=31 rec=fct-hm6l7tmv@1 type=S
@row_none n=32 rec=fct-heaaydxr@1 type=S
@row_none n=33 rec=fct-vjurj4wq@1 type=5
@row_none n=34 rec=fct-spadupwu@1 type=S
@row_none n=35 rec=fct-7ewroczb@1 type=S
@row_none n=36 rec=fct-xuneycqw@1 type=S
@row_none n=37 rec=fct-izgk733s@1 type=S
@row_none n=38 rec=fct-kdmcvdbg@1 type=5
@row_none n=39 rec=fct-p4bhbrip@1 type=S
@row_none n=40 rec=fct-5kn46jcr@1 type=5
@row_none n=41 rec=fct-nd7wvb2o@1 type=S
@row_none n=42 rec=fct-gnv6fqzu@1 type=5
@row_add n=15 row=decision/driver-output-for-agents area=surface-cli rec=fct-or2earye@1
name: Driver command line output read by agents
rule: The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not.
why: These surfaces are read by an agent, not a person.
@row_add n=16 row=contract/escher-session-stdout/command-line-line area=surface-cli rec=fct-or2earye@1
boundary: escher-session stdout
item: command line output
promise: One JSON line per command: the answer, the refusal or a session error's object.
@row_add n=17 row=contract/escher-session-stdout/mcp-line area=surface-cli rec=fct-or2earye@1
boundary: escher-session stdout
item: mcp role output
promise: Newline-delimited JSON-RPC 2.0 lines.
@row_add n=18 row=contract/escher-session-exit-codes/0 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
@row_add n=19 row=contract/escher-session-exit-codes/1 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 1
promise: The command was refused.
@row_add n=20 row=contract/escher-session-exit-codes/2 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 2
promise: A usage error.
@row_add n=21 row=contract/escher-session-exit-codes/3 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 3
promise: A session error; an mcp role ending in Err also exits 3.
@row_none n=43 rec=fct-riuyhmj7@1 type=S
@row_add n=22 row=overview/surface-web-spa area=surface-web-spa rec=fct-x3p4qjpb@1
text: The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.
@row_none n=44 rec=fct-j6ghrsj3@1 type=S
@row_none n=45 rec=fct-5nzdvlmo@1 type=S
@row_none n=46 rec=fct-5jmivcud@1 type=S
@row_none n=47 rec=fct-zjo5o5zs@1 type=5
@row_none n=48 rec=fct-kvxteum3@1 type=5
@row_none n=49 rec=fct-vjavizn2@1 type=S
@row_none n=50 rec=fct-4wznelig@1 type=5
@row_none n=51 rec=fct-3rkpcvos@1 type=5
@row_none n=52 rec=fct-ft5gyw4z@1 type=5
@row_none n=53 rec=fct-wfthjucl@1 type=5
@row_none n=54 rec=fct-ddqph55d@1 type=4
@row_none n=55 rec=fct-cyybiebp@1 type=9
@row_none n=56 rec=fct-ytbrguse@1 type=4
@row_none n=57 rec=fct-prgo5gtm@1 type=5
@row_none n=58 rec=fct-pez3luu5@1 type=5
@row_none n=59 rec=fct-yqrg4ucs@1 type=5
@row_none n=60 rec=fct-2nxzibsi@1 type=5
@row_none n=61 rec=fct-vjqihnw3@1 type=7
@row_none n=62 rec=fct-ciqmyfr7@1 type=7
@row_none n=63 rec=fct-cejbuz7z@1 type=7
@row_none n=64 rec=fct-ls6yfpy7@1 type=7
@row_none n=65 rec=fct-sf23gvry@1 type=7
@row_none n=66 rec=fct-y5s6tvsm@1 type=7
@row_none n=67 rec=fct-3ejwfsuo@1 type=4
@row_add n=23 row=contract/viewport-color-scheme/color-scheme area=color-palette rec=fct-bwfqqjae@1
boundary: The viewport's system color scheme
item: color scheme
promise: The system color scheme is Light by default or Dark, is set on the viewport, and feeds prefers-color-scheme.
@row_add n=24 row=decision/color-scheme-change-full-recascade area=color-palette rec=fct-tkymbjhz@1
name: Full recascade on color-scheme change
rule: A change of color scheme triggers a full recascade of the document.
why: light-dark() and system colors resolve at cascade time.
@row_none n=68 rec=fct-blf3mieo@1 type=4
relied: Iframe sub-documents take the parent viewport's color scheme.
@row_none n=69 rec=fct-or5fa5av@1 type=4
@row_add n=25 row=contract/dioxus-native-config/base-color area=color-palette rec=fct-kmvrmuee@1
boundary: dioxus-native configuration
item: base_color
promise: The frame clear color is an optional base_color, a peniko Color, unset by default.
@row_none n=70 rec=fct-hsrlbfqh@1 type=4
@row_none n=71 rec=fct-obnrxxei@1 type=4
@row_none n=72 rec=fct-ipmksf32@1 type=5
@row_none n=73 rec=fct-lgq6vpvz@1 type=5
@row_none n=74 rec=fct-hu6iqe45@1 type=S
@row_none n=75 rec=fct-ldhghls4@1 type=5
@row_add n=26 row=decision/select-chosen-row-chromium-pair area=color-palette rec=fct-vx2ivc7b@1
name: Select chosen row takes Chromium's inactive selected pair
rule: A multiple select's chosen row is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text.
why: Follows Chromium's inactive selected pair; hardcoded as the engine holds no tokens.
guard: select_interaction
@row_add n=27 row=decision/engine-colours-hardcoded area=color-palette rec=fct-vx2ivc7b@1,fct-4365wjwp@1,fct-4pd3zld4@1,fct-7xjrejiq@1
name: Engine colours hardcoded
rule: The engine's own select colours (chosen row, open list highlighted row, disabled row and border) are fixed hardcoded values; the open select list's are constants of the select list view, not in the default sheet.
why: The engine holds no tokens.
@row_none n=76 rec=fct-itdq3zc7@1 type=9
@row_none n=77 rec=fct-6tygk7ef@1 type=5
@row_none n=78 rec=fct-46r6qldw@1 type=5
@row_none n=79 rec=fct-jgwhhaoy@1 type=5
@row_none n=80 rec=fct-etnvkm2s@1 type=5
@row_none n=81 rec=fct-y4llv7yc@1 type=5
@row_none n=82 rec=fct-4zglaolr@1 type=5
@row_none n=83 rec=fct-sjgxjva6@1 type=5
@row_none n=84 rec=fct-tsfy6s4v@1 type=5
@row_none n=85 rec=fct-anwoxb7v@1 type=5
@row_none n=86 rec=fct-xp75h5al@1 type=5
@row_none n=87 rec=fct-nzvpimut@1 type=5
@row_none n=88 rec=fct-eh5atpfx@1 type=5
@row_none n=89 rec=fct-63vaqdm2@1 type=5
@row_none n=90 rec=fct-xumrd476@1 type=5
@row_none n=91 rec=fct-hr2o5w4y@1 type=5
@row_none n=92 rec=fct-icetjnaz@1 type=5
@row_none n=93 rec=fct-cftb623n@1 type=5
@row_none n=94 rec=fct-fb6jxps4@1 type=5
@row_none n=95 rec=fct-o4xmo4yy@1 type=5
@row_none n=96 rec=fct-u4hnumsv@1 type=5
@row_none n=97 rec=fct-iebm2tqh@1 type=5
@row_none n=98 rec=fct-jyxo2y24@1 type=5
@row_none n=99 rec=fct-pinupjh4@1 type=5
@row_none n=100 rec=fct-27f4t2jl@1 type=5
@row_none n=101 rec=fct-my2w5twf@1 type=4
@row_none n=102 rec=fct-p7icj7bo@1 type=4
@row_none n=103 rec=fct-rgwceom5@1 type=5
@row_none n=104 rec=fct-okx3zhns@1 type=5
@row_none n=105 rec=fct-y37cc5gk@1 type=5
@row_none n=106 rec=fct-uyr6iib2@1 type=4
@row_none n=107 rec=fct-f7eh6bug@1 type=5
@row_add n=28 row=absence/engine-color-tokens area=color-palette rec=fct-5lr3uypt@1
capability: Color tokens or CSS custom properties in the engine and integration crates
@row_none n=108 rec=fct-o4oj77dg@1 type=5
@row_none n=109 rec=fct-tzzpnj2i@1 type=5
@row_none n=110 rec=fct-egb2doim@1 type=5
@row_none n=111 rec=fct-avcjl5al@1 type=5
@row_none n=112 rec=fct-bas4rvdu@1 type=5
@row_none n=113 rec=fct-3chvitmv@1 type=5
@row_none n=114 rec=fct-zlqdkahl@1 type=5
@row_none n=115 rec=fct-mrf3xpbp@1 type=5
@row_none n=116 rec=fct-xbtaw3h6@1 type=5
@row_none n=117 rec=fct-drq5raj7@1 type=7
@row_none n=118 rec=fct-e4jigdc2@1 type=5
@row_none n=119 rec=fct-slgkqo7y@1 type=5
@row_none n=120 rec=fct-cma4e7o6@1 type=5
@row_none n=121 rec=fct-ujkkm46d@1 type=5
@row_none n=122 rec=fct-zetftvqr@1 type=5
@row_none n=123 rec=fct-toyw5jkw@1 type=5
@row_none n=124 rec=fct-zsfmji3l@1 type=5
@row_none n=125 rec=fct-rqdjf62x@1 type=S
@row_none n=126 rec=fct-7f5ztzr3@1 type=5
@row_none n=127 rec=fct-germ3vvh@1 type=5
@row_none n=128 rec=fct-dl5cu6xg@1 type=5
@row_none n=129 rec=fct-a76rgemn@1 type=4
@row_none n=130 rec=fct-g6x7bhkj@1 type=4
relied: Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root.
@row_add n=29 row=decision/hit-test-stops-at-paint-clip area=depth-strategy rec=fct-ynoy6voq@1
name: Hit-testing stops where paint clips
rule: Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box.
why: Ratified; a hit answers only what paint shows.
guard: hit_clipped_at_scrolling_box
except: The root element.
@row_none n=131 rec=fct-z3qfoykp@1 type=4
relied: hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered.
@row_none n=132 rec=fct-dqdo2wfr@1 type=4
@row_none n=133 rec=fct-kywr7h4v@1 type=4
@row_add n=30 row=decision/auto-z-index-shared-paint-level area=depth-strategy rec=fct-f2g7f7ak@1
name: Positioned auto z-index paint level
rule: Positioned descendants with z-index auto share one paint level and paint in tree order.
why: CSS 2.1 Appendix E
guard: paint_order
@row_none n=134 rec=fct-cbe7vaww@1 type=4
relied: The open select list paints after the root's render and the blend layers, so it stands above every box of the page.
@row_none n=135 rec=fct-vhukioye@1 type=4
@row_none n=136 rec=fct-2ssdivf2@1 type=4
@row_none n=137 rec=fct-4lb3fws4@1 type=4
@row_none n=138 rec=fct-eqb53qnu@1 type=4
@row_none n=139 rec=fct-q6ap2crc@1 type=4
@row_none n=140 rec=fct-3j5hjl7t@1 type=5
@row_none n=141 rec=fct-u225y7ui@1 type=4
@row_none n=142 rec=fct-wepbw2vn@1 type=5
@row_none n=143 rec=fct-y3cpbowe@1 type=5
@row_none n=144 rec=fct-mf3536c6@1 type=5
@row_none n=145 rec=fct-ki3c3guz@1 type=7
@row_none n=146 rec=fct-4aaray2i@1 type=7
@row_none n=147 rec=fct-x42hwfvk@1 type=7
@row_none n=148 rec=fct-4ih5wyju@1 type=7
@row_add n=31 row=decision/cursor-icon-crate area=iconography rec=fct-yw4swvug@1
name: cursor-icon crate for cursor icons
rule: Cursor icons are cursor-icon crate values, CSS cursor keywords map one-to-one to them, and cursor: none maps to no cursor.
@row_add n=32 row=contract/shell-provider/set-cursor area=iconography rec=fct-yw4swvug@1
boundary: ShellProvider
item: set_cursor
promise: Takes an optional CursorIcon; None hides the cursor and resets it to Default.
@row_none n=149 rec=fct-keknjqam@1 type=4
relied: The favicon read returns the href of the first link element whose rel contains icon.
@row_none n=150 rec=fct-bbr7kf2x@1 type=5
@row_none n=151 rec=fct-zl77wswc@1 type=5
@row_none n=152 rec=fct-vtmtilnu@1 type=5
@row_none n=153 rec=fct-hxngvsgt@1 type=5
@row_none n=154 rec=fct-5mc57xdd@1 type=5
@row_none n=155 rec=fct-o2mv7754@1 type=7
@row_none n=156 rec=fct-tzidfeny@1 type=7
@row_none n=157 rec=fct-xkcpmz2a@1 type=5
@row_none n=158 rec=fct-wr73fuj5@1 type=4
relied: scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll.
@row_none n=159 rec=fct-zbk2czd3@1 type=4
relied: A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once.
@row_none n=160 rec=fct-vb77s7g2@1 type=4
@row_none n=161 rec=fct-z2ugu37c@1 type=4
@row_add n=33 row=decision/into-view-scroll-nested-boxes-instant area=motion rec=fct-vt52hpbo@1
name: Into-view scroll is instant in nested boxes
rule: A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone.
why: The document holds one scroll animation at a time.
@row_none n=162 rec=fct-enbza3q2@1 type=4
@row_add n=34 row=decision/overlay-scrollbar-fade-timings area=motion rec=fct-57yciv35@1
name: Overlay scrollbar fade timings
rule: Overlay scrollbars show at full opacity on scroll, stay opaque for 500 ms after last activity, then fade linearly over 200 ms.
why: Documented as Chromium's overlay timings.
@row_none n=163 rec=fct-dxtawno3@1 type=4
relied: Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first.
@row_none n=164 rec=fct-6zlaghie@1 type=4
@row_none n=165 rec=fct-qpwq4lud@1 type=4
relied: A harness settle neither waits on nor advances any animating member and returns Settled with animating reading the document's flag.
@row_add n=35 row=decision/driver-advance-moves-one-clock area=motion rec=fct-e2e2e6vg@1
name: Driver advance moves the one virtual clock
rule: The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was.
why: Ratified.
@row_none n=166 rec=fct-5pxpwwyq@1 type=4
relied: The select list has no motion, so a screen read after a step reads the new state without an advance.
@row_none n=167 rec=fct-335wqsvp@1 type=4
relied: While an acting step waits on a load, the harness virtual clock stands still, so no timer or animation runs during the wait.
@row_none n=168 rec=fct-nukfvayp@1 type=4
@row_none n=169 rec=fct-5d7ppgng@1 type=4
@row_none n=170 rec=fct-kez2bs3i@1 type=5
@row_none n=171 rec=fct-72ty63qd@1 type=7
@row_none n=172 rec=fct-u66y3ree@1 type=5
@row_none n=173 rec=fct-vxrvvp5i@1 type=5
@row_none n=174 rec=fct-bbe5tqa5@1 type=5
@row_none n=175 rec=fct-at5inxcy@1 type=5
@row_none n=176 rec=fct-wixqum32@1 type=5
@row_none n=177 rec=fct-je4n53zx@1 type=5
@row_none n=178 rec=fct-bfdjqju7@1 type=5
@row_none n=179 rec=fct-hajfgs6d@1 type=5
@row_none n=180 rec=fct-6t7vmkf6@1 type=5
@row_none n=181 rec=fct-d3mide3k@1 type=S
@row_none n=182 rec=fct-3gkxk3ju@1 type=5
@row_none n=183 rec=fct-qizuazwl@1 type=5
@row_none n=184 rec=fct-anxjke24@1 type=4
@row_none n=185 rec=fct-7onentxb@1 type=4
@row_add n=36 row=decision/body-right-bottom-margin-ignored area=spacing rec=fct-pzibjwj2@1
name: Body rightmargin and bottommargin ignored
rule: The body attributes rightmargin and bottommargin are deliberately ignored, while marginwidth, marginheight, leftmargin and topmargin map to pixel margins.
@row_none n=186 rec=fct-dqhs7pwt@1 type=4
@row_none n=187 rec=fct-utzmagou@1 type=4
@row_none n=188 rec=fct-lwnjcsh2@1 type=4
relied: An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite.
@row_none n=189 rec=fct-wi4mqvjs@1 type=4
@row_none n=190 rec=fct-tmv65jfj@1 type=4
@row_none n=191 rec=fct-ogsz23id@1 type=4
@row_none n=192 rec=fct-e5awoxxj@1 type=5
@row_none n=193 rec=fct-sdf4zybd@1 type=5
@row_none n=194 rec=fct-tge57kfq@1 type=5
@row_none n=195 rec=fct-otn6nfiv@1 type=5
@row_none n=196 rec=fct-misq73if@1 type=5
@row_none n=197 rec=fct-z4dmblpa@1 type=4
relied: Every document gets the blitz default user-agent stylesheet once, through the default document config.
@row_none n=198 rec=fct-aboeejrv@1 type=4
relied: A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token.
@row_none n=199 rec=fct-cbwqjv4z@1 type=4
@row_none n=200 rec=fct-c73v2tqd@1 type=4
@row_none n=201 rec=fct-eao27mhf@1 type=4
@row_none n=202 rec=fct-jperyeod@1 type=4
@row_none n=203 rec=fct-epbt6wng@1 type=4
relied: A single select's list opens on a click, Enter, or a Space with no typed text standing.
@row_add n=37 row=decision/select-list-engine-overlay area=surface rec=fct-ayatph22@1
name: Select list as an engine overlay
rule: On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes.
@row_add n=38 row=contract/select-list-events/open area=surface rec=fct-gacdn4gi@1
boundary: A single select's list events
item: open
promise: The list opens on its chosen option with no input and no change event.
@row_add n=39 row=contract/select-list-events/highlight-move area=surface rec=fct-gacdn4gi@1
boundary: A single select's list events
item: highlight move
promise: Keys and the pointer move the highlight over enabled rows and choose nothing.
@row_add n=40 row=contract/select-list-events/choose area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose
promise: Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
@row_add n=41 row=contract/select-list-events/choose-disabled-row area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose a disabled row
promise: Choosing a disabled row changes nothing and leaves the list open.
@row_add n=42 row=contract/select-list-events/choose-chosen-option area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose the chosen option
promise: Choosing the option already chosen fires nothing.
@row_add n=43 row=contract/select-list-events/close area=surface rec=fct-uazyshoy@1
boundary: A single select's list events
item: close
promise: Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
@row_add n=44 row=contract/select-list-events/close-on-select-change area=surface rec=fct-uazyshoy@1
boundary: A single select's list events
item: close on its select's change
promise: Removing, disabling, hiding or making its select inert closes the list.
@row_none n=204 rec=fct-ypgr7m5t@1 type=4
relied: A disabled, inert or hidden select, a select with no row to show and a multiple select open no list.
@row_none n=205 rec=fct-ih466day@1 type=4
relied: A focused select whose container lost its box keeps focus and takes a key with no panic.
@row_none n=206 rec=fct-d3xc3mih@1 type=4
@row_add n=45 row=contract/select-accessibility/highlighted-row area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: highlighted row
promise: The highlighted row of an open list is in no snapshot and no tree.
@row_add n=46 row=contract/select-accessibility/open-select area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: open select
promise: The tree reads an open select as ComboBox and expanded.
@row_add n=47 row=contract/select-accessibility/chosen-option area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: chosen option
promise: The tree reads a select's chosen option as selected.
@row_add n=48 row=contract/select-accessibility/open-attribute area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: open attribute
promise: An author's open attribute steers nothing on a select.
@row_add n=49 row=contract/snapshot/select-list-row-bounds area=surface rec=fct-grd6lupt@1
boundary: The snapshot
item: a row of an open select list
promise: A row of an open list carries its own bounds in the snapshot.
@row_add n=50 row=contract/screenshot/select-list-row area=surface rec=fct-grd6lupt@1
boundary: The screenshot verb
item: a row of an open select list
promise: A screenshot of a row of an open list crops to the row and leaves the list open.
@row_none n=207 rec=fct-3zxhbi2k@1 type=4
@row_none n=208 rec=fct-lyaafumo@1 type=4
relied: The logical viewport size is the physical window size divided by hidpi scale times zoom.
@row_add n=51 row=contract/js-window/inner-width-height area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: innerWidth / innerHeight
promise: innerWidth and innerHeight are the window size divided by the scale.
@row_add n=52 row=contract/js-window/outer-width-height area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: outerWidth / outerHeight
promise: outerWidth and outerHeight alias innerWidth and innerHeight.
@row_add n=53 row=contract/js-window/device-pixel-ratio area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: devicePixelRatio
promise: devicePixelRatio is the scale.
@row_none n=209 rec=fct-f4jytyaa@1 type=4
@row_add n=54 row=contract/shell-provider/minimize area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: minimize
promise: The shell provider exposes minimize for custom titlebars.
@row_add n=55 row=contract/shell-provider/maximize area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: maximize
promise: The shell provider exposes maximize for custom titlebars.
@row_add n=56 row=contract/shell-provider/decorations-toggle area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: decorations toggle
promise: The shell provider exposes a decorations toggle for custom titlebars.
@row_add n=57 row=contract/shell-provider/window-drag area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: window drag
promise: The shell provider exposes window drag for custom titlebars.
@row_none n=210 rec=fct-4ylmz5ex@1 type=5
@row_none n=211 rec=fct-qhvfmv23@1 type=4
@row_none n=212 rec=fct-qmikpmeb@1 type=4
relied: The shell sets the window title from the document's title node when it was set before the window existed.
@row_none n=213 rec=fct-zhr7gldu@1 type=4
@row_none n=214 rec=fct-y72ncq2q@1 type=4
@row_none n=215 rec=fct-n5dyjnzk@1 type=5
@row_add n=58 row=contract/dioxus-native-config/alpha-mode area=surface rec=fct-ls27dgne@1
boundary: The dioxus-native config
item: window compositing alpha mode
promise: The window compositing alpha mode is configurable, for example for transparent windows; the renderer ignores unsupported modes.
@row_none n=216 rec=fct-d47jlprw@1 type=4
@row_add n=59 row=decision/buffered-render-replay area=surface rec=fct-zgbhkh7f@1
name: Render buffered, then replayed under one borrow
rule: On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow.
why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
@row_add n=60 row=contract/mounteddata-during-render/document-state area=surface rec=fct-zgbhkh7f@1
boundary: MountedData calls during a render
item: document state
promise: A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it.
@row_add n=61 row=contract/mounteddata-during-render/set-focus-to area=surface rec=fct-zgbhkh7f@1
boundary: MountedData calls during a render
item: set_focus_to
promise: set_focus_to returns false for a node created in the same render.
@row_none n=217 rec=fct-x6nmnnxs@1 type=6
@row_none n=218 rec=fct-gvo7figj@1 type=4
@row_none n=219 rec=fct-omhgtsu2@1 type=4
@row_none n=220 rec=fct-t4zdxxh3@1 type=4
@row_none n=221 rec=fct-ul626qzb@1 type=4
@row_none n=222 rec=fct-pibvawow@1 type=4
@row_add n=62 row=decision/mobile-targets-built-not-tested area=surface rec=fct-zxgw7a6c@1
name: Mobile targets built, not tested
rule: CI builds the iOS and Android targets but runs no tests on them.
@row_add n=63 row=contract/wpt-runner-result-colours/pass area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: PASS
promise: PASS prints green.
@row_add n=64 row=contract/wpt-runner-result-colours/fail-some-passing area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: FAIL with some passing subtests
promise: FAIL with some passing subtests prints yellow.
@row_add n=65 row=contract/wpt-runner-result-colours/fail area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: FAIL
promise: FAIL prints red.
@row_add n=66 row=contract/wpt-runner-result-colours/timeout area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: TIMEOUT
promise: TIMEOUT prints bright red.
@row_add n=67 row=contract/wpt-runner-result-colours/skip area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: SKIP
promise: SKIP prints bright black.
@row_add n=68 row=contract/wpt-runner-result-colours/crash area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: CRASH
promise: CRASH prints bright magenta.
@row_add n=69 row=decision/wpt-runner-secondary-text-bright-black area=surface rec=fct-yhgw2pup@1
name: WPT runner secondary text in bright black
rule: The WPT runner prints test kind, flag markers and summary section headings in bright black.
@row_add n=70 row=contract/wpt-runner-output/test-name-hyperlink area=surface rec=fct-uompn4jm@1
boundary: WPT runner output
item: test name
promise: A test name is wrapped in an OSC 8 hyperlink to https://wpt.live/{name} only when stdout supports hyperlinks (a terminal, honouring FORCE_HYPERLINK).
@row_none n=223 rec=fct-h7373ed2@1 type=4
@row_none n=224 rec=fct-qlphyfkz@1 type=4
@row_add n=71 row=decision/escher-session-agent-read-surface area=surface rec=fct-6fb4pz6c@1
name: escher-session is read by an agent, not a person
rule: An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
@row_add n=72 row=decision/escher-session-no-colour area=surface rec=fct-6fb4pz6c@1
name: No colour and no terminal test on escher-session
rule: escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not.
guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
@row_add n=73 row=contract/escher-session-stderr/usage-error area=surface rec=fct-6fb4pz6c@1
boundary: escher-session stderr
item: usage error
promise: A usage error prints one fixed line on stderr with stdout empty.
@row_add n=74 row=contract/escher-session-stderr/session-error area=surface rec=fct-6fb4pz6c@1
boundary: escher-session stderr
item: session error
promise: A session error prints its fixed message on stderr.
@row_add n=75 row=contract/escher-session-exit-status/0 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 0
promise: Accepted.
@row_add n=76 row=contract/escher-session-exit-status/1 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 1
promise: Refused.
@row_add n=77 row=contract/escher-session-exit-status/2 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 2
promise: Usage.
@row_add n=78 row=contract/escher-session-exit-status/3 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 3
promise: Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
@row_add n=79 row=decision/exit-statuses-from-schema-table area=surface rec=fct-6fb4pz6c@1
name: Exit statuses read from the schema's table
rule: The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
@row_add n=80 row=contract/escher-session-commands/help area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: help
promise: help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
@row_add n=81 row=contract/escher-session-session-error/remedy area=surface rec=fct-6fb4pz6c@1
boundary: escher-session session error object
item: remedy
promise: A session error's object carries its fixed remedy.
@row_add n=82 row=contract/escher-session-commands/screenshot area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: screenshot [--id]
promise: screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
@row_add n=83 row=contract/escher-session-commands/snapshot area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: snapshot [--part <part>]
promise: snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
@row_add n=84 row=contract/escher-session-mcp/stdout area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: stdout
promise: stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
@row_add n=85 row=contract/escher-session-mcp/snapshot-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: snapshot tool result
promise: A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
@row_add n=86 row=contract/escher-session-mcp/help-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: help tool result
promise: help's tool result is the self-description's line.
@row_add n=87 row=contract/escher-session-mcp/screenshot-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: screenshot tool result
promise: A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
@row_add n=88 row=contract/escher-session-mcp/other-results area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: other tool results
promise: Any other tool result's text is the command's JSON line.
@row_add n=89 row=contract/escher-session-mcp/err-ending area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: Err ending
promise: An Err ending's fixed message goes to stderr with status 3.
@row_add n=90 row=contract/escher-session-tasks/counter area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: counter
promise: counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=91 row=contract/escher-session-tasks/flight-booker area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: flight-booker
promise: flight-booker is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=92 row=contract/escher-session-tasks/timer area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: timer
promise: timer is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=93 row=contract/escher-session-tasks/crud area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: crud
promise: crud is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=94 row=contract/escher-session-tasks/cells area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: cells
promise: cells is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=95 row=contract/escher-session-tasks/load area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: load
promise: load is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=96 row=contract/escher-session-tasks/unknown-task area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: unknown task
promise: An unknown task is a usage ending.
@row_add n=97 row=contract/escher-session-commands/graph-arguments area=surface rec=fct-gnekz77j@1
boundary: escher-session commands
item: graph arguments
promise: graph takes no argument; an argument is refused malformed with nothing run.
@row_add n=98 row=contract/escher-session-commands/graph-answer area=surface rec=fct-gnekz77j@1
boundary: escher-session commands
item: graph answer
promise: graph answers the screen's whole UI graph as one uncoloured line of JSON {"nodes":[…],"states":[…]} on stdout, status 0, the same on the session socket's call and in a kept check; over the mcp role the tool result is the text block of that line.
@row_add n=99 row=contract/ui-graph-line/content area=surface rec=fct-gnekz77j@1
boundary: The UI graph line
item: content
promise: The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a value or typed text.
@row_add n=100 row=contract/escher-session-commands/actions-arguments area=surface rec=fct-rqe4xefm@1
boundary: escher-session commands
item: actions arguments
promise: actions takes no argument; an argument is refused malformed.
@row_add n=101 row=contract/escher-session-commands/actions-answer area=surface rec=fct-rqe4xefm@1
boundary: escher-session commands
item: actions answer
promise: actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent.
@row_add n=102 row=contract/actions-line/nodes area=surface rec=fct-rqe4xefm@1
boundary: The actions line
item: nodes
promise: Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with.
@row_add n=103 row=contract/actions-line/content area=surface rec=fct-rqe4xefm@1
boundary: The actions line
item: content
promise: The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId.
@row_add n=104 row=decision/actions-share-the-verbs-checks area=surface rec=fct-cu455asz@1
name: actions decided by the verbs' own checks
rule: The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts.
why: so a listed cause is the cause the call returns
@row_add n=105 row=contract/actions-line/stale area=surface rec=fct-cu455asz@1
boundary: The actions line
item: stale
promise: A node that resolves to no element reads stale.
@row_add n=106 row=contract/escher-session-commands/coverage-answer area=surface rec=fct-j6lb6pph@1
boundary: escher-session commands
item: coverage answer
promise: coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
@row_add n=107 row=contract/coverage-line/content area=surface rec=fct-j6lb6pph@1
boundary: The coverage line
item: content
promise: The line carries ids, owner ids, event names and two counts only, owner left out when absent.
@row_add n=108 row=contract/escher-session-commands/coverage-arguments area=surface rec=fct-y7unb4g4@1
boundary: escher-session commands
item: coverage arguments
promise: coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2.
@row_add n=109 row=contract/coverage-check-files/read-bound area=surface rec=fct-y7unb4g4@1
boundary: coverage check files
item: read bound
promise: Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly.
@row_add n=110 row=contract/coverage-check-files/refusal area=surface rec=fct-y7unb4g4@1
boundary: coverage check files
item: refusal
promise: A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file.
@row_none n=225 rec=fct-bzczuomv@1 type=9
@row_none n=226 rec=fct-wvdzkcqu@1 type=9
@row_none n=227 rec=fct-axilysek@1 type=9
@row_add n=111 row=contract/native-config/wasm-font-context area=surface rec=fct-4our3qxs@1,fct-pso5k3lx@1
boundary: dioxus-native configuration
item: font context on WASM
promise: On WASM a font context with bundled fonts must be provided; `build_single_font_ctx` serves where one font is bundled.
@row_none n=228 rec=fct-cp64cmtj@1 type=5
@row_none n=229 rec=fct-cv7cmt4u@1 type=5
@row_none n=230 rec=fct-clshjihy@1 type=5
@row_none n=231 rec=fct-fbgjjqxv@1 type=5
@row_none n=232 rec=fct-iotxo33d@1 type=5
@row_none n=233 rec=fct-swhtsbga@1 type=5
@row_none n=234 rec=fct-biisvyv5@1 type=5
@row_none n=235 rec=fct-lsdv4mfo@1 type=5
@row_none n=236 rec=fct-cqaj3usi@1 type=5
@row_none n=237 rec=fct-4vcrqhgt@1 type=5
@row_none n=238 rec=fct-nrlvlxxd@1 type=5
@row_none n=239 rec=fct-mpgheext@1 type=5
@row_none n=240 rec=fct-pxd6cx5y@1 type=5
@row_none n=241 rec=fct-qgjffy5g@1 type=5
@row_none n=242 rec=fct-pvknxsve@1 type=5
@row_none n=243 rec=fct-xmoyplif@1 type=4
relied: WASM builds render sans-serif, serif, monospace and system-ui text with the bundled DejaVu Sans.
@row_none n=244 rec=fct-vdyur6r3@1 type=S
@row_add n=112 row=contract/native-config/font-context area=typography rec=fct-pso5k3lx@1
boundary: dioxus-native configuration
item: FontContext
promise: A custom `FontContext` can be set.
@row_none n=245 rec=fct-nk2xx5j6@1 type=6
@row_none n=246 rec=fct-plskesan@1 type=4
relied: List bullets can always draw from the bullet font, which the default font context always registers.
@row_add n=113 row=contract/driver-screenshot/fonts-unavailable area=typography rec=fct-okan27km@1
boundary: driver screenshot
item: fonts-unavailable
promise: A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
@row_add n=114 row=contract/script-runtime/document-fonts area=typography rec=fct-5cgh22bd@1
boundary: script runtime DOM
item: document.fonts
promise: `document.fonts` is a stub FontFaceSet in which all fonts report as loaded.
@row_add n=115 row=decision/tests-built-with-system-fonts area=typography rec=fct-zmi56wtl@1
name: Tests built with system fonts
rule: The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_none n=247 rec=fct-mrtzanbs@1 type=4
relied: An element with no generic font family is laid out as sans-serif.
@row_none n=248 rec=fct-wgirrymc@1 type=4
@row_none n=249 rec=fct-zjuxxuk2@1 type=4
@row_none n=250 rec=fct-va3exjt4@1 type=4
relied: Layout treats `line-height: normal` as 1.2 times the font size.
@row_none n=251 rec=fct-tigq2ssu@1 type=4
@row_none n=252 rec=fct-qrvvxmts@1 type=4
@row_none n=253 rec=fct-ua34srkn@1 type=4
relied: The `lang` attribute decides the locale text is shaped and broken with.
@row_none n=254 rec=fct-nwz2uowu@1 type=4
@row_none n=255 rec=fct-gcmoldbw@1 type=4
@row_add n=116 row=decision/root-relative-units-track-root-font area=typography rec=fct-dp73pjt4@1
name: Root-relative units track the root font
rule: rem and rlh units resolve against the root element's font size and line height, also after viewport and hidpi changes, with an initial root font-size of 16px.
why: The stylist device is seeded with the root's values so rem/rlh units do not fall back to the 16px default.
guard: rem_after_viewport_change
@row_none n=256 rec=fct-jjpum73a@1 type=4
@row_none n=257 rec=fct-fwngn32o@1 type=4
@row_none n=258 rec=fct-etlx3awr@1 type=4
relied: Every caller other than the select group header passes the default run paint and adds no synthesized bold.
@row_none n=259 rec=fct-ty77ehpa@1 type=4
@row_none n=260 rec=fct-n7jzo7rd@1 type=5
@row_none n=261 rec=fct-6xglreut@1 type=5
@row_none n=262 rec=fct-4ofq4qli@1 type=5
@row_none n=263 rec=fct-ft4lxsdx@1 type=5
@row_none n=264 rec=fct-o6ppnb4n@1 type=5
@row_none n=265 rec=fct-cj7mlii5@1 type=5
@row_none n=266 rec=fct-yjq7pu4h@1 type=7
@row_none n=267 rec=fct-csqoaj2v@1 type=7
@row_none n=268 rec=fct-yjvyslbr@1 type=7
@row_add n=117 row=absence/project-type-scale area=typography rec=fct-65pzcpcs@1
capability: A project type scale or font tokens
@row_add n=118 row=decision/driver-scroll-instant area=motion-calibrated-to-expression-level-unrecorded rec=fct-tnwqvola@1,fct-vt52hpbo@1
name: Driver scroll is instant
rule: The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step.
why: Ratified.
@row_add n=119 row=absence/reduced-motion-handling area=motion-calibrated-to-expression-level-unrecorded rec=fct-edwhene2@1,fct-vvuq2clh@1
capability: Reduced-motion handling
@row_add n=120 row=decision/cursor-resolution area=iconography rec=fct-hvyxlxhq@1,fct-guxvotwt@1
name: Cursor resolution order
rule: The cursor is taken from the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included, so the shell keeps a visible arrow there.
why: Taken from upstream's #1220, ratified.
