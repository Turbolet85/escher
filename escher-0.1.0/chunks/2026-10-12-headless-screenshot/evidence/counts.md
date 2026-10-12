# Counts — 2026-10-12-headless-screenshot

Read from /implement's final full gate firing (run dir `.andromeda/runs/2026-10-12T18-43-51-implement/`), all 18
non-operator entries green.

## The stand run (entry 15, `bash scripts/agent-run.sh run stand`)

- `run.start` files: **36** (34 before this chunk + `stand_act_screenshot` + `stand_screenshot_repeat`, as predicted)
- `run.end`: passed **139**, failed 0, ignored 7, cargo_exit 0, outcome `passed`
- Against the standing 131 over 34 files: +8 = `stand_act_screenshot` 6, `stand_screenshot_repeat` 1,
  `stand_mcp_tools` 1 (`a_screenshot_tool_call_answers_its_image_then_its_text_and_parents_its_command`).

## The workspace tests (entry 17, `bash .github/scripts/ci-leg.sh fast`, read from `target/ci-logs/test.log`)

- 174 `test result:` lines · **849 passed · 0 failed · 12 ignored**

## Screenshot sizes against the 1,048,576-byte answer bound

Measured through the built `escher-session` binary (one hosted session per task, `screenshot` with no id, the
answer's `png` decoded):

| task | size | PNG bytes | base64 bytes | answer line bytes |
|---|---|---|---|---|
| counter | 800 × 600 | 7,989 | 10,652 | 10,688 |
| flight-booker | 800 × 600 | 16,775 | 22,368 | 22,404 |
| timer | 800 × 600 | 11,431 | 15,244 | 15,280 |
| crud | 800 × 600 | 15,661 | 20,884 | 20,920 |

The largest answer line is flight-booker's, 22,404 bytes: 2.1 % of the bound. No stand screen comes near
`oversize` / `AnswerTooLarge`.
