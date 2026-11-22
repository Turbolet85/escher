Question (implement P2, the url lock move breaking bindgen): How should the chunk go on?

Answer: "Keep syn edges (Recommended)"

Notes: Conditions: (1) the lock must be one cargo accepts unchanged — cargo metadata --locked AND a full workspace build under --locked, both measured, before the block re-runs; (2) do not leave entry 5 red: mend its expectation in plan.md to the move actually made (the url and form_urlencoded lines only), as an operator-directed gate edit recorded in evidence with the reason, then it must read green; (3) record the latent hazard — any plain cargo update re-resolves the two syn edges and breaks bindgen — in the report as a residual candidate for the wrap, with the reproducing command.
