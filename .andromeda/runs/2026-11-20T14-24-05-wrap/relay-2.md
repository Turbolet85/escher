The operator's answer to the wrap's question (P5, 2026-11-20) «How should escher-0.5.0-incubator/ be kept out of git?» — options offered: You run the exclude (Recommended) · I append it by shell · Commit the incubator · Gitignore at a later wrap.

His answer, verbatim:
None of the four: add the line escher-0.5.0-incubator/ to the tracked .gitignore, under the 0.4.0 incubator's line at :41, as an operator-directed edit in this wrap's commit, recorded in the run dir — the form the three earlier incubators have. Leave .git/ alone, verify with git check-ignore, then continue.
