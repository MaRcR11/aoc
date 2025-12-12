def solve(input_data: str) -> str:
    p = input_data.split("\n\n")
    s = {}
    for x in p[:-1]:
        l = x.splitlines()
        i = int(l[0][:-1])
        s[i] = sum(r.count("#") for r in l[1:])

    t = 0
    for r in p[-1].splitlines():
        if not r.strip():
            continue
        sz, ns = r.split(": ")
        R, C = map(int, sz.split("x"))
        ns = list(map(int, ns.split()))
        total = sum(s[i] * n for i, n in enumerate(ns))
        if total * 1.3 < R * C:
            t += 1
    return str(t)
