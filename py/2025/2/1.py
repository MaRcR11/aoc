def solve(input_data: str) -> str:
    t = 0
    for r in input_data.strip().split(","):
        if r:
            a, b = map(int, r.split("-"))
            t += sum(
                x
                for x in range(a, b + 1)
                if len(s := str(x)) % 2 == 0 and s[: len(s) // 2] == s[len(s) // 2 :]
            )
    return str(t)
