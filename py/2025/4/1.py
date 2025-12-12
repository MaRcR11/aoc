def solve(input_data: str) -> str:
    g = [list(l) for l in input_data.strip().splitlines()]
    R, C = len(g), len(g[0])
    d = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    t = 0
    for r in range(R):
        for c in range(C):
            if g[r][c] == "@":
                t += (
                    sum(
                        0 <= r + dr < R and 0 <= c + dc < C and g[r + dr][c + dc] == "@"
                        for dr, dc in d
                    )
                    < 4
                )
    return str(t)
