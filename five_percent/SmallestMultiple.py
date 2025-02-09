max_num = 17

found = False
num = max_num
while not found:
    for i in range(1, max_num + 1):
        if num % i != 0:
            num += 1
            break
    else:
        print(num)
        break

