# layouts · standing conventions

- S1 · a new stand-facing witness reaches stand controls only by their stable element ids, never by position or text · surface-desktop-native
- S2 · a new stand check boots the headless stand at its pinned viewport and reads a held instance as a fresh boot · surface-desktop-native
- S3 · a new served-page check drives the stand through the existing escher-session actions and answers, under its exit codes and refusals · surface-cli
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
  contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from the verb table and the task names, stdout empty, nothing booted and no state created.
  contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object with its fixed remedy on stdout and its fixed message on stderr.
  contract/escher-session-refusals/unknown-verb · escher-session refusals · unknown-verb — A first word the verb table does not hold is refused unknown-verb, exit 1.
