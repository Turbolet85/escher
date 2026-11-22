@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=14 class=omission
leaf-text: its merged output lands in `target/ci-logs/{leg}.log`; unknown leg → exit 2
source: dec-ivlt63uv «The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive for the rustc host and export SKIA_BINARIES_URL, and a refused archive ends»
replace-with: the `clippy`, `test`, `build`, `msrv`, `doc`, `coverage` and `a11y` legs each first verify skia's prebuilt archive for the rustc host and export `SKIA_BINARIES_URL`, a refused archive ending the leg before any cargo runs; its merged output lands in `target/ci-logs/{leg}.log`; unknown leg → exit 2
@leaf_edit n=2 leaf=.claude/docs/commands.md line=13 class=omission
leaf-text: its line printed and its exit discarded; a red leg calls no sweep
source: dec-d4sn2vhm «Every cargo call in a skia-building leg, the fast leg's closing target sweep included, runs under the verified SKIA_BINARIES_URL.»
replace-with: its line printed and its exit discarded, the sweep's cargo calls run under the verified `SKIA_BINARIES_URL`; a red leg calls no sweep
@leaf_edit n=3 leaf=.claude/docs/commands.md line=15 class=omission
leaf-text: called by ci.yml's install steps only (no leg calls it; never run on the dev host)
source: dec-mt5ihu7p «except: The test matrix installs its one package through a SHA-pinned apt cache action under its step timeout, and the Android job sets up its NDK through a SHA-pinned setup-ndk action.»
replace-with: called by ci.yml's install steps only (no leg calls it; never run on the dev host; the test matrix installs its one package through a SHA-pinned apt cache action under its step timeout instead, and the Android job sets up its NDK through a SHA-pinned setup-ndk action)
@leaf_edit n=4 leaf=.claude/docs/commands.md line=62 class=omission
leaf-text: Enter and Space opening the list and a released Space doing nothing more
source: dec-7ef5o35w «A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list, arrows, Home, End and typeahead choose and scroll the row into»
replace-with: Enter and Space opening a drop-down's list and a released Space doing nothing more, a single select whose size parses above 1 an in-flow list box that opens no list, its arrows, Home, End and typeahead choosing and scrolling the row into view and a row click choosing that row alone
