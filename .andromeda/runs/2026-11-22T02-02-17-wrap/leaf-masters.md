@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=107 class=omission
leaf-text: build every crate (Linux needs `libfontconfig1-dev`; Arch: `fontconfig`)
source: dec-d4sn2vhm «decision/cargo-under-verified-skia-url · Every skia-building cargo call under the verified URL — Every cargo call in a skia-building leg, the fast leg's closing target sweep included, runs under the verified SKIA_BINARIES_URL.»
replace-with: build every crate (Linux needs `libfontconfig1-dev`; Arch: `fontconfig`) — a cargo run on the shared target directory without the legs' verified `SKIA_BINARIES_URL` re-runs skia-bindings' build script, so every cargo call of a skia-building leg, the fast leg's closing sweep included, runs under it
@leaf_edit n=2 leaf=CLAUDE.md line=110 class=omission
leaf-text: log in `target/ci-logs/{leg}.log`;
source: dec-ivlt63uv «decision/skia-legs-verify-archive-first · Skia-building legs verify skia's archive first — The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive»
replace-with: log in `target/ci-logs/{leg}.log`; the clippy, test, build, msrv, doc, coverage and a11y legs each first verify skia's prebuilt archive for the rustc host (`.github/scripts/skia_archive.py`) and export `SKIA_BINARIES_URL`, a refused archive ending the leg before any cargo runs;
