def solve(input_data: str) -> str:
    L = input_data.strip().split("\n")
    g = [list(l) for l in L]
    R, C = len(g), len(g[0]) if g else 0
    sr, sc = None, None
    for r in range(R):
        for c in range(C):
            if g[r][c] == "S":
                sr, sc = r, c
                break
        if sr is not None:
            break
    t = 0
    b = [(sr, sc, "down")]
    v = set()
    while b:
        nb = []
        for row, col, d in b:
            if d == "down":
                r = row + 1
                while r < R:
                    if g[r][col] == "^":
                        s = (r, col)
                        if s not in v:
                            v.add(s)
                            t += 1
                            nb.append((r, col - 1, "left"))
                            nb.append((r, col + 1, "right"))
                        break
                    r += 1
            elif d == "left":
                c = col
                while c >= 0:
                    if c < C and g[row][c] == "^":
                        s = (row, c)
                        if s not in v:
                            v.add(s)
                            t += 1
                            nb.append((row, c - 1, "left"))
                            nb.append((row, c + 1, "right"))
                        break
                    nb.append((row + 1, c, "down"))
                    break
            elif d == "right":
                c = col
                while c < C:
                    if c >= 0 and g[row][c] == "^":
                        s = (row, c)
                        if s not in v:
                            v.add(s)
                            t += 1
                            nb.append((row, c - 1, "left"))
                            nb.append((row, c + 1, "right"))
                        break
                    nb.append((row + 1, c, "down"))
                    break
        b = nb
    return str(t)
