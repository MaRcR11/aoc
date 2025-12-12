import re


def solve(input_data: str) -> str:
    L = input_data.strip().splitlines()
    t = 0
    for l in L:
        pat = re.findall(r"\[(.*?)\]", l)[0]
        tgt = [1 if c == "#" else 0 for c in pat]
        n = len(tgt)
        btns = []
        for b in re.findall(r"\((.*?)\)", l):
            if b.strip():
                btns.append(list(map(int, b.split(","))))
        m = len(btns)
        A = [[0] * m for _ in range(n)]
        for j, lst in enumerate(btns):
            for i in lst:
                A[i][j] = 1
        best = float("inf")
        for mask in range(1 << m):
            vec = [0] * n
            p = 0
            for j in range(m):
                if mask >> j & 1:
                    p += 1
                    for i in range(n):
                        if A[i][j]:
                            vec[i] ^= 1
            if vec == tgt and p < best:
                best = p
        t += best
    return str(t)
