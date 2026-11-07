@answer n=1 entity=threat-model-summary#1.1 v=no
why: its remote-content entry points and trust boundary sit in blitz-net, blitz-html, blitz-dom and apps/browser, none of which the Dioxus render-path change reaches
@answer n=2 entity=threat-model-summary#2.1 v=no
why: its script-execution rules sit in blitz-vibey-script, the browser feature flag and the WPT runner, none of which the session reaches
@answer n=3 entity=threat-model-summary#3.1 v=no
why: its session-socket rules sit in escher-driver and the seven_guis session host, which the session does not touch
@answer n=4 entity=threat-model-summary#4.1 v=no
why: its MCP stdio rules sit in escher-driver's mcp.rs, which the session does not touch
@answer n=5 entity=threat-model-summary#5.1 v=no
why: its statement sits in docs/capabilities.md, which the session does not touch
