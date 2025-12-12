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
            return False
        if sz[ra] < sz[rb]:
            ra, rb = rb, ra
        p[rb] = ra
        sz[ra] += sz[rb]
        return True

    c = n
    last = None
    for _, i, j in d:
        if u(i, j):
            c -= 1
            last = (i, j)
            if c == 1:
                break
    t = pts[last[0]][0] * pts[last[1]][0]
    return str(t)
