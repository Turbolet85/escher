# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · yes · stale · stand-repeat's named targets move from fifteen to seventeen (ci-leg.sh:59-60, test_ci_workflows.py:367), stand_*.rs from 50 to 51 files, and the a11y leg's 112 tests to 114 (stand_keyboard 21 to 23)
ci-integration#2.1 · yes · stale · stand-repeat's "fifteen" named --test words is now seventeen (ci-leg.sh:59-60), while the claim that pseudo_element_flow_classification is none of them still holds
ci-integration#3.1 · no · the session moves no cache, job or key, and ci.yml is unreached with its caches unedited
ci-integration#4.1 · yes · add-to · the record of red runs gains F8 (CI#41233234207 red on Test (macos) alone over blitz-paint/font-embolden, which no local gate builds, with select_popup_painted as the local witness) and F9 (cargo stops at the first failing target, so a red job's later targets go unread)
ci-integration#5.1 · no · the free-disk step at ci.yml:311-316 lies in ci.yml, which the session does not reach or change
ci-integration#6.1 · no · the free-disk step's cure record concerns ci.yml and CI#41192433140, neither moved by the session
ci-integration#7.1 · no · the matrix free-disk step at ci.yml:392-394 lies in ci.yml, unreached, and no matrix row changes
ci-integration#8.1 · no · ci-leg.sh's change adds two --test words at 59-60 and no apt-install call, so "no leg of ci-leg.sh calls the script" still holds on the changed file, and apt-install.sh and ci.yml are unedited
ci-integration#9.1 · yes · stale · host_log.rs citations 45-135 and 137-280 shift under the 17-20 insertion and document.rs:2846-2873 under the earlier document.rs insertions, while the engine-feature claims stand
ci-integration#10.1 · yes · stale · the latest per-OS tally reading moves from CI#41192433140 to CI#41235503227 (linux 217 · 1236, a11y 14 · 114, stand repeat 228 · 1155), changing ci-integration:75
