@row_set n=1 row=decision/font-face-css-descriptors cell=rule seen=1 fact=F1 detector=D-arch-decisions
value: `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching.
basis: fct-xojovjqq, fct-ysqotf4p
because: The take adds the `font-stretch` descriptor to the registered overrides, ratified as a behaviour change for every document, so the row naming only family, weight and style says it less.
@row_set n=2 row=decision/no-git-dependencies cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: No git dependency is left in the manifest and every crate is a crates.io registry version, save the taffy crate alone, which may come from a git source and still passes the lock, audit and MSRV legs.
basis: fct-xv7xpnit
because: The founder's answer to Q1 makes taffy alone an exception to the no-git-dependency rule, so the row stating no exception must carry it.
@row_set n=3 row=decision/deferred-upstream-dependency-bumps cell=rule seen=1 detector=D-arch-decisions
value: Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version; the Taffy-main content comes through registry Taffy 0.15.x save #1303, which waits with the later git Taffy chain for its own chunk.
basis: fct-vixmrsy5, fct-xv7xpnit
because: Registry taffy 0.15.1 carries the Taffy-main commits but #1303, and that remainder now waits for a git-Taffy entry, so the row saying all the Taffy-main commits wait is stale.
@row_set n=4 row=decision/mutation-tier-host-only cell=except seen=1 fact=F3 detector=none
value: rdme and blitz, upstream's demo shells holding no tests, are not run by the mutation tier and owe no test for it; churn, sizes and the other collectors keep both units.
basis: fct-wuhhml6l
because: The founder rules the two test-less units out of the mutation tier's scope, and the code audit reads its extra exclusions only from this architecture row.
