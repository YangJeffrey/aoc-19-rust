def TEST(in_put):
    with open('input.txt', 'r') as f:
        op_list = [int(op) for op in f.readline().split(',')]
    p = 0
    while op_list[p] != 99:
        args = {1: op_list[p + 1] if int((op_list[p] % 1000) / 100) else op_list[op_list[p + 1]],
                2: op_list[p + 2] if int((op_list[p] % 10000) / 1000) or op_list[p]%100==4 else op_list[op_list[p + 2]]}
        instr = op_list[p] % 100
        instr_func = {1: (op_list[p + 3], args[1] + args[2], p + 4),
                      2: (op_list[p + 3], args[1] * args[2], p + 4),
                      3: (op_list[p + 1], in_put, p + 2),
                      4: (0, op_list[0], p + 2),
                      5: (0, op_list[0], args[2] if args[1] else p + 3),
                      6: (0, op_list[0], args[2] if not args[1] else p + 3),
                      7: (op_list[p + 3], int(args[1] < args[2]), p + 4),
                      8: (op_list[p + 3], int(args[1] == args[2]), p + 4)}[instr]
        op_list[instr_func[0]] = instr_func[1]
        p = instr_func[2]
        if instr == 4:
            output = args[1]
    return(output)
print(TEST(1)) #part 1
print(TEST(5)) #part 2
