/*
Cvičení 01 — Úloha 1.5
Název: Prostřední ze tří čísel
Body: 6 bodů

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami (ideálně různými). Napište program, který určí prostřední (= druhou
nejvyšší) z hodnot výše uvedených tří proměnných. Cíl příkladu a jeho výsledek vypište vhodným
způsobem na obrazovku. [6 bodů]

Ukázka:

Hodnota první proměnné je 11, hodnota druhé proměnné je 18, hodnota třetí proměnné je 5.

Prostřední z hodnot 11, 18 a 5 je 11.

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

    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int a = 11;
        int b = 18;
        int c = 5;

        // Среднее по порядку число — второе наибольшее, а не среднее арифметическое.
        int middle ;

        // Проверяем, лежит ли a между b и c при любом их порядке.
        // && означает «и», || — «или». Знаки >= и <= учитывают равные значения.
        if ((a >= b && a <= c) || (a <= b && a >= c)) {
            middle = a;
        // Если a не подходит, тем же способом проверяем b; иначе ответом остаётся c.
        } else if ((b >= a && b <= c) || (b <= a && b >= c)) {
            middle = b;
        } else {
            middle = c;
        }

        Console.WriteLine("The middle of {0}, {1} and {2} is {3}.", a, b, c, middle);
    }
}
