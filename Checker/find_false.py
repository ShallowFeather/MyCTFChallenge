from z3 import *
import random
solver = Solver()

flag = [BitVec(f'flag_{i}', 16) for i in range(32)]

equations = [
    flag[28] + flag[14] + flag[20] + flag[5] + flag[26] == 331,
    flag[18] + flag[12] + flag[19] + flag[30] + flag[14] == 395,
    flag[25] + flag[4] + flag[2] + flag[19] + flag[3] == 442,
    flag[11] + flag[28] + flag[2] + flag[12] + flag[4] == 449,
    flag[16] + flag[22] + flag[15] + flag[11] + flag[3] == 670,
    flag[13] + flag[31] + flag[24] + flag[21] + flag[15] == 473,
    flag[2] + flag[29] + flag[5] + flag[4] + flag[28] == 420,
    flag[20] + flag[12] + flag[14] + flag[18] + flag[4] == 427,
    flag[3] + flag[23] + flag[27] + flag[7] + flag[12] == 417,
    flag[3] + flag[5] + flag[1] + flag[6] + flag[28] == 401,
    flag[11] + flag[27] + flag[2] + flag[7] + flag[21] == 351,
    flag[17] + flag[4] + flag[31] + flag[29] + flag[5] == 471,
    flag[1] + flag[27] + flag[21] + flag[22] + flag[3] == 341,
    flag[13] + flag[9] + flag[22] + flag[25] + flag[0] == 400,
    flag[21] + flag[24] + flag[25] + flag[3] + flag[15] == 379,
    flag[23] + flag[16] + flag[25] + flag[8] + flag[24] == 421,
    flag[19] + flag[31] + flag[24] + flag[12] + flag[27] == 312,
    flag[16] + flag[20] + flag[1] + flag[19] + flag[5] == 431,
    flag[29] + flag[20] + flag[30] + flag[12] + flag[15] == 392,
    flag[26] + flag[4] + flag[23] + flag[15] + flag[1] == 324,
    flag[16] + flag[20] + flag[30] + flag[4] + flag[0] == 400,
    flag[25] + flag[12] + flag[31] + flag[5] + flag[13] == 494,
    flag[8] + flag[17] + flag[26] + flag[5] + flag[30] == 345,
    flag[3] + flag[8] + flag[25] + flag[0] + flag[12] == 378,
    flag[9] + flag[11] + flag[20] + flag[10] + flag[18] == 380,
    flag[0] + flag[4] + flag[31] + flag[2] + flag[31] == 499,
    flag[3] + flag[30] + flag[6] + flag[11] + flag[18] == 338,
    flag[31] + flag[0] + flag[3] + flag[9] + flag[4] == 442,
    flag[20] + flag[18] + flag[27] + flag[14] + flag[2] == 355,
    flag[5] + flag[4] + flag[0] + flag[2] + flag[3] == 500,
    flag[28] + flag[6] + flag[27] + flag[8] + flag[2] == 430,
    flag[20] + flag[16] + flag[30] + flag[3] + flag[22] == 350,
    flag[8] + flag[3] + flag[7] + flag[23] + flag[21] == 342,
    flag[16] + flag[25] + flag[28] + flag[30] + flag[21] == 345,
    flag[16] + flag[7] + flag[12] + flag[20] + flag[19] == 474,
    flag[3] + flag[14] + flag[22] + flag[2] + flag[21] == 318,
    flag[24] + flag[7] + flag[31] + flag[10] + flag[27] == 499,
    flag[26] + flag[4] + flag[21] + flag[11] + flag[3] == 324,
    flag[19] + flag[31] + flag[25] + flag[13] + flag[27] == 487,
    flag[16] + flag[22] + flag[5] + flag[12] + flag[3] == 432
]
#random.shuffle(equations)
for f in flag:
    solver.add(And(f >= 48, f <= 128))
solver.add(flag[0] == ord('A'))
solver.add(flag[1] == ord('I'))
solver.add(flag[2] == ord('S'))
solver.add(flag[3] == ord('3'))
solver.add(flag[4] == ord('{'))
solver.add(flag[31] == ord('}'))

unsat_equations = []
for i in equations:
    solver.push()
    solver.add(i)

    if solver.check() == unsat:
        unsat_equations.append(i)

    solver.pop()

print("The following equations are causing unsatisfiability:")
for eq in unsat_equations:
    print(eq)
