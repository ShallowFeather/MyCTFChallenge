import random
a = "AIS3{UrWINn3r_1n_WInD0WS_K3RN31}"

for i in range(35):
    cnt = 0    
    print("s.add(", end="")
    check = []
    for j in range(0, 5):
        rand = random.randint(0, 31)
        while(rand in check):
            rand = random.randint(0, 31)
        check.append(rand)
        cnt += ord(a[rand])
        if j == 4:
            print(f"a[{rand}]", end=" == ")
        else:
            print(f"a[{rand}]", end=" + ")
#print("")
    print(cnt, end="")
    print(")")
