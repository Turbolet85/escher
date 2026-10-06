# blitz-shell

_Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts Paint and shell, §Infrastructure Patterns Shell loop, [Shell and windowing])._

## Responsibility
Event loop, windowing and system integration on winit: `BlitzApplication`, `View`, `WindowConfig`, the `BlitzShellProvider` implementation of `ShellProvider`, the AccessKit adapter wiring and a data-URI-only net provider. It is the windowed path — escher's headless driver does not go through it.

## Key integrations

### Consumes from
- winit `=0.31.0-beta.3`, accesskit + accesskit_xplat, arboard (clipboard), rfd (file dialog), blitz-dom, blitz-paint, anyrender window renderers.

### Publishes to
- `BlitzApplication`, `BlitzShellEvent` (Poll, ResumeReady, RequestRedraw, CloseWindow, Accessibility, Embedder, Navigate, NavigationLoad, ResizeSettleCheck), `BlitzShellProxy` (also a `NetWaker`), `View`, `WindowConfig`, `DataUriNetProvider`; re-exports winit `ControlFlow`, `EventLoop`, `EventLoopProxy`, `Window`.

## Internal conventions
- Shell events flow through an mpsc channel drained in `proxy_wake_up`; every window event is followed by a Poll.
- The accessibility tree is rebuilt on poll when the document changed, through `Document::accessibility_tree` (`AccessibilityState::update_tree(&dyn Document)`), so a wrapper's override reaches the platform tree; window focus and bounds are forwarded to the adapter on every window event.
- Default features: accessibility, clipboard, file-dialog.

## Crate-specific gotchas
- Windows are created invisible and shown after AccessKit initialises (AccessKit panics otherwise); the adapter must exist before first show.
- View's Drop suspends the renderer first (GPU surface outliving its window segfaults on Wayland); windows are dropped before exiting the loop (winit 4135).
- Shortcuts: Ctrl/Meta +/-/0 zoom; Alt+D layout, Alt+H hover highlight, Alt+T taffy tree.
- Event loop, window creation and `downcast_doc_mut` unwrap.

## Entry points for modification
- `src/{application,window,event,accessibility,convert_events,net,lib}.rs`

## Testing this crate
- No in-crate tests; headless behaviour is tested through `blitz-test-harness`.

## References
- `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
