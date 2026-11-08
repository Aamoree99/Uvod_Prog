def minimum():
    artem = [5, 2, 8, 1]

    min_value = artem[0]

    for i in range(len(artem)):
        if artem[i] < min_value:
            min_value = artem[i]

    print(min_value)


minimum()
