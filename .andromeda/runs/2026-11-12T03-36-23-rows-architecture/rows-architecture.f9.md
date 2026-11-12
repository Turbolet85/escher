@row_add n=1 row=decision/scroll-into-view-whole-chain area=established-decisions rec=1,2
name: Scroll into view writes the whole containing-block chain
rule: Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone.
why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it.
except: a row of an open select list, which moves only the list's own scroll offset
@row_add n=2 row=decision/hit-stops-at-paint-clip area=established-decisions rec=1,2
name: Hit walk stops where paint clips
rule: A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk.
why: Uses paint's own clipping predicate so hit and paint agree; ratified.
guard: hit_clipped_at_scrolling_box
except: the root element
@row_add n=3 row=absence/dioxus-native-js-eval area=established-decisions rec=3
capability: JS eval from a Dioxus native app
