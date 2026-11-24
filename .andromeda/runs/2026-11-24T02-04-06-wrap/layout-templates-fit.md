@fit n=1 row=decision/stand-lints-clean-at-boot sha=e0fbbc450b2a v=fits type=1
@fit n=2 row=contract/stand-element-ids/circle-canvas sha=6e172d9dca5e v=not_master type=4
why: The added element kind, role and name retell how the canvas is built rather than the stable id promised, and the code holds them.
@fit n=3 row=decision/circle-drawer-circles-painted-on-canvas sha=85a043308d91 v=fits type=1
@fit n=4 row=decision/circle-drawer-diameter-label sha=be2b578e441a v=not_master type=4
why: How the canvas widget paints its diameter label (integer, 12 px sans-serif, centred) is one mechanism whose basis states no reason, and the code holds it.
