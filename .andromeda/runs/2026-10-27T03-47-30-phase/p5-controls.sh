#!/usr/bin/env bash
# P5 controls for 2026-10-27-readme-and-records: each entry fired through the gate tool over a scratch clone,
# first unmodified, then with its control input. Run from the project root.
set -u
root=$(git rev-parse --show-toplevel)
chunk=escher-0.3.0/chunks/2026-10-27-readme-and-records
plan="$root/$chunk/plan.md"
gate=~/.claude/skills/andromeda-tools/scripts/gate.py
d=$(mktemp -d)
trap 'rm -rf "$d"' EXIT
c="$d/c"
git clone -q --shared "$root" "$c"
echo "clone HEAD $(git -C "$c" rev-parse --short=12 HEAD)"

reset() {
    git -C "$c" checkout -q -- .
    git -C "$c" clean -fdq
    mkdir -p "$c/escher-0.3.0/chunks"
    cp -r "$root/$chunk" "$c/escher-0.3.0/chunks/"
    cp "$c/.git/info/exclude.orig" "$c/.git/info/exclude"
}
cp "$c/.git/info/exclude" "$c/.git/info/exclude.orig"

fire() {
    # $1 entry, $2 label
    line=$(python -X utf8 $gate run --plan "$plan" --entry "$1" --project-root "$c" | grep -E "^ *$1 ")
    echo "$2 · $line"
}

edit() {
    python3 - "$c" "$1" <<'PY'
import sys, pathlib
c, what = pathlib.Path(sys.argv[1]), sys.argv[2]
readme, stmt = c / "README.md", c / "docs/capabilities.md"
if what == "status":
    t = readme.read_text(encoding="utf-8")
    t = t.replace("### 0.2.0 — planned, in five stages", "### 0.2.0 — done and verified")
    t = t.replace("## License", "### 0.3.0 — planned, in six stages\n\n## License")
    readme.write_text(t, encoding="utf-8")
elif what == "pin":
    t = readme.read_text(encoding="utf-8").replace("twelve driver commands as tools", "twelve commands")
    readme.write_text(t, encoding="utf-8")
elif what == "lower":
    readme.write_text(readme.read_text(encoding="utf-8") + "\nBuilt by escher, a fork.\n", encoding="utf-8")
elif what == "stale":
    keep = [l for l in stmt.read_text(encoding="utf-8").split("\n")
            if not any(p in l for p in ("eleven tools, 43", "not been re-fired", "sink is unmeasured"))]
    stmt.write_text("\n".join(keep), encoding="utf-8")
elif what == "run":
    stmt.write_text(stmt.read_text(encoding="utf-8") + "\nThe measured instance is the 2026-10-25 cold-agent live run.\n", encoding="utf-8")
elif what == "home":
    stmt.write_text(stmt.read_text(encoding="utf-8") + "\nsee /home/<user>/x\n", encoding="utf-8")
elif what == "unignore":
    g = c / ".gitignore"
    g.write_text("\n".join(l for l in g.read_text(encoding="utf-8").split("\n") if l != "escher-0.3.0-incubator/"), encoding="utf-8")
elif what == "exclude":
    x = c / ".git/info/exclude"
    x.write_text(x.read_text(encoding="utf-8") + "escher-0.3.0-incubator/\n", encoding="utf-8")
elif what == "records":
    r = c / "escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence"
    r.mkdir(parents=True, exist_ok=True)
    (r / "records.md").write_text("exactly_the_chosen_flight_type_reads_selected\na_flight_type_choice_returns_the_select_value_and_both_options_selected_reading\nthe_stand_at_boot_reads_its_measured_findings\n", encoding="utf-8")
elif what == "source":
    p = c / "packages/blitz-dom/src/lib.rs"
    p.write_text(p.read_text(encoding="utf-8") + "\n// control\n", encoding="utf-8")
elif what == "stray":
    (c / "packages/stray_control.rs").write_text("// control\n", encoding="utf-8")
PY
}

reset
for n in 1 2 3 4 5 6 7 8 9 10 11; do fire $n "unmodified"; done

reset; edit pin;      fire 1  "must-fail (README pin phrase removed)"
reset; edit status;   fire 2  "must-pass (three status headings)"
reset; edit status;   fire 3  "must-pass (0.2.0 planned heading replaced)"
reset; edit lower;    fire 4  "must-fail (a lowercase escher in prose)"
reset; edit stale;    fire 5  "must-pass (the three stale lines removed)"
reset; edit run;      fire 6  "must-pass (the 2026-10-25 run sentence)"
reset; edit home;     fire 7  "must-fail (a /home/ path in the statement)"
reset; edit unignore; fire 8  "must-fail (.gitignore:39 removed)"
reset; edit exclude;  fire 9  "must-fail (an incubator line in the exclude)"
reset; edit records;  fire 10 "must-pass (records.md naming the three names)"
reset; edit status;   fire 11 "must-pass (README edited, allowed)"
reset; edit source;   fire 11 "must-fail (a tracked source file edited)"
reset; edit stray;    fire 11 "must-fail (an untracked file minted under packages)"
