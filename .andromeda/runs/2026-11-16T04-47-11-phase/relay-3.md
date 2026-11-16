The operator's answer, 2026-11-16, at phase P4 (question dialog), verbatim:

Q «Errors with the action: dioxus-core 0.7.10 wraps every app in a default root ErrorBoundary, so a handler `Err` lands there (never logged), and with no app boundary the whole screen becomes a red <pre> of the error. A session-provided ErrorContext would sit above it and see nothing. How should the driver report a handler `Err`?» → "Read the root boundary (Recommended)"
