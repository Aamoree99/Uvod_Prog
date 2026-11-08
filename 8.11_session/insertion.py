def inserion_sort():
    artem = [5, 2, 8, 1, 9, 4, 0, -12]

    for i in range(1, len(artem)):
        current = artem[i]
        j = i - 1

        while j >= 0 and artem[j] < current:
            artem[j + 1] = artem[j]
            j -= 1

        artem[j + 1] = current

    print(artem)


inserion_sort()
