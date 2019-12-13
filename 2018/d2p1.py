IN = "./d2p1.in"

two = []
three = []

with open(IN, 'r') as f:
    for l in f:
        tmp = {}
        for c in l:
            if c in tmp:
                tmp[c] += 1
            else:
                tmp[c] = 1

        has_two = 0
        has_three = 0
        for k in tmp:
            if tmp[k] == 2 and has_two == 0:
                two.append(l)
                has_two = 1
            
            if tmp[k] == 3 and has_three == 0:
                three.append(l)
                has_three = 1

    print("two: ", len(two))
    print("three: ", len(three))
    print("mul: ", len(two) * len(three))


