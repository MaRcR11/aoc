def solve(input_data: str) -> str:
    p, t = 50, 0
    for l in input_data.splitlines():
        p = (p + int(l[1:]) * (1 - 2 * (l[0] == "L"))) % 100
        t += p == 0
    return str(t)
