IN = "./d2p1.in"

ids = []
with open(IN, 'r') as f:
    for line in f:
        ids.append(line)

    id_len = len(ids[0])

    for index1 in range(len(ids)):
        for index2 in range(len(ids[index1:])-1):
            #print(index1, " vs ", index1+index2+1)
            
            cnt = 0
            for i in range(id_len):
                if ids[index1][i] != ids[index1+index2+1][i]:
                    cnt += 1

            if cnt <= 1:
                #print(ids[index1])
                #print(ids[index1+index2+1])
                answer = ""
                for i in range(id_len):
                    if ids[index1][i] == ids[index1+index2+1][i]:
                        answer += str(ids[index1][i])
                print(answer)
                exit(0)
