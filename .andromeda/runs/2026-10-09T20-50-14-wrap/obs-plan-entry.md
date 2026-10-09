
## 2026-10-09-audit-corrections-agent-surfaces — the package install's output stays in the step log
**Section:** §9 CI Integration → Telemetry artifact handling
**Change:** a new sentence under the artifact table: the package install's output stays in the job's step log — ci.yml's install steps call `.github/scripts/apt-install.sh`, which no `ci-leg.sh` leg runs, so no `target/ci-logs/` file holds its output and no artifact uploads it; the script's own diagnostic is one stderr line per failed attempt (the attempt number, the phase, the exit status), nothing of a user's. No row of the table changed: failure logs still come from `target/ci-logs/` alone and the coverage report is still the one fork-CI artifact outside it.
**Why:** the chunk put a script with its own output into nine CI jobs, and §9 is where a reader asks whether that output is kept. The detector found every claim standing; the sentence was raised from the plan's reviewed list, which asked for the fact to be recorded.
**Kept:** no upload step or upload path was added to keep the install's output — a new artifact out of fork CI is a boundary widening that waits for the founder.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/
