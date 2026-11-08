@answer n=1 entity=visual-design-verification#8.6 v=yes
why: a pointerdown in the shown scrollbar band right after a scroll dispatches nothing although action_point accepts it, so the read's accepted-now verdict for a node in that band must agree with a real call (hit-testing, scope item 8).
@answer n=2 entity=visual-design-verification#8.7 v=yes
why: an open list covers the elements under it and a click there is refused covered, the cause the read returns for those nodes and the owner-served rows of item 9.
@answer n=3 entity=visual-design-verification#8.8 v=yes
why: the open list is the topmost painted layer, so the covered cause for a target under it must follow that paint order, and the blend-layer order it leaves unmeasured is a fork the stand must not assert.
@answer n=4 entity=visual-design-verification#8.9 v=no
why: devtools overlay settings are not painted over or hit-tested onto any listed node, and the chunk reads no devtools setting.
