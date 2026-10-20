# Readings — 2026-10-19-visual-lint

Measured by `/andromeda-implement` (run `.andromeda/runs/2026-10-20T00-14-31-implement/`), on the working tree over
base e97a7f41b, linux x86_64, both layout modes in each reading.

## The stand at boot (plan step 6)
`the_stand_at_boot_reads_no_finding` (tests/blitz-tests/tests/stand_lint.rs), gate entry 3 green:

| task | incremental=false | incremental=true |
|---|---|---|
| Counter | 0 findings | 0 findings |
| Flight Booker | 0 findings | 0 findings |
| Timer | 0 findings | 0 findings |
| CRUD | 0 findings | 0 findings |

The take-up's prediction holds: no overlap, clip, off-screen or zero-size reading (the five no-area nodes
`task-header-spacer` ×4 and `timer-progress` hold no content under T1). The two readings research left unmeasured
read nothing too: no truncated text on any task, and no contrast finding on any node outside `stand_contrast`'s rows.
No stand finding was taken to the operator, because none was read.

## The planted page (plan step 5)
`the_planted_defects_are_named_each_by_its_id_and_reason`, gate entry 3 green. The seven findings, in document order,
read once through a temporary diagnostic message on incremental=false (the file restored from a scratch copy; the
reading was taken before the page's element order was corrected to the plan's, so `fx-offscreen` printed last there,
and the green run after the fix asserts the order below):

| row | id | defect | reason as measured |
|---|---|---|---|
| 0 | fx-overlap-a | overlap | other `fx-overlap-b`, intersection 40 × 20 |
| 1 | fx-clipped | clipped | bounds 200 wide, visible 100 wide, inside `fx-clip-box` |
| 2 | fx-paint-clipped | clipped | bounds 200 wide, visible 100 wide, inside `fx-paint-box` (`contain: paint`, the lint's own walk) |
| 3 | fx-truncated | truncated | text box 200.6953125 wide against bounds 80 |
| 4 | fx-offscreen | off-screen | — |
| 5 | fx-zero | zero-size | bounds 0 × 0 |
| 6 | fx-low-contrast | contrast | ratio 2.8490272 (equal to `Harness::text_contrast`'s), required 4.5 |

The six negatives (`fx-spacer`, `fx-inert`, `fx-hidden`, `fx-disabled`, `fx-faded`, `fx-long-input`, `fx-wrapped`)
read nothing: the test asserts the list equals exactly the seven rows.

## The CRUD agreement (plan step 8)
`the_rows_cut_by_the_list_agree_with_off_screen`, gate entry 3 green. After twelve Creates, the last row
`crud-person-14` reads `clipped` (one finding of the two words), bounds y 519.796875 · 367 × 29, visible 367 ×
11.203125 inside `crud-list`'s box (24, 112.796875, 369 × 419.203125). A `click` on it is refused `off-screen`: its
centre lies past the list's box.

## The text extent reader (plan step 2)
`tests/blitz-tests/tests/text_extent.rs`, gate entries 1 and 11 green: the nowrap paragraph's content box reads 80 px
and its text box wider, the wrapped one's text box reads inside it, and an empty block and a removed id read `None`.
