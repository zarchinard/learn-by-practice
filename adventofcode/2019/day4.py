begin = 134564
end = 585159


def check(n):
    sixDigitNumber = n > 99999 and n < 1000000
    inRange = n >= begin and n <= end
    array = list(map(int, list(str(n))))
    sameTwo = False
    for i in range(5):
        if array[i] == array[i + 1]:
            sameTwo = True
            break
    neverDecrease = True
    for i in range(5):
        if array[i] > array[i + 1]:
            neverDecrease = False
            break
    return sixDigitNumber and inRange and sameTwo and neverDecrease


def check2(n):
    array = list(map(int, list(str(n))))
    if array[0] == array[1] and array[1] != array[2]:
        return True
    if array[4] == array[5] and array[4] != array[3]:
        return True
    for i in range(1, 4):
        if (
            array[i] == array[i + 1]
            and array[i + 1] != array[i + 2]
            and array[i] != array[i - 1]
        ):
            return True
    return False


answer1 = 0
answer2 = 0
for n in range(begin, end + 1):
    if check(n):
        answer1 += 1
        if check2(n):
            answer2 += 1

print(answer1)
print(answer2)
