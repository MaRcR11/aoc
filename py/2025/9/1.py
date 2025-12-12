def solve(input_data: str) -> str:
    pts = [
        tuple(map(int, l.split(",")))
        for l in input_data.strip().splitlines()
        if l.strip()
    ]
    t = 0
    n = len(pts)
    for i in range(n):
        x1, y1 = pts[i]
        for j in range(i + 1, n):
            x2, y2 = pts[j]
            t = max(t, (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1))
    return str(t)
