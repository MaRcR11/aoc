def solve(input_data: str) -> str:
    t = 0
    for l in input_data.splitlines():
        d = [int(c) for c in l.strip()]
        t += max(d[i] * 10 + d[j] for i in range(len(d)) for j in range(i + 1, len(d)))
    return str(t)
