import re
import sys

filename = sys.argv[1]
file = open(filename).read()
ans1 = 0
ans2 = 0
enabled = True
for m in re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)", file):
    if m[0] == "do()":
        enabled = True
    elif m[0] == "don't()":
        enabled = False
    else:
        res = int(m[1]) * int(m[2])
        ans1 += res
        if enabled:
            ans2 += res
print(ans1)
print(ans2)
