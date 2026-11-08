def higher_avg(pole):
    sum = 0
    for i in range(len(pole)):
        sum += pole[i]
    avg = sum / len(pole)
    count = 0

    for i in range(len(pole)):
        if pole[i] > avg:
            count += 1

    return count

pole = [2, 4, 6, 8, 10]

print(higher_avg(pole))  # 2

    