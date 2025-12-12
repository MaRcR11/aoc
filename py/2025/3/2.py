def solve(input_data: str) -> str:
    t = 0
    for l in input_data.splitlines():
        d = [int(c) for c in l.strip()]
        n, s, r = len(d), [], len(d) - 12
        for x in d:
            while s and r > 0 and s[-1] < x:
                s.pop()
                r -= 1
            s.append(x)
        t += int("".join(map(str, s[:12])))
    return str(t)
