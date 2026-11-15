# P5 review — 2026-11-15-browser-corrective «Browser corrective»

The plan: `doc:2026-11-15-browser-corrective/plan` (rev 3). To read it in an editor:
`.andromeda/cache/docs/2026-11-15-browser-corrective/plan.md`. No plan stands in a chunk's folder, and writing in that
file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-15-browser-corrective` diffs the chunk's records.

## Goal
Three unit tests in the browser app (`document_loader.rs`, a new inline test module) check the config that
`make_doc_config` returns: its page URL, its net provider (the very one passed), and the `file:` gate's two directions
through its own navigation provider. Together they catch the Epoch 4 audit's three mutation survivors (`:52:5`,
`:56:9`, `:57:9`). Two comment lines justify the two bare `#[allow(clippy::expect_used)]` (`favicon.rs:76`,
`tab.rs:178`). Browser app only: tests and comments, no behaviour change, no log site, print, env read or dependency.

## Top acceptance criteria
- The three new tests pass, found in the browser crate's one test binary.
- cargo-mutants over `make_doc_config`'s 11 mutants prints no line for `:52:5`, `:56:9` or `:57:9`, so all three are
  caught. Baseline: all 11 missed. Prediction: 7 missed, since the `base_url` deletion falls too.
- Every browser `expect_used` allow stands below a justification line. The awk probe reads 2 now and must read 0.
- No added log, print or env line in `apps/browser`; `Cargo.lock` unchanged. `fast` passes, and after the operator's
  push CI reads green on HEAD.

## Touchpoints
- `apps/browser/src/document_loader.rs`: append the test module (no existing line changes)
- `apps/browser/src/favicon.rs`: add one comment line
- `apps/browser/src/tab.rs`: add one comment line

## Test commands (11 entries)
1. `cargo test -p browser --locked` (the three names)
2. its `--list`
3. the allow-justification awk count
4. the silence diff from `73a168d5a`
5. `Cargo.lock` and the manifest unchanged
6. `cargo nextest run -p browser --locked`
7. cargo-mutants `-F make_doc_config`, under the mutants profile with `--baseline=skip` (host only)
8. `ci-leg.sh fast`
9–11. the operator's hygiene, push and CI read

## What P5 found and folded (a plan edit; every check was re-run after it)
The first form of entry 7 named each survivor by `--re`. Its baseline showed the filter does not narrow: cargo-mutants
27.1.0 ran all 11 mutants of the function, and `-F 'net_provider'` alone selects all ten field deletions. The entry now
runs `-F make_doc_config` (exactly the 11, read with `--list`) and asserts the three survivors by coordinate, absent
from the not-caught lines. Its exit is not asserted: the equivalent deletions `viewport: None` and `media_type: None`
always read missed. Whatever the run measures for the other five field deletions goes in the report for the wrap to
route. They are outside the CARRY, and the audit's in-diff run never planned them.

## Controls (`p5-controls.md`)
- Probe 3: red 2 on the tree, 0 in a scratch clone with the two comment lines added.
- Probe 4: red on an added `println!`, green on added comments.
- Probe 5: red on an edited `Cargo.lock`.

## Capabilities
None claimed. The four unclaimed caps (`v030-10`, `-11`, `-12`, `-15`) each name a surface this chunk does not build.

## Expected amendments (wrap)
- architecture `decision/justified-expect-unwrap`: its `except` is dropped.
- test-plan `decision/file-urls-only-for-file-documents`: its guard may name the new navigation tests.
- test-plan §1/§4: `document_loader.rs` joins the browser's tested files.

## Notes
- Plan prose is about 125 lines, under the ~150 guide. The chunk is small: three files, tests and comments.
- No fork was asked, because every lean was decisive. No escalate pattern applies.

## Beyond the plan
none

Apply? (yes / review / cancel)
