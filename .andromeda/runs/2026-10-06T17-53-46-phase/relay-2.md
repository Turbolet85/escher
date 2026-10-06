Question: Upstream main is still at 23354585 (the last sync's pin), so there are 0 commits to merge. How should the 'Upstream sync ahead of the observation model' entry be handled?
Answer: "Measured no-op (Recommended)" — Promote it as a light chunk: record that upstream/main = 23354585 at take-up (ls-remote, 0 ahead), no merge commit. The proof is CI green on HEAD 42b80ad9 (16/16) plus the local fast+doc gate.
Operator notes: Keep it as light as the pipeline allows: record the measured upstream pin and the 0-ahead read, no merge commit, and no work beyond what the gates require.
