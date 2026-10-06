/*
Zápočet — E
Název: Druhé největší číslo v poli

Тест на оценку 3: найти второе максимальное число в массиве.
Это учебное решение по описанию теста, а не копия полного условия.
Здесь второй максимум — второй элемент по убыванию с учётом повторов:
для [9, 9, 5] ответ 9. Если нужны разные значения, правило будет другим.
*/

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // Второй максимум учитывает повторы: для [9,9,5] ответ 9.
    // Отсутствующий результат отличаем от любого целого числа, включая минимальное.
    static int? second_maximum(int[] numbers) {
        if (numbers.Length < 2) return null;
        // Начинаем с реальных элементов, поэтому отрицательные значения тоже работают.
        int maximum=numbers[0], second=numbers[1];
        if (second > maximum) { int temp=maximum; maximum=second; second=temp; }
        int index=2;
        while (index < numbers.Length) {
            int current=numbers[index];
            if (current > maximum) {
                // Сначала сохраняем старого лидера, потом заменяем его новым.
                second=maximum; maximum=current;
            } else if (current > second) second=current;
            index += 1;
        }
        // Один проход: O(n) времени, O(1) дополнительной памяти.
        return second;
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int[] numbers = new int[] {2,5,7,1,4,6,9,8,2};
        Console.WriteLine("Массив: {0}", "[" + string.Join(", ", numbers) + "]");
        int? value=second_maximum(numbers);
        if (value.HasValue) Console.WriteLine("Второе максимальное число: {0}", value.Value);
        else Console.WriteLine("Для второго максимума нужны хотя бы два элемента.");
    }
}
