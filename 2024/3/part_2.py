f = open(input(), "r")
i = f.read().strip()
r = 0
just_start = True

while True:
    a = i.find("mul(")
    b = i.find("don't()")
    c = i.find("do()")

    if b < a and b < c:
        i = i[c:]
    elif a == b == c == -1:
        break
    else:
        i = i[a:]

    z = ["", ""]
    x = 0
    y = 0
    p = 0
    j = 4

    while True:
        if i[j].isdigit():
            z[p] += i[j]
            j += 1
        elif i[j] == "," and p == 0:
            j += 1
            p = 1
        elif i[j] == ")" and p == 1:
            x = int(z[0])
            y = int(z[1])
            j += 1
            break
        else:
            break

    r += x * y
    i = i[j:]

print(r)
