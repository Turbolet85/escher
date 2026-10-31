# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the overlay lives in the engine and reaches every renderer through blitz-paint's paint_scene, which paints over resolved styles and layout
design-philosophy#2.1 · yes · the list's redraw request reaches the shell only through the ShellProvider the embedder supplies, with DummyShellProvider as the no-op default
design-philosophy#3.1 · yes · the focus order (SC 2.1.1, 2.4.3), keyboard and pointer gates, accessibility state and overlay paint order the chunk builds must honour the cited specs
design-philosophy#4.1 · yes · open-state persistence over versioned NodeIds on DOM nodes only, cleared or retargeted on removal, and the damage and redraw path when a list opens or closes
design-philosophy#5.1 · yes · the harness's screenshot, screenshot_rect, text_contrast and settle stay headless and never move time, with synthesized input through the real dispatch pipeline
