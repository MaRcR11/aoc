from collections import deque


def solve(input_data: str) -> str:
    g = [list(l) for l in input_data.strip().splitlines()]
    R, C = len(g), len(g[0])
    d = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    deg = [[0] * C for _ in range(R)]

    for r in range(R):
        for c in range(C):
            if g[r][c] == "@":
                deg[r][c] = sum(
                    0 <= r + dr < R and 0 <= c + dc < C and g[r + dr][c + dc] == "@"
                    for dr, dc in d
                )

    q = deque(
        (r, c) for r in range(R) for c in range(C) if g[r][c] == "@" and deg[r][c] < 4
    )
    t = 0

    while q:
        r, c = q.popleft()
        if g[r][c] != "@":
            continue
        g[r][c] = "."
        t += 1
        for dr, dc in d:
            nr, nc = r + dr, c + dc
            if 0 <= nr < R and 0 <= nc < C and g[nr][nc] == "@":
                deg[nr][nc] -= 1
                if deg[nr][nc] == 3:
                    q.append((nr, nc))

    return str(t)
