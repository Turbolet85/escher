# The operator's answer at implement P1 (dialog, 2026-11-22)

Question: The plan exports FORCE_SKIA_BINARIES_DOWNLOAD, but in a crates.io build that flag makes skia-bindings take a git-hash branch that fails (no repository) and fall through to a FULL source build of skia, with no download and no panic. How should implement proceed?

Answer: "Drop FORCE, keep {key} (Recommended)"

Option text: Leg and matrix export only SKIA_BINARIES_URL, as a template file:///…/skia-binaries-{key}.tar.gz of the verified archive, so a build asking another key finds no file. The script still verifies the sum first; gate 5 witnesses the file read. Deviations recorded: FORCE dropped, test and control 2 re-aimed at the URL export. A key mismatch falls back to a source build instead of panicking, stated as a limit.

Notes: State the fallback as a limit in the plan's own words. If the witness that the build read the local file (gate 5's reading) can also run in the CI matrix step at no real cost, add it there so a silent source build on a non-Linux key reads red; if it cannot, say why in the report.
