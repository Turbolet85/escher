@answer n=1 entity=threat-model-summary#1.1 v=no
why: the session's change sits in the Dioxus render path of dioxus-native-dom, and this remote-content entity's cited paths (blitz-html, blitz-net, blitz-dom, apps/browser) are not among the reached files
@answer n=2 entity=threat-model-summary#2.1 v=no
why: its script-execution paths (blitz-vibey-script, apps/browser, the WPT runner) are not among the reached files, and the session adds no script surface
@answer n=3 entity=threat-model-summary#3.1 v=no
why: its session-socket paths (escher-driver, examples/seven_guis) are not among the reached files, and the wire and its trust boundary are unchanged
@answer n=4 entity=threat-model-summary#4.1 v=no
why: its MCP stdio paths (packages/escher-driver/src/mcp.rs) are not among the reached files, and what a result carries is unchanged
@answer n=5 entity=threat-model-summary#5.1 v=no
why: the committed capability statement it describes is not among the reached files, and the carried set it states is unchanged
