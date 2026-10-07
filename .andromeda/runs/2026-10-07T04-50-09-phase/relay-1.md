# Relay 1 — the P4 fork on what `attach` crosses, and the answer

Asked by /andromeda-phase P4 on 2026-10-07, chunk 2026-10-07-driver-session, as one question round (two questions).

## Question 1 (verbatim)
What should `attach` cross in this chunk? (Playbook pattern "Boundary widening", verdict: escalate — the project has no listener, socket or pipe today, so this is your word, not my lean.)

Options as written:
1. **Unix socket now (Recommended)** — A std-only Unix-domain socket in an owner-only session state directory; only lifecycle messages cross (hello, stop) — no ids, names or values. Why: v010-12 needs separate CLI invocations to reach one held instance, so a local endpoint is unavoidable; building it here keeps infrastructure ahead of the CLI and adds no dependency. Cost: arch's "listeners: none" becomes one local socket, Windows gets a typed "unsupported" in 0.1.0, and it is a boundary widening to ratify.
2. **Stdio child only** — The session host is a child of whoever starts it, driven over its stdin/stdout pipes — the one crossing shape already ratified (the cold-agent stub). `attach` is an in-process handle to that child. Trade-off: no listener and it works on Windows, but separate CLI invocations cannot share a session, so the Driver CLI entry reopens this question and may rework the lifecycle.
3. **Library only** — An in-process `Session` type that holds the instance; no process and no crossing in this chunk. Trade-off: zero boundary change and the smallest chunk, but the entry's "one process" and `attach` are not built — a later entry (Driver CLI) carries the process, the endpoint and this same question.

## Question 2 (verbatim)
Whose word is this answer, for the record? A boundary widening is ratified only by the founder's own word; a delegate's answer is recorded PROVISIONAL until the epoch boundary.

Options as written:
1. **Founder's own word** — You are the founder answering here; the answer is recorded as ratified (founder, 2026-10-07).
2. **Delegate (provisional)** — You are answering in the founder's stead; the work proceeds on it and it is recorded PROVISIONAL, superseded by the founder's word at the Epoch 4 boundary.

## The answer (verbatim, as returned to the session)
- Question 1: "Unix socket now (Recommended)"
- Question 2: "Founder's own word"
- Note attached to the answer: "The founder chose this himself this morning (2026-10-07) in the overseer session, from the three options as you wrote them.."
