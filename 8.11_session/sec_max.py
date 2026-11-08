def sec_maximum(): #def - static/fn/func sec_maximum - NAME () -передача значения  
    artem = [5, 2, 8, 1] # обьявление масива

    max_value = artem[0] #обьявление переменных
    sec_max = artem[0] #обьявление переменных
    for i in range(len(artem)):
        if artem[i] > max_value:
            sec_max = max_value
            max_value = artem[i]

    print(max_value, sec_max) # вывод значений


sec_maximum() #питон вызов функции
