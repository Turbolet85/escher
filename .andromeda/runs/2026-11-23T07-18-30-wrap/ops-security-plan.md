@row_add n=1 row=decision/canvas-content-crosses-as-shot-pixels area=threat-model-summary fact=F1 detector=none
name: Canvas content crosses only as shot pixels
rule: A canvas's app-painted content, a widget painted from Rust, crosses only as pixels in a screenshot on every ratified carrier, the model provider on a cold-agent live run included, with no new carrier, field, cause, session error, exit, log field or span, and a kept check keeps no pixels.
why: A boundary widening, ratified.
guard: stand_canvas_shot
basis: fct-o5thr6xa (expected amendment, security-plan §Threat Model Summary, the founder's F1 A), fct-h53hgvcx, fct-xx3c5d24
because: The chunk paints a widget's pixels into the harness and driver shots with no verb, field, cause, exit, log field or span added, and the canvas crossing as pixels was ratified for every carrier, a decision no standing row states.
@row_add n=2 row=decision/harness-blitz-paint-custom-widget area=dependency-security fact=F2 detector=D-security-deps
name: Harness turns on blitz-paint's custom-widget feature
rule: blitz-test-harness turns on blitz-paint's custom-widget feature, so blitz-paint's widget paint pass is compiled into the harness, the driver and the escher-session binary in every build selection that reaches the harness, adding no package to Cargo.lock.
why: So a shot paints an app's widget in every build selection.
guard: stand_canvas_shot
basis: fct-xkqgbvsv, fct-wqnem4ql, fct-o5thr6xa (expected amendment, security-plan §Dependency Security)
because: The harness's feature set widened to compile the widget paint pass into every binary reaching it, a dependency-surface choice no standing row of the section states; peniko as a blitz-tests dev-dependency adds no package and stays within the workspace-inherited rule.
