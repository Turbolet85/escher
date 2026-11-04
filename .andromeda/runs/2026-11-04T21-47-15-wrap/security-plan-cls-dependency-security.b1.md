@answer n=1 entity=dependency-security#1.0 v=yes mode=add-to
why: the session adds RUSTSEC-2026-0429 to the zip group at deny.toml:26 (F2), the local reproduction ci-leg.sh audit is the audit record's audit leg (F1), and pulldown-cmark moves 0.13.5 to 0.13.6 with the deny.toml citations below line 25 shifting
@answer n=2 entity=dependency-security#2.1 v=no
why: the 10-10 sync's six-name entry and its advisories reading concern that sync's own lock diff, which the session's pulldown-cmark-only bump does not reach
@answer n=3 entity=dependency-security#2.2 v=yes mode=stale
why: the session moves pulldown-cmark to 0.13.6 in Cargo.lock, and the lock's name@version set equality with upstream at 434e0661 cannot be shown to still hold on the changed file
@answer n=4 entity=dependency-security#2.3 v=yes mode=stale
why: the session moves pulldown-cmark to 0.13.6 in Cargo.lock, and the lock's set equality with upstream at the pin cannot be shown to still hold on the changed file
@answer n=5 entity=dependency-security#2.4 v=no
why: the claim is the 10-18 cherry-pick's own diff, which the session's later pulldown-cmark-only bump does not change
@answer n=6 entity=dependency-security#2.5 v=no
why: the claim is the 10-19 cherry-pick's own diff, which the session's later pulldown-cmark-only bump does not change
@answer n=7 entity=dependency-security#2.6 v=no
why: the claim is the 10-24 sync's own lock diff, and the session's bump enters and leaves no package, so the 987-package count holds
@answer n=8 entity=dependency-security#2.7 v=no
why: the claim is the 10-27 sync's own commit diff, which the session's pulldown-cmark-only lock bump does not reach
@answer n=9 entity=dependency-security#2.8 v=no
why: the claim is the 11-01 sync's own commit diff, which the session's pulldown-cmark-only lock bump does not reach
@answer n=10 entity=dependency-security#3.1 v=no
why: the session's pulldown-cmark bump is an in-place lock version change with no coupled pin, so the escher-driver serde_json row at Cargo.lock:3099 and serde_json v1.0.151 stand
@answer n=11 entity=dependency-security#4.1 v=no
why: the session adds no package to Cargo.lock, so the png dependency rows and the absence of a base64 crate stand
@answer n=12 entity=dependency-security#5.1 v=no
why: the claim is the 10-13 chunk's own dependency-line diff, which the session's pulldown-cmark bump does not touch
@answer n=13 entity=dependency-security#6.1 v=yes mode=stale
why: the new deny.toml line at 26 shifts the [licenses] table below it, so the cited deny.toml:32-51 and deny.toml:47 move by one line while the table's content holds
@answer n=14 entity=dependency-security#6.2 v=no
why: its rule lives in .github/workflows/ci.yml:311-316, which the session does not reach
@answer n=15 entity=dependency-security#6.3 v=no
why: its rule lives in .github/workflows/ci.yml and test_ci_workflows.py, which the session does not reach
@answer n=16 entity=dependency-security#6.4 v=no
why: its rule lives in .github/scripts/ci-leg.sh, which the session does not reach, and the embolden leg's --locked command still holds on the bumped lock
