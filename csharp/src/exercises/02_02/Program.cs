/*
Cvičení 02 — Úloha 2.2
Název: Tři čísla vzestupně
Body: 3 body

Zadání:
Uživatel zadá tři čísla z klávesnice. Setřiďte tato číslo ve vnitřní paměti. Vypište tato čísla
setříděná vzestupně dle jejich hodnot. [3 body]

Ukázka:

Setridime tri cisla vzestupne

Zadejte prvni cislo: 33

Zadejte druhe cislo: 6

Zadejte treti cislo: 14

Setridena cisla jsou 6, 14, 33

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

    // Вводим три целых числа, каждое на своей строке; Предполагается корректный ввод.

    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Console.WriteLine("Setridime tri cisla vzestupne");

        Console.WriteLine("Zadejte prvni cislo:");

        int a = ReadInt();

        Console.WriteLine("Zadejte druhe cislo:");
        // Каждое число читается отдельно.

        int b = ReadInt();

        Console.WriteLine("Zadejte treti cislo:");

        int c = ReadInt();

        // Три сравнения с обменом упорядочивают числа: сначала a и b, затем b и c,
        // потом снова a и b, потому что второй обмен мог нарушить их порядок.
        // В temp сохраняем заменяемое значение, чтобы не потерять его.
        if (a > b) {
            var temp = a;
            a = b;
            b = temp;
        }

        if (b > c) {
            var temp = b;
            b = c;
            c = temp;
        }

        if (a > b) {
            var temp = a;
            a = b;
            b = temp;
        }

        Console.WriteLine("Setridena cisla jsou {0}, {1}, {2}", a, b, c);
    }
}
