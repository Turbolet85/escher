@row_set n=1 row=decision/skia-default-build-fetch cell=rule seen=1 fact=F1 detector=D-security-deps
value: Skia is built by default in the browser app and the WPT runner from a prebuilt archive for the locked skia-bindings tag, fetched once per platform and verified against a committed per-target sha256; a differing archive is refused, leaving no file, and the build reads only the verified local copy.
basis: fct-z3khe52c, fct-sods3b6s
because: The build-time fetch is no longer unchecked: the archive is verified against a committed sha256 record before the build reads it, so the row's «with no content check» is false.
@row_set n=2 row=decision/advisory-ignores-by-id cell=rule seen=2 fact=F4 detector=D-security-deps
value: An advisory is ignored only by its ID, each with a written reason stating its outcome, the upstream release it waits on or the residual row by its id that owns it, with no blanket allow and unmaintained and unsound at their defaults; no product file names the residuals file.
basis: fct-wpxa3yhl, fct-jccksell
because: The ignore reasons now name the release each waits on, only one naming its owning residual row, so a rule demanding every reason name a residual row is false.
@row_set n=3 row=decision/dependency-audit-cargo-deny cell=except seen=1 detector=D-security-deps
value: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate, a wait on a cacache release as cacache 13.1.0 uses no advise_range.
basis: fct-sods3b6s, fct-wmr7jejp
because: The memmap2 exception now carries its stated outcome, a wait on a cacache release, which the row's except leaves out.
@row_add n=1 row=decision/skia-archive-fallback-source-build area=dependency-security fact=F2 detector=D-security-deps
name: Skia archive fallback stated as a limit
rule: A failed read of skia's verified archive, or a build asking for an unrecorded key, falls back to a full source build of skia with no panic; the CI matrix witness step and the dev host's gate read that red.
why: A forced download cannot turn the fallback into a panic in a crates.io build, so the fallback is stated as a limit.
guard: the CI matrix step Skia built from the verified archive
basis: fct-pufyyo3s, fct-twf6j6ew
because: The chunk states the skia archive fallback as a limit read red by a witness, a decision no row holds.
@row_add n=2 row=decision/no-force-skia-binaries-download area=dependency-security fact=F3 detector=D-security-deps
name: No forced skia binaries download
rule: FORCE_SKIA_BINARIES_DOWNLOAD is set by no CI leg, step or build.
why: In a crates.io build it routes skia-bindings to a full source build instead of a forced download.
basis: fct-uvfujhlk, fct-twf6j6ew
because: The planned forced download was measured to give a source build and dropped, a deliberate choice no row states.
@row_add n=3 row=contract/environment-variables/skia-binaries-url area=secret-management fact=F5 detector=D-security-auth
boundary: Environment variables
item: SKIA_BINARIES_URL
promise: Holds no secret, a local file:// path template; set by the CI leg runner only in the skia-building legs and before the fast leg's sweep, overwriting any value a caller set, and by the CI matrix step; read by skia's build script.
basis: fct-z3khe52c, fct-vpyradka
because: The chunk adds an environment variable set by CI and read by a build script, and the environment-variable contract lists none for it.
@row_add n=4 row=decision/skia-archive-call-validation area=input-validation fact=F6 detector=D-security-input
name: Skia archive script validates its call first
rule: The skia archive script takes exactly a target and a directory, exits 2 with a fixed line on a bad call or an unrecorded target before any fetch, exits 1 with fixed attempt lines when no verified archive is obtained, and reads no environment variable.
basis: fct-z3khe52c, fct-scmtjoxy
because: The chunk adds a build-time script taking a call and an external archive, validated before any fetch, a boundary no row of Input Validation holds.
