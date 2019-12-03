import math

totalfuel = 0

def div3floorsub2(m):
  m /= 3
  m = math.floor(m)
  m -= 2
  return m

for i in range(1, 101):
    mass = int(input())
    mass = div3floorsub2(mass)
    totalfuel += mass

print(totalfuel)

