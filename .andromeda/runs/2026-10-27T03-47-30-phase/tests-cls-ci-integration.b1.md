# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · binds the ci-scripts leg the capability pin runs in, the fast and doc legs the done gate names (their components and doc's command), and the rule that a red leg ends with its status and no retry; the a11y, stand-repeat and coverage membership is not reached
ci-integration#2.1 · no · the cache is CI job state (Swatinem on the ci.yml jobs); the chunk touches no ci.yml job and its done gate runs local legs only (Not needed: caches)
ci-integration#3.1 · no · the package installs of the ci.yml jobs (apt-install.sh, the python3-yaml guard); ci.yml is not touched and no leg of ci-leg.sh calls the script (Not needed: package installs)
ci-integration#4.1 · yes · binds the screenshot reading the README's shot claim must state (byte-identical across processes and builds on one host, bytes differing across OSes; Needs), and the rule that a per-package green is not the workspace leg's green, the done gate's fast leg being the workspace build
ci-integration#5.1 · no · the per-OS tallies of the ci.yml job logs, which the chunk neither runs nor cites (Not needed: per-OS tallies; ci.yml is not touched; the chunk moves no count)
ci-integration#6.1 · yes · states the dev host's exit-0 readings of the done gate's legs (ci-leg.sh fmt, clippy and doc, doc with RUSTDOCFLAGS="-D warnings" and --locked, and the fast leg's workspace tests with their last count), which the chunk re-measures on this host; the cold and warm timings are not reached
ci-integration#7.1 · no · the coverage leg and its floor script: the done gate runs fast and doc, not coverage, and no crate source changes (Not needed: coverage floors)
