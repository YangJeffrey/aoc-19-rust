def TEST(in_put):
    with open('day-5.txt', 'r') as f:
        opcodelist = [int(op) for op in f.readline().split(',')]
    p = 0
    while opcodelist[p] != 99:
        args = {1: opcodelist[p + 1] if int((opcodelist[p] % 1000) / 100) else opcodelist[opcodelist[p + 1]],
                2: opcodelist[p + 2] if int((opcodelist[p] % 10000) / 1000) or opcodelist[p] % 100 == 4 else opcodelist[opcodelist[p + 2]]}
        instr = opcodelist[p] % 100
        instr_func = {1: (opcodelist[p + 3], args[1] + args[2], p + 4),
                      2: (opcodelist[p + 3], args[1] * args[2], p + 4),
                      3: (opcodelist[p + 1], in_put, p + 2),
                      4: (0, opcodelist[0], p + 2)}[instr]
        opcodelist[instr_func[0]] = instr_func[1]
        p = instr_func[2]
        if instr == 4:
            output = args[1]
    return(output)
print(TEST(1))
