# A11y validation — route draft

## Insert
- Between `Circle Drawer's circles as virtual nodes` and `Upstream sync ahead of drag, pan and zoom`: **"Canvas keyboard path — Circle Drawer's circles reached and acted on by keyboard; engine, snapshot and tree focus agree; Tab order in the shared table"** (epoch: `Epoch 4`)
  Reason: The Polish chunk `A11y re-read` re-checks "the canvas samples' keyboard paths", but no feature chunk builds one; tier 0 requires SC 2.1.1 and 2.4.3 (a11y-plan §1 decision/a11y-tier-0) and requires stated Tab orders to be walked against the shared table (§5 decision/stated-tab-order-walked, §8 decision/stand-shared-tab-order-table).

## Rewrite
- `Virtual nodes as elements`: "in the snapshot and the diff" → "in the accessibility tree, and from it the snapshot and the diff"
  Reason: The snapshot takes role, name and focus from the accessibility tree and from nothing else, and has no second role or name mapping (a11y-plan §7 decision/snapshot-reads-the-tree, decision/readers-of-one-snapshot-model), so virtual nodes that bypass the tree would break that contract.
- `Virtual nodes declared`: "author key, role, name and region inside a canvas" → "author key, role, non-empty name and region inside a canvas; roles from the stated expectation tables"
  Reason: Every interactive-role node must have a non-empty name (a11y-plan §7 decision/interactive-nodes-named), and expected roles and names are stated only once, in the shared tables (§7 decision/stand-expectation-tables).
- `Node-graph stand app`: "nodes dragged, a wire pulled, the surface panned and zoomed" → "nodes dragged, a wire pulled, the surface panned and zoomed; a keyboard path for each, or a stated SC 2.1.1 limit"
  Reason: Tier 0 includes SC 2.1.1 (a11y-plan §1 decision/a11y-tier-0), and drag, pan and zoom are pointer-only unless a keyboard path exists or the gap is recorded as a stated limit.
- `A11y re-read`: "re-asserted in the `a11y` leg (a11y-plan §9)" → "re-asserted in the `a11y` leg and stand-repeat for SC 2.1.1 + 2.4.3 + 1.4.3, the node-graph app included (a11y-plan §9)"
  Reason: Verification chunks must name the tier-0 SC IDs (a11y-plan §1 decision/accessibility-reach-tier-0), the stand checks also run three rounds in stand-repeat (§9 decision/stand-checks-repeated-three-rounds), and the node-graph app is the second new surface. The plan's §Bootstrap item (decision/a11y-ci-gate, the `a11y` CI job) already exists, so this Rust/AccessKit project needs no bootstrap-chunk Insert.
- `Capability statement`: "what 0.4.0 adds and its stated limits" → "what 0.4.0 adds and its stated limits, its Accessibility reach section updated by hand for canvas and virtual nodes"
  Reason: Changes to accessibility reach are written by hand into that section because no check reads it (a11y-plan §1 decision/capability-statement-reach-by-hand).
- `Upstream sync ahead of the debts`: "merge-surface residuals re-read" → "merge-surface residuals re-read; CI, a11y leg and audit green"
  Reason: Every later sync chunk gates on the `a11y` leg (a11y-plan §3 decision/a11y-ci-leg), but this first merge is the only one that leaves that gate out.
