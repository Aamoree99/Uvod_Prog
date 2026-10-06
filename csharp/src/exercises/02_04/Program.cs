/*
Cvičení 02 — Úloha 2.4
Název: Faktoriál
Body: 1 bod

Zadání:
Uživatel zadá číslo. Vypište faktoriál tohoto čísla. Faktoriál čísla 0 je definován jako 1,
faktoriál pro číslo n>0 je definovaný jako součin všech čísel od 1 do n. Pro záporná čísla není
faktoriál definován. Př.: 6!  = 1*2*3*4*5*6 =720. [1 bod]

Ukázka:

Vypocteme faktorial kladneho cisla

Zadejte cislo: 6

Faktorial je 720

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
    // Читаем одну строку, как в Rust-примере; ошибки ввода не скрываем.
    private static int ReadInt() => int.Parse(Console.ReadLine() ?? throw new InvalidOperationException("End of input"), CultureInfo.InvariantCulture);

    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Console.WriteLine("Zadejte cislo:");

        // trim убирает перевод строки, parse преобразует текст в int. Ввод должен быть целым числом.
        int a = ReadInt();

        // Факториал определён для неотрицательных целых. В Rust 34! помещается в u128, а 35! уже нет; сохраняем этот предел и здесь.
        if (a < 0) {
            Console.WriteLine("Faktorial zaporneho cisla neni definovan.");
        } else if (a > 34) {
            Console.WriteLine("Hodnota je prilis vysoka pro datovy typ u128.");
        } else {
            // Начинаем произведение с 1. Для 0 цикл не выполняется, поэтому 0! = 1.
            BigInteger factorial = 1; // int вмещает 12!, но уже не вмещает 13!.
            int number = a;

            // Перемножаем a, a-1, ..., 1. Для точного большого произведения используем подходящий числовой тип.
            while (number >= 1) {
                factorial = factorial * number;
                number -= 1;
            }

            Console.WriteLine("Faktorial je {0}", factorial);
        }
    }
}
