def maximum():
    artem = [5, 2, 8, 1]

    max_value = artem[0]

    for i in range(len(artem)):
        if artem[i] > max_value:
            max_value = artem[i]

    print(max_value)


maximum()
