"""Whole-screen shot of each served task through escher-session, on two binaries; compare pixels.
argv: repo_root, head_bin, trial_bin. State dirs are relative (socket path bound), under target/tmp."""
import base64, hashlib, json, os, subprocess, sys, zlib, struct, shutil

root, head_bin, trial_bin = sys.argv[1:4]
os.chdir(root)
TASKS = ["counter", "flight-booker", "timer", "crud", "cells", "load"]

def run(binary, *args):
    p = subprocess.run([binary, *args], capture_output=True, text=True, timeout=120)
    return p.returncode, p.stdout.strip()

def decode_png(data):
    assert data[:8] == b"\x89PNG\r\n\x1a\n"
    pos, idat, w = 8, b"", None
    while pos < len(data):
        n = struct.unpack(">I", data[pos:pos + 4])[0]
        t = data[pos + 4:pos + 8]
        body = data[pos + 8:pos + 8 + n]
        if t == b"IHDR":
            w, h, depth, ctype = struct.unpack(">IIBB", body[:10])
        elif t == b"IDAT":
            idat += body
        pos += 12 + n
    assert depth == 8 and ctype in (2, 6)
    bpp = 3 if ctype == 2 else 4
    raw = zlib.decompress(idat)
    stride = w * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(h):
        f = raw[i]; line = bytearray(raw[i + 1:i + 1 + stride]); i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]; c = prev[x - bpp] if x >= bpp else 0
            if f == 1: line[x] = (line[x] + a) & 255
            elif f == 2: line[x] = (line[x] + b) & 255
            elif f == 3: line[x] = (line[x] + (a + b) // 2) & 255
            elif f == 4:
                p = a + b - c; pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line)); prev = line
    return w, h, bpp, rows

def shot(binary, task, tag):
    d = f"target/tmp/shot-{tag}-{task}"
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d, mode=0o700)
    rc, out = run(binary, "start", task, "--session", d)
    assert rc == 0, (task, tag, rc, out[:200])
    rc, out = run(binary, "screenshot", "--session", d)
    run(binary, "stop", "--session", d)
    shutil.rmtree(d, ignore_errors=True)
    assert rc == 0, (task, tag, rc, out[:200])
    return base64.b64decode(json.loads(out)["png"])

for task in TASKS:
    a, b = shot(head_bin, task, "head"), shot(trial_bin, task, "trial")
    if a == b:
        print(f"{task}: identical PNG ({len(a)} B, sha256 {hashlib.sha256(a).hexdigest()[:12]})")
        continue
    wa, ha, bpa, ra = decode_png(a); wb, hb, bpb, rb = decode_png(b)
    diff, maxd = 0, 0
    for y in range(min(ha, hb)):
        for x in range(min(wa, wb)):
            pa = ra[y][x * bpa:x * bpa + 3]; pb = rb[y][x * bpb:x * bpb + 3]
            if pa != pb:
                diff += 1; maxd = max(maxd, max(abs(pa[k] - pb[k]) for k in range(3)))
    print(f"{task}: differs — {wa}x{ha} vs {wb}x{hb}, {diff} of {wa*ha} pixels differ, max channel delta {maxd}")
