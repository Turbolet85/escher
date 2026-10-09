# The answers to the two P4 forks — 2026-10-09-audit-corrections-agent-surfaces

Given through the session's question dialog at phase P4, 2026-10-09, in this window. The questions and option labels are this phase's wording; the answers and the note are verbatim.

## Fork 1

**Question:** Where should the bound and the retry of CI's package install live?

**Options offered:**
- One script (Recommended) — A new .github/scripts/apt-install.sh that every job calls. The bound and retry are stated once, and it is the only form provable on the dev host (a stand-in apt-get on PATH that fails or stalls, the way ci-leg.sh is already tested). Each ci.yml step stays one line, so no cited line moves. Cost: one new file and its tests.
- Loop in each step — No new file: a shell loop plus timeout-minutes written into each of the eight steps. Smallest concept, but eight copies of the numbers, about 56 added lines that shift every ci.yml citation below line 46, and nothing provable locally beyond the YAML's shape — the retry is first exercised when a mirror next stalls.
- Third-party retry action — A SHA-pinned retry action wrapping the command in each step. No script of ours to maintain, but it brings new third-party code into every linux job (security-plan gains a pinned action; the wrap may judge it a boundary widening and hold it PROVISIONAL), and it cannot be proven locally.

**Answer:** One script (Recommended)

## Fork 2

**Question:** How far should the bound reach? The ruling names the libfontconfig1-dev step (8 places); ci.yml has two more install sites it does not name.

**Options offered:**
- All ten sites (Recommended) — The eight named steps and the conditional python3-yaml install (ci.yml:191, in the job that runs the invariants) get the bound and retry; the matrix job's android install (ci.yml:377-382, a pinned apt-cache action) gets a step timeout only — an action step cannot be looped without replacing it. Leaves no unbounded install in the file; the android leg has no local witness, only CI.
- Eight plus the conditional — Every run step that calls apt-get is bounded and retried (the eight and ci.yml:191). The matrix job's action step is left as it is and recorded as a finding for the wrap to give an owner. Keeps the android leg untouched; one unbounded install site remains, never observed hanging.
- The eight only — Exactly the ruling's words. The conditional python3-yaml install and the matrix action step are both recorded as findings for the wrap. Smallest change; two install sites of the same failure class stay unbounded, one of them in the job that would detect a regression.

**Answer:** All ten sites (Recommended)

## Note given with the answers (verbatim)

The purpose of the ruling is that no CI job hangs on a package install; the two unnamed sites are the same failure class. For the android action step a step timeout alone is enough; say in the plan that it has only CI as its witness.
