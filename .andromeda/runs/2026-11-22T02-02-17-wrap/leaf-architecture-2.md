@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/stack.md line=42 class=omission
leaf-text: no package or action added.
source: dec-mt5ihu7p «except: The test matrix installs its one package through a SHA-pinned apt cache action under its step timeout, and the Android job sets up its NDK through a SHA-pinned setup-ndk action.»
replace-with: no package or action added — except that the test matrix installs its one package through a SHA-pinned apt cache action under its step timeout, and the Android job sets up its NDK through a SHA-pinned setup-ndk action.
@leaf_edit n=2 leaf=.claude/docs/stack.md line=42 class=omission
leaf-text: the bash leg runner `ci-leg.sh`
source: dec-ivlt63uv «The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive for the rustc host and export SKIA_BINARIES_URL, and a refused archive»
replace-with: the bash leg runner `ci-leg.sh` (whose clippy, test, build, msrv, doc, coverage and a11y legs each first verify skia's prebuilt archive for the rustc host through `skia_archive.py` and export `SKIA_BINARIES_URL`, a refused archive ending the leg before any cargo runs)
