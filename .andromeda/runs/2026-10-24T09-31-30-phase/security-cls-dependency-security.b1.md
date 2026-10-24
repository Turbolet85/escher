# Classified — security · dependency-security.b1

## Answers
dependency-security#1.0 · yes · the cargo-deny audit config (per-ID ignores, unsound default, six-target graph) that the chunk fixes the tokio advisory under and re-reads against the new lock
dependency-security#2.1 · yes · the take rule for a whole upstream set (each new name reviewed by hand, audit green, no ignore added or dropped) that the chunk's name-by-name lock read repeats
dependency-security#2.2 · yes · the pinning rule's registry Taffy, which this take moved from git to crates.io, and the name-by-name lock read against upstream that the chunk repeats
dependency-security#2.3 · yes · the registry Taffy pin, the in-repo = version pins and the lock-set equality against upstream that the chunk's lock read repeats
dependency-security#2.4 · yes · the cherry-pick take that moves no dependency and refuses upstream's git Taffy pins, one of the two shapes the chunk's P4 fork chooses between
dependency-security#2.5 · yes · the cherry-pick take that moves no dependency and refuses upstream's git Taffy pins again, a shape the chunk's P4 fork may repeat
dependency-security#3.1 · no · the chunk changes no escher-driver or harness dependency entry (its Not needed names the serde_json, png and color entries), so this serde_json entry and its reader rules are not reached
dependency-security#4.1 · no · the chunk changes no escher-driver or harness dependency entry (its Not needed names the png entries), so png's encoder and dev-dependency entries are not reached
dependency-security#5.1 · no · the chunk changes no escher-driver or harness dependency entry (its Not needed names the color entry), so blitz-test-harness's color entry is not reached
dependency-security#6.1 · yes · the [licenses] table in deny.toml that gates every push, which the chunk's mimalloc crate must pass by hand review and the licence check
