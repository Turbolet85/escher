@amendment marker=2026-11-23-canvas-placement-and-text
title: cookie 0.18.3, a reviewed pin
section: Dependency Security
change: No row changed: the reviewed pin is recorded here, the lockfile holding the fact.
  - RUSTSEC-2026-0594 («Private cookie nonces can repeat in processes that fork») on cookie 0.18.2, reached through cookie_store 0.22.1 ← reqwest 0.13.5 ← blitz-net, is fixed by a lock-only move to the patched cookie 0.18.3: its version and checksum lines alone, no manifest line, no deny.toml ignore; the audit reads advisories ok, licenses ok.
  - The syn edges of bindgen 0.73.2 and cssparser-macros 0.7.1 stay at 3.0.6 by edit, as cargo update -p cookie would move them to 2.0.119: the second instance of residual res-evkq636g, no new residual.
  - blitz-tests takes the workspace's one registry parley as a dev-dependency: one lock line, no package.
why: Recorded as a reviewed pin on the operator's wrap directive, delegate the operator, 2026-11-23 (the founder away); a master row of it read as an inventory the lockfile holds, so none was landed. The syn edges kept at 3.0.6 rest on the operator's word at the P4 stop.
ref: .andromeda/runs/2026-11-23T12-15-50-wrap/
