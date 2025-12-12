from functools import lru_cache


def solve(input_data: str) -> str:
    g = {}
    for l in input_data.strip().splitlines():
        a, b = l.split(":")
        g[a.strip()] = b.split()
    st = "svr"
    tgt = "out"
    nd = {"dac": 1, "fft": 2}

    @lru_cache(None)
    def dfs(x, mask):
        if x == tgt:
            return 1 if mask == 3 else 0
        m = mask
        if x in nd:
            m |= nd[x]
        t = 0
        for y in g.get(x, ()):
            t += dfs(y, m)
        return t

    t = dfs(st, 0)
    return str(t)
