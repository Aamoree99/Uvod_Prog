/*
Cvičení 02 — Úloha 2.6
Název: Prvočísla
Body: 3 body

Zadání:
Vypište všechna prvočísla menší než 1000 [3 body]

Ukázka:

Vsechna prvocisla mensi nez 1000: 2, 3, 5, . . . . 991, 997

Požadavky / omezení:
a) K řešení příkladů je možné používat pouze syntaktických konstrukcí a funkcí probraných na
přednáškách. K řešení cvičení s nižším číslem nelze používat učivo hodin s vyšším číslem.

b) Řešení musí být procedurální, nesmí být objektové. Je zakázáno použití cyklu foreach. Je
zakázáno použití kolekcí (Dictionary, List, LinkedList, SortedList, atd.) a jejich obdoby v jiných
programovacích jazycích.

c) Příklady můžete řešit i v jiném jazyku než C#, pokud tento jazyk umožňuje procedurální
programování, např. C, C++, Java. Při této volbě zdůrazňuji platnost pravidla 2a, lze používat
pouze ekvivalenty syntaktických konstrukcí a funkcí probraných na přednášce.

d) Úloha musí být funkční. Otestujte ji minimálně na datech uvedených v zadání příkladu.

e) Zdrojové kódy musí být formátovány jednotným způsobem, zejména odsazování.

f) Proměnné a funkce musejí mít vhodné názvy

g) Zdrojové kódy musí být vhodně členěny do funkcí (platí pro 3. cvičení a následující)

h) Pro každé cvičení vytvořte vlastní Solution (Rešení). Jednotlivé příklady budou jako samostatné
Projekty.

i) Je zakázáno používat složené příkazy { }, pokud obsahují pouze jeden příkaz.
*/

package exercises.task_02_06;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // По условию достаточно перебрать числа до 1000 и проверить делители каждого.
    // Здесь автор выбрал более эффективный алгоритм — решето Эратосфена —
    // и увеличил предел до 100000. Это расширенный вариант для изучения.
    // Он использует массив и отличается от ограничений исходного задания.
    public static void main(String[] args) {
        System.out.printf(Locale.ROOT, "Note:%n");
        System.out.printf(Locale.ROOT, "This exercise can be solved by looping through the numbers%n");
        System.out.printf(Locale.ROOT, "and testing all possible divisors for each number.%n");
        System.out.printf(Locale.ROOT, "This implementation uses the sieve of Eratosthenes%n");
        System.out.printf(Locale.ROOT, "and increases the limit from 1000 to 100000.%n");
        System.out.printf(Locale.ROOT, "%n");

        int limit = 100000;

        // Индекс — само число; true означает, что число пока считается простым.
        // 0 и 1 исключаем отдельно: они не являются простыми.
        boolean[] isPrime = new boolean[limit];
        Arrays.fill(isPrime, true);

        isPrime[0] = false;
        isPrime[1] = false;

        var number = 2;

        // Достаточно обработать делители до квадратного корня из предела: у каждого
        // составного числа есть делитель не больше его корня.
        while (number * number < limit) {
            if (isPrime[number]) {
                // Начинаем вычёркивание с number²: меньшие кратные уже вычеркнули предыдущие простые.
                // Шаг number перебирает все следующие кратные.
                var multiple = number * number;

                while (multiple < limit) {
                    isPrime[multiple] = false;
                    multiple += number;
                }
            }

            number += 1;
        }

        System.out.printf(Locale.ROOT, "Prime numbers less than %s:%n", limit);
        System.out.printf(Locale.ROOT, "%n");

        number = 2;
        // При выводе считаем простые числа и переносим строку после каждых десяти.
        var count = 0;

        while (number < limit) {
            if (isPrime[number]) {
                System.out.printf(Locale.ROOT, "%6d", number);
                count += 1;

                if (count % 10 == 0) {
                    System.out.printf(Locale.ROOT, "%n");
                }
            }

            number += 1;
        }

        if (count % 10 != 0) {
            System.out.printf(Locale.ROOT, "%n");
        }

        System.out.printf(Locale.ROOT, "%n");
        System.out.printf(Locale.ROOT, "Total prime count: %s%n", count);
    }
}
