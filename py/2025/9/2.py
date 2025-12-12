from collections import deque


def solve(input_data: str) -> str:
    pts = [
        tuple(map(int, l.split(",")))
        for l in input_data.strip().splitlines()
        if l.strip()
    ]
    n = len(pts)
    xs = sorted(set(x for x, _ in pts))
    ys = sorted(set(y for _, y in pts))
    xv = {v: i for i, v in enumerate(xs)}
    yv = {v: i for i, v in enumerate(ys)}
    w, h = len(xs), len(ys)
    g = [[0] * h for _ in range(w)]
    for i in range(n):
        x1, y1 = pts[i]
        x2, y2 = pts[(i + 1) % n]
        if x1 == x2:
            x = xv[x1]
            a, b = yv[y1], yv[y2]
            if a > b:
                a, b = b, a
            for y in range(a, b + 1):
                g[x][y] = 1
        else:
            y = yv[y1]
            a, b = xv[x1], xv[x2]
            if a > b:
                a, b = b, a
            for x in range(a, b + 1):
                g[x][y] = 1
    q = deque()
    for x in range(w):
        for y in (0, h - 1):
            if g[x][y] == 0:
                g[x][y] = -1
                q.append((x, y))
    for y in range(h):
        for x in (0, w - 1):
            if g[x][y] == 0:
                g[x][y] = -1
                q.append((x, y))
    while q:
        x, y = q.popleft()
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nx, ny = x + dx, y + dy
            if 0 <= nx < w and 0 <= ny < h and g[nx][ny] == 0:
                g[nx][ny] = -1
                q.append((nx, ny))
    for x in range(w):
        for y in range(h):
            g[x][y] = 1 if g[x][y] != -1 else 0
    ps = [[0] * (h + 1) for _ in range(w + 1)]
    for x in range(w):
        s = 0
        for y in range(h):
            s += g[x][y]
            ps[x + 1][y + 1] = ps[x][y + 1] + s

    def ok(x1, y1, x2, y2):
        if x1 > x2:
            x1, x2 = x2, x1
        if y1 > y2:
            y1, y2 = y2, y1
        tot = (x2 - x1 + 1) * (y2 - y1 + 1)
        s = ps[x2 + 1][y2 + 1] - ps[x1][y2 + 1] - ps[x2 + 1][y1] + ps[x1][y1]
        return s == tot

    t = 0
    for i in range(n):
        x1, y1 = pts[i]
        for j in range(i + 1, n):
            x2, y2 = pts[j]
            if ok(xv[x1], yv[y1], xv[x2], yv[y2]):
                t = max(t, (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1))
    return str(t)
