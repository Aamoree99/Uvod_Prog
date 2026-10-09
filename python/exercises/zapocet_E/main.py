"""
Zápočet — E
Název: Druhé největší číslo v poli

Тест на оценку 3: найти второе максимальное число в массиве.
Это учебное решение по описанию теста, а не копия полного условия.
Здесь второй максимум — второй элемент по убыванию с учётом повторов:
для [9, 9, 5] ответ 9. Если нужны разные значения, правило будет другим.
"""

def second_maximum(numbers):
    if len(numbers) < 2:
        return None
    maximum, second = numbers[0], numbers[1]
    if second > maximum:
        maximum, second = second, maximum
    i = 2
    while i < len(numbers):
        current = numbers[i]
        if current > maximum:
            second, maximum = maximum, current
        elif current > second:
            second = current
        i += 1
    return second


def main():
    numbers = [2, 5, 7, 1, 4, 6, 9, 8, 2]
    print(f"Array: {numbers}")
    print(f"Second maximum: {second_maximum(numbers)}")

if __name__ == "__main__":
    main()
