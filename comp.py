import time


def pow(x, y):
    if y > 0:
        return x * pow(x, y - 1)

    else:
        return 1.0


def factorial(x):
    if x > 0:
        return x * factorial(x - 1)

    else:
        return 1


def cos2(x):
    sum = 0
    i = 0
    mult = 1
    while i < 10:
        sum = sum + mult * pow(x, i) / factorial(i)
        mult = mult * -1
        i = i + 2

    return sum


start = time.time()
c = cos2(0.5)
t = time.time() - start
print("cos2(0) = ", c, " time = ", t)
print("t", time.time())
