def counting_sort():
    artem = [5, 2, 8, 1, 9, 4, 0, 9, 9, 9, 8, 8]

    max_value = artem[0]

    for i in range(len(artem)):
            if artem[i] > max_value:
                max_value = artem[i]

    counts = [0] * (max_value + 1)

    for i in range(len(artem)):
         counts[artem[i]] += 1 # i = 0 artem[i] = 5 => count[5] += 1 

    position = 0
    # 9 9 9 9 8 8 8 5 4 2 1 0
    for number in range(max_value, -1, -1):
         for j in range(counts[number]):
              artem[position] = number
              position += 1;

    print(artem)

counting_sort()

#O(n+k) n - колво элмен k - размер диапазона