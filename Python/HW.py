import random

def generat():
    x = [random.randint(0, 122) for i in range(12)]
    return x

def distance(sol:str):
    ans = "Hello World!"
    res = [abs(ord(ans[i]) - sol[i]) for i in range(12)]

    return sum(res)

def cross(x, y):
    r = random.randint(0, 2**12)
    a1 = []
    for i in range(12):
        if r >> i & 1 == 1:
            a1.append(y[i])
        else:
            a1.append(x[i])

    return a1


def next_gen(group):
    score = [(i, distance(i)) for i in group]
    score.sort(key=lambda x:x[1])
    tmp = [i[0] for i in score[:10]]
    for i in range(10):
        for j in range(i+1, 10):
            t1 = cross(group[i], group[j])
            tmp.append(t1)
    r = random.randint(1, 2**32)
    c = 0
    res = []
    for i in range(32):
        if r >> i & 1 == 1:
            res.append(tmp[i])
            c += 1
    res.extend([generat() for i in range(100 - c)])

    return res

def main():
    group = [generat() for i in range(100)]
    cnt = 0

    while distance(group[0]) != 0:
        if cnt % 1000 == 0:
            t1 = "".join(map(chr, group[0]))
            t2 = distance(group[0])
            print("{} : {}".format(t1, t2))
        group = next_gen(group)
        cnt += 1
    
    print("".join(map(chr, group[0])))
    print("{}世代目で完成".format(cnt))


if __name__ == "__main__":
    main()