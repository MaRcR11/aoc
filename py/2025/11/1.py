def solve(input_data: str) -> str:
    g = {}
    for l in input_data.strip().splitlines():
        a, b = l.split(":")
        g[a.strip()] = b.split()
    tgt = "out"
    st = "you"

    def dfs(x, seen):
        if x == tgt:
            return 1
        t = 0
        for y in g.get(x, ()):
            if y not in seen:
                t += dfs(y, seen | {y})
        return t

    t = dfs(st, {st})
    return str(t)
