
## 2026-10-12-headless-screenshot — the scopes, the tests and the stack: `image`, three test targets, `png`
**Section:** §Existing Scopes → escher-driver · blitz-test-harness · blitz-tests · §Conventions → Tests · §Occupied Resources → Process-wide state and threads · §Stack and Technologies → Testing · Media
**Change:**
- escher-driver: was thirteen private modules, now fourteen — `image` (the PNG encoder and the hand-written padded base64); execute holds `Outcome::Shot` and the crop, json the shot's line, mcp the image block; was four dependencies, now `png` the fifth.
- blitz-test-harness: gains the private module `screenshot`, its three types re-exported.
- blitz-tests: a thirteenth driver-action file, `stand_act_screenshot`; `stand_screenshot_repeat` (one state's shot byte-identical in two child runs and the parent, 12 shots); the engine check `password_painted_masked`. Was `session_common` read by eighteen; now twenty (+ `stand_act_screenshot`, `stand_screenshot_repeat`), holding the two `screenshot` builders and the `shot` reader.
- Conventions → Tests: was the twelve `stand_act_*` and `stand_mcp_tools`; now the thirteen and `stand_screenshot_repeat` too.
- Process-wide state: was four blitz-tests binaries re-executing their test binary; now five — `stand_screenshot_repeat` runs it twice, each child printing its shots for the parent's assertions only.
- Testing: blitz-tests' dev-dependencies gain `png`. Media: `png` (0.18.1) also taken by escher-driver and blitz-tests, no package entering `Cargo.lock`; no `base64` crate.
**Why:** the chunk's code: one more private module, one harness module, three new test targets and a dependency already in the graph.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/
