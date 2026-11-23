# security · standing conventions

- S1 · each new stand check runs in both layout modes, incremental and non-incremental · input-validation
- S2 · each new test builds and runs under locked builds with workspace-inherited dependency versions, in the unpublished test harness crate · dependency-security
  decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
- S3 · each kept evidence log holds no credentials and no temp or home path · secret-management
  decision/cold-agent-record-unprinted · Cold-agent record kept unprinted — The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
  decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
  decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
