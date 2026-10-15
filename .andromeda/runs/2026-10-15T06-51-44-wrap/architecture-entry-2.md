
## 2026-10-14-quality-gates — the test-only listener, the manifests, the merge surface read, the pinned driver claims
**Section:** §Occupied Resources → Network ports and listeners, Filesystem, Environment variables, Names · §Project Intent → Product type, Upstream sync · §Established Decisions → [Scrolling and input] · §Standard Contracts → Driver session
**Change:**
- Network ports and listeners: the session socket is the one served listener site. Beside it, one test-only bind: escher-driver's unix-only `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` binds a `UnixListener` in `eab-{pid}` under the system temp directory, never answers, removes it — no crossing. Filesystem gains that directory; Environment variables gain `TMPDIR`, through `std::env::temp_dir()`, test code only.
- Names and Product type: the workspace package's and debug_timer's homepage and repository read `https://github.com/Turbolet85/escher` (was `https://github.com/dioxuslabs/blitz`, kept as the as-adopted reading).
- [Scrolling and input]: the range model's and keyboard activation's merge surfaces read at upstream `e72650a6`, untouched since `434e0661`. Was "owned by a CARRY on the route entry Quality gates".
- Upstream sync: the merge base stays `434e0661`; upstream `main` read at `e72650a6`, three commits past it (#1198, #1212, #1211), not merged.
- Driver session: the 30 s per-read answer bound and "a write to a closed pipe is dropped, not a panic" each pinned by a check; `busy` read on a real not-settled `command` line. Was "covered by its unit test only".
**Why:** measured at this chunk. The test-only bind registered on the operator's answer at this wrap's escalation, «Register test-only»: never served, reached by no caller, so no crossing and not the founder's — registered as `PAINT_TREE_BENCH_HTML` and `CARGO_TARGET_TMPDIR` are.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
