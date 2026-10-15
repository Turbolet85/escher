#!/usr/bin/env python3
"""Check per-crate line coverage in an lcov file against a floor.

Usage: coverage_floor.py LCOV CRATE_DIR=PERCENT [CRATE_DIR=PERCENT ...]

Sums the `LF` (lines found) and `LH` (lines hit) of every `SF` record whose
path, relative to the working directory, lies under a crate directory, and
prints one line per crate then a verdict line. Exits 0 when every crate is at
or above its floor, 1 when one is under, 2 on a malformed call or a crate the
lcov holds no record for (a floor over nothing is never a pass).
"""

import os
import re
import sys

FLOOR = re.compile(r"^([A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*)=(\d{1,3}(?:\.\d+)?)$")


def usage():
    print("usage: coverage_floor.py LCOV CRATE_DIR=PERCENT [CRATE_DIR=PERCENT ...]", file=sys.stderr)
    return 2


def parse_floors(args):
    """Map each crate directory to its floor, or None when an argument is malformed."""
    floors = {}
    for arg in args:
        match = FLOOR.match(arg)
        if not match:
            return None
        crate, percent = match.group(1), float(match.group(2))
        if percent > 100 or crate in floors:
            return None
        floors[crate] = percent
    return floors


def records(text):
    """Yield (path, lines found, lines hit) for each `SF` record."""
    path, found, hit = None, 0, 0
    for line in text.splitlines():
        if line.startswith("SF:"):
            path, found, hit = line[3:], 0, 0
        elif line.startswith("LF:"):
            found = int(line[3:])
        elif line.startswith("LH:"):
            hit = int(line[3:])
        elif line == "end_of_record" and path is not None:
            yield path, found, hit
            path = None


def totals(text, crates, cwd):
    """Sum lines found and hit per crate directory; a crate with no record is absent."""
    sums = {}
    for path, found, hit in records(text):
        relative = os.path.relpath(os.path.realpath(path), cwd).replace(os.sep, "/")
        for crate in crates:
            if relative.startswith(crate.rstrip("/") + "/"):
                lines, covered = sums.get(crate, (0, 0))
                sums[crate] = (lines + found, covered + hit)
    return sums


def percent(covered, lines):
    return 100.0 * covered / lines if lines else 0.0


def main(argv):
    if len(argv) < 2:
        return usage()
    floors = parse_floors(argv[1:])
    if not floors:
        return usage()
    try:
        with open(argv[0], encoding="utf-8") as file:
            text = file.read()
    except OSError:
        print("coverage floor: cannot read the lcov file", file=sys.stderr)
        return 2

    sums = totals(text, floors, os.getcwd())
    missing = [crate for crate in floors if crate not in sums]
    if missing:
        for crate in missing:
            print(f"coverage floor: no lcov record under {crate}", file=sys.stderr)
        return 2

    under = 0
    for crate, floor in floors.items():
        lines, covered = sums[crate]
        reading = percent(covered, lines)
        print(f"{crate} {covered}/{lines} = {reading:.2f} % (floor {floor:g} %)")
        if reading < floor:
            under += 1
    if under:
        print(f"coverage floor: {under} of {len(floors)} crates under their floors")
        return 1
    print(f"coverage floor: {len(floors)} of {len(floors)} crates at or above their floors")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
