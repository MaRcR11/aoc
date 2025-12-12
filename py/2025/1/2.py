def solve(input_data: str) -> str:
    p, t = 50, 0
    for l in input_data.strip().split("\n"):
        for _ in range(int(l[1:])):
            p = (p + (1 if l[0] == "R" else -1)) % 100
            t += p == 0
    return str(t)
