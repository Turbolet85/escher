# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.0 · no · deny.toml and Cargo.lock are byte-identical with no ignore added or dropped, so the eight standing ignores and the audit invocation still hold
dependency-security#2.1 · no · Cargo.toml and Cargo.lock are byte-identical through this sync, so the 2026-10-10 record's edges and line citations still hold
dependency-security#2.2 · no · the root manifest and lock are byte-identical through this sync, so the 2026-10-13 record's taffy and workspace-version edges still hold
dependency-security#2.3 · no · no manifest or lock line moved in this sync, so the 2026-10-17 record's keyboard-types and test-that edges still hold
dependency-security#2.4 · no · Cargo.toml, Cargo.lock and deny.toml are byte-identical through this sync, so the 2026-10-18 record's no-edge take still holds
dependency-security#2.5 · no · the record states commits already taken on 2026-10-19 (17de049b1, the WPT pin, among them), which this sync cannot move, and its lock and manifest reads stand on the byte-identical files
dependency-security#2.6 · yes · add-to · F6, this sync's lock record, joins the list after this record; apps/browser/src/main.rs, a merge surface, also reaches the default global-allocator claim this record states
dependency-security#3.1 · no · Cargo.toml:165 and escher-driver are unchanged this session, so the serde_json root entry and its reader claims still hold
dependency-security#4.1 · no · the png entries sit in manifests the session left byte-identical and in escher-driver, which is unchanged, so the claim still holds
dependency-security#5.1 · no · blitz-test-harness's color entry (Cargo.toml:27) sits in a manifest the session left byte-identical, so the claim still holds
dependency-security#6.1 · no · deny.toml's [licenses] table and ci-leg.sh:38 are byte-identical and unreached, and licenses ok still holds on this sync
