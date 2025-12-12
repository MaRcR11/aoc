def solve(input_data: str) -> str:
    p = input_data.strip().split("\n\n")
    r = sorted([tuple(map(int, l.split("-"))) for l in p[0].splitlines()])
    m = []
    for a, b in r:
        if not m or a > m[-1][1] + 1:
            m.append([a, b])
        else:
            m[-1][1] = max(m[-1][1], b)
    t = sum(b - a + 1 for a, b in m)
    return str(t)
