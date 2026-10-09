/*
Cvičení 02 — Úloha 2.5
Název: Nejmenší společný násobek
Body: 3 body

Zadání:
Uživatel zadá dvě čísla z klávesnice. Vypište jejich nejmenší společný násobek. Nejmenší společný
násobek čísel x a y je nejmenší číslo n takové, že x dělí n beze zbytku a zároveň y dělí n beze
zbytku.  [3 body]

Ukázka:

Spocteme nejmensi spolecny nasobek cisel

Zadej prvni cislo: 21

Zadej druhe cislo: 30

Nejmensi spolecny nasobek je 210

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

    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Console.WriteLine("Enter the first number:");

        int a = ReadInt();

        Console.WriteLine("Enter the second number:");

        int b = ReadInt();

        // Этот алгоритм рассчитан на положительные a и b. b = 0 вызывает ошибку деления,
        // a = 0 сразу даёт 0; отрицательные значения не подходят для этого способа поиска.
        var multiple = a;

        // Перебираем a, 2a, 3a, ... — они уже делятся на a. Первый такой кандидат,
        // который делится и на b без остатка, является наименьшим общим кратным.
        while (multiple % b != 0) {
            multiple += a;
        }

        Console.WriteLine("The least common multiple is {0}", multiple);
    }
}
