# layouts · standing conventions

- S1 · every new stand check addresses elements by the stand's stable ids as attributes alone, never by position or text · surface-desktop-native
- S2 · every new stand check runs headless at the pinned viewport and boots the stand lint-clean · surface-desktop-native
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
- S3 · every new session verb is listed in MCP tools/list, described in the help answer, and recordable as a replayable check file · surface-cli
- S4 · every new log line or span field carries no caller input and no answer content · surface-cli
