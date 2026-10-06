"""Delete raw tool output after summarization — ONLY paths inside this run's run_dir (the sanctioned delete scope)."""
import os, shutil, sys

RD = os.path.realpath(sys.argv[1])
assert RD.endswith("-code-audit") and "/.andromeda/runs/" in RD, RD
RAW = ["attempt1-mutants-dioxus-native-dom", "attempt2-mutants-dioxus-native-dom", "attempt3-mutants-dioxus-native-dom",
       "attempt4-mutants-dioxus-native-dom", "attempt5-mutants-dioxus-native-dom",
       "attempt1-mutants-dioxus-native-dom.log", "attempt2-mutants-dioxus-native-dom.log", "attempt3-mutants-dioxus-native-dom.log",
       "attempt4-mutants-dioxus-native-dom.log", "attempt5-mutants-dioxus-native-dom.log",
       "mutants-dioxus-native-dom", "mutants-dioxus-native-dom.log", "rca", "jscpd-abs", "tokei.json", "graph-raw.json",
       "dead-classed.json", "coverage-raw.log", "machete-raw.txt", "jscpd-stdout.txt", "rca-stdout.txt"]
for name in RAW:
    p = os.path.realpath(os.path.join(RD, name))
    assert os.path.commonpath([p, RD]) == RD and p != RD, p
    if os.path.isdir(p):
        shutil.rmtree(p)
    elif os.path.exists(p):
        os.remove(p)
print("\n".join(sorted(os.listdir(RD))))
