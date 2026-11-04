@answer n=1 entity=log-coverage#1.1 v=no
why: the closed-span line format lives in escher-telemetry and the span emitters, none of which the session changed
@answer n=2 entity=log-coverage#2.1 v=no
why: the verdict fields are cold-agent script fields, and the session touches no script or verdict
@answer n=3 entity=log-coverage#3.1 v=no
why: span_counts reads the cold-agent server log, which the session does not change
@answer n=4 entity=log-coverage#3.2 v=no
why: the live server recording directory is untouched, the session adds no recording
@answer n=5 entity=log-coverage#3.3 v=no
why: tasks_met reads cold-agent snapshot answers, code the session did not change
@answer n=6 entity=log-coverage#4.1 v=no
why: the apps/browser log sites sit in files the session did not touch
@answer n=7 entity=log-coverage#5.1 v=no
why: the examples output sites and the escher-session lines are unchanged, and no examples/ file moved
@answer n=8 entity=log-coverage#6.1 v=no
why: escher-driver's command and tool_call spans and their fields are unchanged, the driver is not in the change
@answer n=9 entity=log-coverage#7.1 v=no
why: docs/capabilities.md is not in the change, and the span set and scrub limits it restates are unchanged
@answer n=10 entity=log-coverage#8.1 v=no
why: the blitz-dom log sites sit in engine files the session did not touch, and no packages/ file changed
@answer n=11 entity=log-coverage#8.2 v=no
why: the blitz-dom events log sites sit in engine files the session did not touch
@answer n=12 entity=log-coverage#8.3 v=no
why: the blitz-dom layout log sites sit in engine files the session did not touch
@answer n=13 entity=log-coverage#8.4 v=no
why: the blitz-dom node log sites sit in engine files the session did not touch
@answer n=14 entity=log-coverage#8.5 v=no
why: the blitz-net log sites sit in a crate the session did not touch
@answer n=15 entity=log-coverage#8.6 v=no
why: the blitz-html parse-error log site sits in a crate the session did not touch
@answer n=16 entity=log-coverage#8.7 v=no
why: the blitz-shell and blitz-paint log sites sit in crates the session did not touch
@answer n=17 entity=log-coverage#8.8 v=no
why: the blitz and blitz-vibey-script log sites sit in crates the session did not touch
@answer n=18 entity=log-coverage#8.9 v=no
why: mutation_writer.rs is not in the change (read, not edited), so its trace sites and citations stay unmoved
@answer n=19 entity=log-coverage#8.10 v=no
why: the new probe target prints nothing, and the print sites this entity lists are in other test files the session did not change
@answer n=20 entity=log-coverage#8.11 v=no
why: the wpt runner is untouched and no engine source changed
@answer n=21 entity=log-coverage#8.12 v=no
why: publish-browser.yml and no other CI file is in the change
@answer n=22 entity=log-coverage#9.1 v=no
why: the console-call search covers the s02 files, which the session did not touch
@answer n=23 entity=log-coverage#9.2 v=no
why: the structured-logging search covers the examples slice files, and the probe lies outside that slice
@answer n=24 entity=log-coverage#9.3 v=no
why: its census over the 0.2.0 test additions still reads 0 on unchanged files, and the probe is not one of them
@answer n=25 entity=log-coverage#9.4 v=yes mode=add-to
why: F1 — the session adds the silent blitz-tests target dioxus_runtime_probe, which joins this list of test targets with no subscriber, env read, print or listener (silence census 0)
