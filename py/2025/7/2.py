from functools import lru_cache


def solve(input_data: str) -> str:
    L = [l.rstrip("\n") for l in input_data.splitlines() if l.strip()]

    g = [list(l) for l in L]
    R, C = len(g), len(g[0])
    s = None
    for r in range(R):
        for c in range(C):
            if g[r][c] == "S":
                s = (r, c)
                break
        if s:
            break

    def nb(r, c):
        ch = g[r][c]
        res = []
        if ch == "^":
            for nc in (c - 1, c + 1):
                res.append((r, nc) if 0 <= nc < C else None)
        else:
            nr = r + 1
            res.append((nr, c) if nr < R else None)
        return res

    @lru_cache(None)
    def cnt(n):
        if n is None:
            return 1
        r, c = n
        return sum(cnt(x) for x in nb(r, c))

    t = cnt(s)
    return str(t)
