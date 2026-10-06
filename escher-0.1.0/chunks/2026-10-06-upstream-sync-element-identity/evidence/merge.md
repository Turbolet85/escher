# Merge record — upstream/main into build/escher-0.1.0

Read 2026-10-06T08:12Z, /implement steps 2–3.

## Pin
- `git rev-parse upstream/main` → `2335458530518cdf167c55ce635fae99323e0789` (the pin, unmoved)
- `git ls-remote upstream refs/heads/main` → `2335458530518cdf167c55ce635fae99323e0789` (live read, unmoved)
- `git merge-base HEAD upstream/main` → `0f60502ea724ef703b34220b3847bb3483f70b5f`
- First parent (HEAD at merge): `ab936a9e46ee19365929465444725a0bc837dbbc`

## Upstream commits (`git log --oneline 0f60502e..23354585`, 11)
```
23354585 Update Taffy for cyclic percentage grid minimum contributions (#1070)
d3ced119 Add `children` accessor to Document (#1069)
6a30c04a Update WPT pin to c271c10de (#1068)
3aa87bc1 fix(wpt): include root titles in native checkLayout subtest names (#1067)
11f2ecf1 Implement `align-content` for block, inline and table (Taffy #1251) (#1059)
30bb22e0 Expose `Node::inline_fragment_boxes` for inline CSSOM geometry + misc. (#1060)
6832e9cb Bump Parley to latest main (break-lines speedup) (#1065)
420abe39 Use Taffy's first-class `normal` alignment keyword in `stylo_taffy` (#1063)
6a15a4d7 Move inline fragment rect computation to Node without changing behavior (#1062)
73fb6a16 Bump Taffy to fix used auto margins beside floats (#1061)
0b5f1fe2 Support self-alignment (align-self/justify-self) of absolutely positioned boxes (#977)
```

## The merge
- `git merge --no-ff --no-commit 2335458530518cdf167c55ce635fae99323e0789` → exit 0, "Automatic merge went well; stopped before committing as requested". Auto-merged: `Cargo.lock`, `Cargo.toml`, `packages/blitz-dom/src/node/node.rs`. No conflict.
- `MERGE_HEAD` → `2335458530518cdf167c55ce635fae99323e0789`
- Staged: the 17 upstream files of research.md §Files to modify, no other.

## Worktree equals the P3 trial tree
- `git merge-tree --write-tree ab936a9e… 23354585…` → `aed58ca83233514f2ca2a5d685d8fe8ca77535dd` (same as P3)
- `git diff --quiet aed58ca8 -- . ':(exclude)escher-0.1.0' ':(exclude).andromeda' ':(exclude).claude'` → exit 0 (worktree)
- `git diff --cached --quiet aed58ca8 -- …` (same pathspecs) → exit 0 (index)

## Lockfile summary
- `git diff --stat ab936a9e -- Cargo.lock` → 1 file changed, 7 insertions(+), 7 deletions(-)
- `^[-+]source = ` lines: 14 (7 moved: 6 parley-repo crates, 1 taffy)
- `^[-+]name = ` lines: 0. The 14 changed lines are all `source =` lines (7 + 7 = the whole diffstat), so every crate's name and version is unchanged
