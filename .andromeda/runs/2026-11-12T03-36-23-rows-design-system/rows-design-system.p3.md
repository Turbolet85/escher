@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=5
@row_none n=9 rec=9 type=5
@row_none n=10 rec=10 type=5
@row_none n=11 rec=11 type=4
@row_none n=12 rec=12 type=4
@row_none n=13 rec=13 type=5
@row_none n=14 rec=14 type=5
@row_none n=15 rec=15 type=5
@row_none n=16 rec=16 type=4
@row_none n=17 rec=17 type=5
@row_add n=1 row=absence/engine-color-tokens area=color-palette rec=18
capability: Color tokens or CSS custom properties in the engine and integration crates
@row_none n=18 rec=19 type=5
@row_none n=19 rec=20 type=5
@row_none n=20 rec=21 type=5
@row_none n=21 rec=22 type=5
@row_none n=22 rec=23 type=5
@row_none n=23 rec=24 type=5
@row_none n=24 rec=25 type=5
@row_none n=25 rec=26 type=5
@row_none n=26 rec=27 type=5
@row_none n=27 rec=28 type=7
@row_none n=28 rec=29 type=5
@row_none n=29 rec=30 type=5
@row_none n=30 rec=31 type=5
@row_none n=31 rec=32 type=5
@row_none n=32 rec=33 type=5
@row_none n=33 rec=34 type=5
@row_none n=34 rec=35 type=5
@row_none n=35 rec=36 type=S
@row_none n=36 rec=37 type=5
@row_none n=37 rec=38 type=5
@row_none n=38 rec=39 type=5
@row_none n=39 rec=40 type=4
@row_none n=40 rec=41 type=4
relied: Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root.
@row_add n=2 row=decision/hit-test-stops-at-paint-clip area=depth-strategy rec=42
name: Hit-testing stops where paint clips
rule: Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box.
why: Ratified; a hit answers only what paint shows.
guard: hit_clipped_at_scrolling_box
except: The root element.
@row_none n=41 rec=43 type=4
relied: hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered.
@row_none n=42 rec=44 type=4
@row_none n=43 rec=45 type=4
@row_add n=3 row=decision/auto-z-index-shared-paint-level area=depth-strategy rec=46
name: Positioned auto z-index paint level
rule: Positioned descendants with z-index auto share one paint level and paint in tree order.
why: CSS 2.1 Appendix E
guard: paint_order
@row_none n=44 rec=47 type=4
relied: The open select list paints after the root's render and the blend layers, so it stands above every box of the page.
@row_none n=45 rec=48 type=4
@row_none n=46 rec=49 type=4
@row_none n=47 rec=50 type=4
@row_none n=48 rec=51 type=4
@row_none n=49 rec=52 type=4
@row_none n=50 rec=53 type=5
@row_none n=51 rec=54 type=4
@row_none n=52 rec=55 type=5
