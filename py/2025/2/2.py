def solve(input_data: str) -> str:
    t = 0
    for r in input_data.strip().split(","):
        if r:
            a, b = map(int, r.split("-"))
            for x in range(a, b + 1):
                s = str(x)
                n = len(s)
                for k in range(1, n // 2 + 1):
                    if n % k == 0 and s[:k] * (n // k) == s:
                        t += x
                        break
    return str(t)
