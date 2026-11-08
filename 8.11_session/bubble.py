def bubble_sort():
    artem = [-5, 0, 4, -2, 4, 9, 0, 3]

    for i in range(len(artem) -1):
        for j in range(len(artem) - 1 - i):
            if artem[j] > artem[j+1]:
                temp = artem[j]
                artem[j] = artem[j+1]
                artem[j+1] = temp

    print(artem)

bubble_sort()