@answer n=1 entity=visual-design-verification#8.6 v=no
why: the rule is a driver click's pointerdown in a shown scrollbar band right after a scroll; the chunk's scope names scroll only as a MountedData call, and no test or code it adds clicks a scrollbar band, so the chunk does not reach it.

@answer n=2 entity=visual-design-verification#8.7 v=no
why: the rule is the open select list's paint over an opacity ancestor and driver clicks refused as covered; the chunk changes no select control, list paint or pointer path, and the select list is only painted through, not named by the scope.

@answer n=3 entity=visual-design-verification#8.8 v=no
why: the rule is the open list's paint order past mix-blend-mode and isolation layers in render_element; the buffered render replays mutations and touches no paint order, and the scope names no blend or isolation check.

@answer n=4 entity=visual-design-verification#8.9 v=no
why: the rule is that devtools layout outlines and hover overlays exist as settings in blitz-traits; the chunk neither reads nor changes those settings and the scope names no devtools check.
