## Addendum 1 — resumed 2026-11-15

Written at the 2a resume of this wrap (run dir `.andromeda/runs/2026-11-15T07-30-02-wrap`), before P2's first fan-out. No line of the report above is edited; where this addendum and a line above differ, this addendum is the later fact.

### Inputs read at the resume
- I6 · `../additional/escher-overseer/relays/morning-answers-founder-2026-11-15.md` · copy no-repo — the founder's own answers of the morning of 2026-11-15, by question dialog in overseer1's session (results at 07:26Z and 07:29Z), each selection verbatim; his own words, not a delegate's — inputs#I6.
- I7 · message: the operator, at the /andromeda-wrap-session resume, 2026-11-15 · copy message — the resume's direction: resume at P2; act on I6's items 2, 3 and 4 as the founder's own words; item 1 waits for a 0-pending wrap; name the row that states rdme and blitz outside the mutation scope; an escalated operation reaches the operator only with its fit verdict; read-backs carry `--run-dir` — inputs#I7.

### The founder's rulings (supersede the PROVISIONAL dispositions above)
The report above carries Q1 and Q2 PROVISIONAL with no answer (inputs#I5). The founder is back and answered (inputs#I6, acted on by inputs#I7). Every disposition above that reads PROVISIONAL for Q1 or Q2 now reads as follows.

- **Q2 — ratified, both** (inputs#I6 §2, «Ратифицировать оба (Recommended)»). The take's two behaviour changes for every document land ratified, not PROVISIONAL:
  - `f5ea1caf1`: an `@font-face` rule's `font-stretch` (`font-width`) descriptor reaches font matching, as `FontFaceOverrides::width` (Changes → Symbols / APIs). Expected amendment 5 (architecture `decision/font-face-css-descriptors`, `dec-zn2z2ubp`, + width) lands ratified.
  - taffy 0.15.0 → 0.15.1 from the registry: its layout fixes for every document (Changes → Dependencies). Ratified.
  - The two are this take's engine behaviour changes for every document, ratified — the fact `decision/upstream-take-engine-changes` (`dec-z45xhlo5`) states for the previous take (`563d3db73`, twenty-nine commits). That row stays true of its take; this take's ratification is a new fact beside it.
- **Q1 — «Снять для Taffy»** (inputs#I6 §3). The plan applied option 1 (keep the rule, the registry carrier); the founder's answer is option 2, and:
  - **This chunk stays as planned:** registry `taffy` 0.15.1, no git source, no manifest moved (Changes → Dependencies unchanged). No source of this chunk changes on the answer.
  - **An exception for Taffy ALONE to the no-git-dependency rule**, written by this wrap as an amendment on his word, at every site the rule stands:
    - architecture `decision/no-git-dependencies` (`dec-ofaljlii`: "No git dependency is left in the manifest; taffy is a registry version.");
    - security-plan `decision/no-git-dependency` (`dec-jm3wczan`: "The workspace takes no git dependency.");
    - security-plan `decision/no-git-dependencies` (`dec-kxrtstbq`: "The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.");
    - CLAUDE.md's Critical Warning ("The workspace takes no git dependency — every crate a crates.io registry version") and `.claude/rules/security.md` §Dependencies ("the workspace takes no git dependency, the root manifest included") — leaves, the cascade's.
    The exception permits a git source for the `taffy` crate alone; every other crate stays a crates.io registry version, and today none is a git source (taffy resolves to registry 0.15.1). A git Taffy, when one is taken, still passes the lock, audit and MSRV legs.
  - **A further chunk** moves Taffy to upstream's git version with #1303 and the left-out chain (`b8f4ba6fd` … `b4952e15f` toward `a3997d55`, `1a11761bc`'s `inline.rs` line). That move is not measured (upstream's pin is 49 commits past v0.15.1's fork point): its phase measures it before it plans it. The entry is minted at this wrap's route-resolve, placed after «Browser corrective» (inputs#I7). Expected amendment 2 (`decision/deferred-upstream-dependency-bumps`, `dec-vu7h54n4`) therefore reads: the Taffy-main content arrives by registry 0.15.1 save #1303, and #1303 with the later git chain waits for that entry — no longer "a registry release or the founder's Q1".
  - The P5 card's Beyond-the-plan row 3 ("waits on Q1's answer or a registry release", owner «Upstream sync ahead of polish and ship") is answered: its owner becomes the minted git-Taffy entry. Row 4 ("applied PROVISIONAL now … discharged at the epoch boundary") is discharged now by the founder's own word.
- **`dec-kxrtstbq`'s evidence** (the operator's re-verify, inputs#I5) stands as measured above: its rule holds, its quoted checksum and range moved (`Cargo.lock:8042-8043` now hold the registry source and `checksum = "31f7bd2d97…"`). Its rule is amended for the Taffy exception (above) in the same operation.

### A new expected amendment, on the founder's word (inputs#I6 §4)
8. **rdme and `blitz` stated outside the mutation tier's scope** — «Вывести из охвата (Recommended)». `apps/readme` (package `rdme`) and `packages/blitz` (package `blitz`) hold no tests: the Epoch 4 code audit skipped both `baseline-test-failure` (nextest `no tests to run` on the unmutated tree; `blitz` 1 mutant, rdme 33 in-diff mutants), recorded in `.andromeda/runs/2026-11-14T19-15-33-code-audit/proposals.md` §Skips (sealed). Both are upstream's demo shells. The fact: the mutation tier does not run these two units; no test is owed them for it. Its home: architecture, beside `decision/mutation-tier-host-only` (`dec-q5vvocav`, area `stack-and-technologies`) — the code audit's scope letter reads its extra exclusions only from an architecture decision row ("A new exclude is a decision a wrap writes into the architecture", the audit's `collectors.md` §Source paths). The exclusion is the mutation tier's alone: churn, sizes and the other collectors keep both units.

### Not acted on here
- I6 §1 (the two route placements of `ba44c3559` confirmed) waits for a 0-pending wrap (inputs#I7): their PROVISIONAL marks stand through this wrap.
- I6 §5 (the disk, the desk queue's #140–#145) is overseer1's, at the seam after this wrap.
