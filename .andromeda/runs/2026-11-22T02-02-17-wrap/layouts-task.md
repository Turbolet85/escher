@task_prose part=session
The session paid dependency and CI debts. It changed no Rust source, no UI, no stand page and no element id.
It added a build-time step that fetches skia's prebuilt archive once per platform and checks its sha256 against a committed record. Every skia-building CI leg and the CI matrix's test rows now build skia from that verified local archive through SKIA_BINARIES_URL. FORCE_SKIA_BINARIES_DOWNLOAD was dropped and is set nowhere.
It added a matrix witness step that reads red when a row's skia build did not read the verified archive. After implement, that step was changed to read skia-bindings' build tree by content, because Cargo 1.100 moved the build output layout.
It moved the yanked async-executor 1.14.0 to 1.14.1 in the lock and nothing else. The stand's accessibility-adapter probe still reads 0 after the move.
It rewrote the advisory-ignore reasons as stated waits (the six IDs unchanged) and retired the cancelled-runner watch by its rule. Final CI was green, 17 of 17 checks.
@task_file n=1 path=.github/skia-archives.toml
what: added; the committed record of skia's prebuilt archives, holding the skia-binaries tag, the release URL template and each of five target triples' key and sha256
@task_file n=2 path=.github/scripts/skia_archive.py
what: added; the CI helper command that fetches and verifies one target's skia archive and prints the SKIA_BINARIES_URL template, exit 0, 1 or 2 with fixed stderr lines
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: added; the unit tests of the skia archive helper and of the committed archive record, run by the ci-scripts leg
@task_file n=4 path=.github/scripts/ci-leg.sh
what: edited; the CI leg runner script, which gains the SKIA_LEGS list of skia-building legs and runs each, with the fast leg's target sweep, under the verified skia archive
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: edited, again after implement; the unit pins of the CI workflow and the leg runner, now pinning the matrix's verified-archive and witness steps and the skia legs' exported URL
@task_file n=6 path=.github/workflows/ci.yml
what: edited, again after implement; the CI workflow, whose matrix test job gains the verified skia archive step and the witness step that reads skia-bindings' build tree by content
@task_file n=7 path=deny.toml
what: edited; the cargo-deny dependency audit policy, with only its advisory-ignore reasons reworded as stated waits and an owning residual row
@task_file n=8 path=Cargo.lock
what: edited; the workspace dependency lock, with only the async-executor entry moved from 1.14.0 to 1.14.1
@task_item n=1 part=touches
Any row of the cli surface stating how a CI or test script is called, its exit codes or its stdout. The session adds a new CI helper command with its own exit codes 0, 1 and 2, a one-line stdout and fixed stderr lines. This is a contract beside agent-run.sh and cold-agent.sh, and no row says it yet.
@task_item n=2 part=touches
Any row that says how skia reaches a build: of the headless stand, the escher-session driver, paint_bench's backend or multi_doc. On CI, skia now comes from a verified prebuilt archive, with a stated fallback to a full source build that the witness step reads red.
@task_item n=3 part=touches
decision/stand-a11y-by-attributes-alone and any stand rule that rests on the accessibility adapter being absent from the stand's dependency graph. After the async-executor lock move, that graph was read again and still holds no accesskit adapter, so the rule stays true.
@task_item n=4 part=touches
decision/session-roles-share-stand-boot and decision/held-instance-reads-as-fresh-boot, only through the async runtime that the stand and session boot on. Its async-executor dependency moved one patch version, with no change in behaviour.
@task_item n=5 part=touches
decision/cold-agent-pipe-builds-nothing and the cold-agent and agent-run CLI contracts, indirectly. Those scripts drive builds made on the same shared target directory, which CI now builds under SKIA_BINARIES_URL. A cargo run without that variable rebuilds skia-bindings.
@task_item n=6 part=touches
Any row naming an environment variable a command reads, beside MULTI_DOC_N, MULTI_DOC_SEQ and MULTI_DOC_FLAT. SKIA_BINARIES_URL is now set by the CI leg runner and the matrix step, and FORCE_SKIA_BINARIES_DOWNLOAD is not set anywhere.
@task_fact n=1 slug=surface-cli
fact: The skia archive helper is called with a target and a directory. It exits 0 and prints one line, the file URL template with a literal key placeholder. It exits 1 after three failed attempts, each in fetch or verify, ending with the fixed line no verified archive. It exits 2 on a bad call or an unrecorded target, before anything is fetched.
@task_item n=1 part=untouched
The stand stable element ids: no id was added, renamed or freed.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window, chrome or decoration behaviour changed.
@task_item n=3 part=untouched
The desktop stand's page decisions, which are the document skeleton, viewport, accent colour, list box, scroll and clipping, open list rows, the order and load pages, and lints at boot. No UI or layout code changed.
@task_item n=4 part=untouched
The escher-session MCP server, its verbs, refusals, recording, snapshot, screenshot and served tasks: no driver code changed.
@task_item n=5 part=untouched
The mobile-native and web-spa surfaces and the decisions log: nothing was measured or decided there.
@task_item n=6 part=untouched
The absence of arbitrary index.html templates: no document template capability was added.
