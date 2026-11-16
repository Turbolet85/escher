# layouts · standing conventions

- S1 · a new or changed stand check boots the stand the way every stand check does: shared stand boot, pinned headless viewport, lints clean at boot, controls reached by their stable element ids · surface-desktop-native
  decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD and the load page read zero visual-lint findings, in both layout modes. · guard: stand_lint
- S2 · a new or changed check of the session's answer reads it through the escher-session command line or MCP surface as the answer contracts state it, and is replayable as a session recording · surface-cli
  contract/escher-session-record/check-file · escher-session recording · check-{n}.json — Written 0600, never replacing a file, in a 0700 directory whose parent must exist, removed by escher only when its recording fails; a shot kept without its png.
  contract/escher-session-record/record-dir-not-private · escher-session recording · record-dir-not-private — A recording directory open to its group or others is this session error, exit 3, start spawning nothing.
  contract/escher-session-record/recording-failed · escher-session recording · recording-failed — A recording that ends answers this session error, exit 3; over MCP isError.
