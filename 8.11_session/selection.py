def selection_sort():
    artem = [5, 2, 8, 1, 9, 4, 0, -12]

    #for(int i = 0; i < artem.Lengh; i++ )
    for i in range(len(artem) - 1):
        max_index = i

        for j in range(i + 1, len(artem)):
            if artem[j] > artem[max_index]:
                max_index = j

        temp = artem[i]
        artem[i] = artem[max_index]
        artem[max_index] = temp

    print(artem)

selection_sort()

#O(n^2)