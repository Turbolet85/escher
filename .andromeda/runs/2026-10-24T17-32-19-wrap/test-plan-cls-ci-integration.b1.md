# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · yes · add-to · the stand-repeat cargo line now also runs ten named --test words after the stand_*.rs glob each round, and its leg-pin test asserts them (F1)
ci-integration#2.1 · no · the session changes no cache key or save rule in ci.yml, which it does not reach, and brings no CI timing
ci-integration#3.1 · no · the one ci-leg.sh change is ten --test words on the stand-repeat cargo line, adding no apt-install.sh call, so the claim that no leg of ci-leg.sh calls the script still holds, and the install steps sit in ci.yml, which the session does not reach
ci-integration#4.1 · yes · add-to · host_log's standing check at trace under --record joins the 2026-10-14 one-off reading of a client's own stderr (F10, F11)
ci-integration#5.1 · yes · add-to · the CI#40114977936 readings (green 17 of 17 on 18b50a2a20f3) are appended, not substituted: linux 206/1105, macos and coverage 187/1102, windows 187/1057, stand repeat 198/837 with 21 ignored, a11y unmoved at 6/43
ci-integration#6.1 · yes · add-to · the session's workspace re-count is appended, 206 result lines and 1105 passed from 1099 (stand_requirement_sweep +1, stand_lint +1, text_extent +3, host_log +1)
ci-integration#7.1 · no · the coverage leg's llvm-cov and floor-script lines (ci-leg.sh:40-47) are not the changed stand-repeat cargo line, and the floors are unmoved at 97.99, 85.57 and 94.15 %
