@answer n=1 entity=design-philosophy#1.1 v=yes
why: the read is a new public read through dioxus-native-dom's graph and snapshot, which must stay separately usable and headless for external code to drive
@answer n=2 entity=design-philosophy#2.1 v=no
why: the read reaches no networking, navigation, shell or parser provider, and the scope names none of them
@answer n=3 entity=design-philosophy#3.1 v=yes
why: the read's disabled, covered, owner-served select and clickable semantics are web behaviour the new code must state against named specs
@answer n=4 entity=design-philosophy#4.1 v=yes
why: the read keys nodes by stable id over versioned node identity and must hold no NodeId across the driver boundary
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the scope proves the read headless on the stand, and the no-window headless harness rule governs the chunk's tests
