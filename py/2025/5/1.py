def solve(input_data: str) -> str:
    p = input_data.strip().split("\n\n")
    r = sorted([tuple(map(int, l.split("-"))) for l in p[0].splitlines()])
    m = []
    for a, b in r:
        if not m or a > m[-1][1] + 1:
            m.append([a, b])
        else:
            m[-1][1] = max(m[-1][1], b)
    ids = list(map(int, p[1].split()))

    def f(x):
        left, right = 0, len(m) - 1
        while left <= right:
            mid = (left + right) // 2
            if m[mid][0] <= x <= m[mid][1]:
                return True
            if x < m[mid][0]:
                right = mid - 1
            else:
                left = mid + 1
        return False

    t = sum(f(x) for x in ids)
    return str(t)
