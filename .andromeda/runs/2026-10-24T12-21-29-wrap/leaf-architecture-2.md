group: architecture-2
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:8   class: retired
  leaf-text: «The 1.90.0 pin sits below `rust-version` 1.91.0 despite its keep-in-sync comment.»
  source: .andromeda/architecture.md:21 «the `ci-scripts` leg needs Python 3.11 or later; mimalloc is the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner.»
  replace-with: «»

- leaf: .claude/docs/stack.md:8   class: stale
  leaf-text: «Nix flake dev shell: rust-bin stable 1.90.0 with rust-src»
  source: .andromeda/architecture.md:21 «the `ci-scripts` leg needs Python 3.11 or later; mimalloc is the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner.»
  replace-with: «Nix flake dev shell: rust-bin stable 1.91.0, in sync with `rust-version`, with rust-src»

- leaf: .claude/docs/stack.md:27   class: omission
  leaf-text: «| Parallelism & misc | rayon 1, slotmap, rustc-hash, bytes, smallvec, thin-vec, atomic_refcell, bitflags, bytemuck, pollster, smol_str, thread_local |»
  source: .andromeda/architecture.md:21 «mimalloc is the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner.»
  replace-with: «| Parallelism & misc | rayon 1, slotmap, rustc-hash, bytes, smallvec, thin-vec, atomic_refcell, bitflags, bytemuck, pollster, smol_str, thread_local; mimalloc, the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner |»

- leaf: .claude/docs/workflow.md:15   class: stale
  leaf-text: «upstream `main` was last read at `17de049b1`, 7 commits past it, of which three are taken by no-commit cherry-picks of upstream's text, the merge base unmoved: `fb8d6c15e` (the default cursor over no box, upstream #1220) alone (`275950dc0`), then `38ca4e777` (`:focus-within`, upstream #1226) and `17de049b1` (the WPT pin, upstream #1228) together (`79603d1ef`) — the four Taffy-main commits wait for the next sync, since they put a git Taffy in the root manifest, so Taffy stays the registry "0.15"; both partial takes ratified.»
  source: .andromeda/architecture.md:286 «upstream `main` last read at `f9cbb2756`, thirteen of its commits taken by no-commit cherry-picks with the merge base unmoved; git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for»
  replace-with: «upstream `main` was last read at `f9cbb2756`, thirteen of its commits taken by no-commit cherry-picks with the merge base unmoved; git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".»
