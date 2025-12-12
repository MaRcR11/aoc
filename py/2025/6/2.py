def solve(input_data: str) -> str:
    lines = input_data.rstrip("\n").split("\n")
    H, W = len(lines), max(len(l) for l in lines)
    g = [l.ljust(W) for l in lines]
    sc = [all(g[r][c] == " " for r in range(H)) for c in range(W)]
    seg, i = [], 0

    while i < W:
        if sc[i]:
            i += 1
        else:
            j = i
            while j < W and not sc[j]:
                j += 1
            seg.append((i, j))
            i = j
    t = 0
    for a, b in seg:
        nums = []
        for c in range(a, b):
            col = "".join(g[r][c] for r in range(H - 1)).strip()
            if col and col.isdigit():
                nums.append(int(col))
        op = g[H - 1][a:b].strip()
        v = sum(nums) if op == "+" else 1
        if op != "+":
            for x in nums:
                v *= x
        t += v
    return str(t)
