/*
Zápočet — E
Název: Druhé největší číslo v poli

Тест на оценку 3: найти второе максимальное число в массиве.
Это учебное решение по описанию теста, а не копия полного условия.
Здесь второй максимум — второй элемент по убыванию с учётом повторов:
для [9, 9, 5] ответ 9. Если нужны разные значения, правило будет другим.
*/

package exercises.task_zapocet_E;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Второй максимум учитывает повторы: для [9,9,5] ответ 9.
    // Отсутствующий результат отличаем от любого целого числа, включая минимальное.
    static Integer second_maximum(int[] numbers) {
        if (numbers.length < 2) return null;
        // Начинаем с реальных элементов, поэтому отрицательные значения тоже работают.
        int maximum=numbers[0], second=numbers[1];
        if (second > maximum) { int temp=maximum; maximum=second; second=temp; }
        int index=2;
        while (index < numbers.length) {
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
    public static void main(String[] args) {
        int[] numbers = new int[] {2,5,7,1,4,6,9,8,2};
        System.out.printf(Locale.ROOT, "Массив: %s%n", Arrays.toString(numbers));
        Integer value=second_maximum(numbers);
        if (value != null) System.out.printf(Locale.ROOT, "Второе максимальное число: %s%n", value);
        else System.out.printf(Locale.ROOT, "Для второго максимума нужны хотя бы два элемента.%n");
    }
}
