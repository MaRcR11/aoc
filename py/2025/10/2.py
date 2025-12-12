import re
import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp


def solve(input_data: str) -> str:
    L = input_data.strip().split("\n")
    t = 0
    for l in L:
        btns = []
        for m in re.finditer(r"\(([0-9,]+)\)", l):
            btns.append([int(x) for x in m.group(1).split(",")])
        j_match = re.search(r"\{([0-9,]+)\}", l)
        jolts = [int(x) for x in j_match.group(1).split(",")]
        n, m = len(jolts), len(btns)
        A = np.zeros((n, m), dtype=float)
        for j, btn in enumerate(btns):
            for i in btn:
                A[i][j] = 1
        bl = np.array(jolts, dtype=float)
        bu = np.array(jolts, dtype=float)
        c = np.ones(m)
        cons = LinearConstraint(A, bl, bu)
        integ = np.ones(m)
        bnds = Bounds(0, np.inf)
        res = milp(c=c, constraints=cons, integrality=integ, bounds=bnds)
        if res.success:
            t += int(np.sum(res.x))
    return str(t)
