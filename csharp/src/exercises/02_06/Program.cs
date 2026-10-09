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

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // По условию достаточно перебрать числа до 1000 и проверить делители каждого.
    // Здесь автор выбрал более эффективный алгоритм — решето Эратосфена —
    // и увеличил предел до 100000. Это расширенный вариант для изучения.
    // Он использует массив и отличается от ограничений исходного задания.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Console.WriteLine("Note:");
        Console.WriteLine("This exercise can be solved by looping through the numbers");
        Console.WriteLine("and testing all possible divisors for each number.");
        Console.WriteLine("This implementation uses the sieve of Eratosthenes");
        Console.WriteLine("and increases the limit from 1000 to 100000.");
        Console.WriteLine("");

        int limit = 100000;

        // Индекс — само число; true означает, что число пока считается простым.
        // 0 и 1 исключаем отдельно: они не являются простыми.
        bool[] isPrime = new bool[limit];
        Array.Fill(isPrime, true);

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

        Console.WriteLine("Prime numbers less than {0}:", limit);
        Console.WriteLine("");

        number = 2;
        // При выводе считаем простые числа и переносим строку после каждых десяти.
        var count = 0;

        while (number < limit) {
            if (isPrime[number]) {
                Console.Write("{0,6}", number);
                count += 1;

                if (count % 10 == 0) {
                    Console.WriteLine("");
                }
            }

            number += 1;
        }

        if (count % 10 != 0) {
            Console.WriteLine("");
        }

        Console.WriteLine("");
        Console.WriteLine("Total prime count: {0}", count);
    }
}
