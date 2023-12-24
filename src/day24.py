from z3 import *

hailstones = []
with open("inputs/24.txt") as f:
    for line in f.read().split("\n"):
        pos, vel = line.split(" @ ")
        pos = [int(x.strip()) for x in pos.split(", ")]
        vel = [int(x.strip()) for x in vel.split(", ")]
        hailstones.append((pos, vel))

count = 0
for i, (pos_a, vel_a) in enumerate(hailstones):
    print(i)
    for (pos_b, vel_b) in hailstones[i+1:]:
        solver = Solver()
        pos_a_x = pos_a[0]
        pos_a_y = pos_a[1]
        vel_a_x = vel_a[0]
        vel_a_y = vel_a[1]

        pos_b_x = pos_b[0]
        pos_b_y = pos_b[1]
        vel_b_x = vel_b[0]
        vel_b_y = vel_b[1]

        time_a = Real("time_a")
        time_b = Real("time_b")
        line_a_x = pos_a_x + vel_a_x * time_a
        line_a_y = pos_a_y + vel_a_y * time_a
        line_b_x = pos_b_x + vel_b_x * time_b
        line_b_y = pos_b_y + vel_b_y * time_b
        solver.add(time_a >= 0)
        solver.add(time_b >= 0)
        solver.add(line_a_x == line_b_x)
        solver.add(line_a_y == line_b_y)
        solver.add(line_a_x >= 200000000000000)
        solver.add(line_a_x <= 400000000000000)
        solver.add(line_a_y >= 200000000000000)
        solver.add(line_a_y <= 400000000000000)
        if solver.check() == sat:
            count += 1
print(count)