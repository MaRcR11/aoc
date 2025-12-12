def solve(input_data: str) -> str:
    pts = [tuple(map(int, l.split(","))) for l in input_data.strip().splitlines()]
    n = len(pts)
    d = []
    for i in range(n):
        x1, y1, z1 = pts[i]
        for j in range(i + 1, n):
            x2, y2, z2 = pts[j]
            dx, dy, dz = x1 - x2, y1 - y2, z1 - z2
            d.append((dx * dx + dy * dy + dz * dz, i, j))
    d.sort()
    p = list(range(n))
    sz = [1] * n

    def f(x):
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x

    def u(a, b):
        ra, rb = f(a), f(b)
        if ra == rb:
            return
        if sz[ra] < sz[rb]:
            ra, rb = rb, ra
        p[rb] = ra
        sz[ra] += sz[rb]

    for _, i, j in d[:1000]:
        u(i, j)
    c = {}
    for i in range(n):
        r = f(i)
        c[r] = c.get(r, 0) + 1
    s = sorted(c.values(), reverse=True)
    t = s[0] * s[1] * s[2]
    return str(t)
