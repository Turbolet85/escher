@fit n=1 row=decision/circle-drawer-click-selects-or-adds sha=d431c206364b v=fits type=1
@fit n=2 row=decision/circle-drawer-dialog-disables-other-circles sha=ef12e92b844e v=fits type=1
@fit n=3 row=decision/circle-drawer-canvas-keeps-size sha=05f44de9e596 v=fits type=1
@fit n=4 row=decision/circle-drawer-canvas-selection-attributes sha=35fbea10bc2e v=not_master type=4
why: How Circle Drawer's app hands selection state to its own canvas widget through two attributes is internal plumbing of one mechanism, and its basis gives no reason for it, so the code holds it.
