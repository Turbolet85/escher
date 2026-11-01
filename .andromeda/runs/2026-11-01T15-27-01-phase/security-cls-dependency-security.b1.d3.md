# Classified — security · dependency-security.b1

## Answers
dependency-security#1.0 · yes · the audit tool, the six-platform `[graph]` and the per-ID ignores with reasons are what the audit leg re-runs and what security.md §Dependencies re-reads at this take (Needs: audit gate)
dependency-security#2.1 · yes · a whole take's lock is read name by name: each entering crate is reviewed by hand (licence, repository, build script, proc-macro, dependent) and run through the licence check (Needs: any crate that enters; a lock read per shape)
dependency-security#2.2 · yes · Taffy's registry "0.15.0" pin with its eight features, the workspace version and `=` pins in the Cargo.toml merge surface, and the lock's name@version set, which must keep no git dependency
dependency-security#2.3 · yes · Taffy stays registry "0.15" with no git dependency, the lock's set equals upstream's, and the audit read with the three zbus-family ignores is the one the chunk re-reads
dependency-security#2.4 · yes · the cherry-pick onto registry pins the founder may choose: no package enters, leaves or moves, Cargo.toml, Cargo.lock and deny.toml unchanged, git Taffy `ea07c4c3` not taken
dependency-security#2.5 · yes · the second cherry-pick record: no package enters or moves, Taffy stays registry "0.15", upstream's Taffy-main commits not taken, licence and advisory reads unchanged
dependency-security#2.6 · yes · the pins Stylo "0.22.0", Taffy registry "0.15" and Dioxus "0.7.3", and the not-taken git Stylo, git Taffy and Dioxus 0.8.0-alpha.3 are the whole-merge options the chunk chooses among; tokio 1.54.1 for RUSTSEC-2026-0386; mimalloc enters as a hand-reviewed direct dependency
dependency-security#2.7 · yes · the pins Stylo "0.22.0", Taffy registry "0.15", Dioxus "0.7.3" and winit "=0.31.0-beta.3", which the chunk's winit beta.4 and pick move; no git dependency; audit reads ok
dependency-security#3.1 · yes · the root `serde_json = "1"` entry (Cargo.toml:165, a merge surface) carries no features, so no `preserve_order` may enter through a moved manifest line; the reader of untrusted stdin
dependency-security#4.1 · yes · no package enters `Cargo.lock` and no `base64` crate is taken, and the chunk adds no crate; the root manifest's png "0.18" pin is on the Cargo.toml merge surface
dependency-security#5.1 · yes · the root manifest's color "0.3" pin (merge surface), and the rule that blitz-test-harness takes no `tracing` and no `style` and blitz-tests gains no dependency, which a take's dependency lines must keep
dependency-security#6.1 · yes · deny.toml's `[licenses]` table (13 expressions, six-platform graph) is the licence check every entering crate runs through (Needs: licence check); the licences leg reads it on every push
dependency-security#6.2 · no · the free-disk action on the `Stand checks repeated` job is another job's unit in ci.yml; the chunk edits only the matrix_test free-disk step, so this is a re-check every change owes
dependency-security#6.3 · yes · the matrix_test free-disk step's `if:` naming ubuntu-24.04 and ubuntu-24.04-arm, with the android row on ubuntu-latest, is the unit the chunk fixes or removes
