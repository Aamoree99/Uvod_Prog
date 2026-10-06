/*
Cvičení 01 — Úloha 1.9
Název: Sčítance čísla
Body: 5 bodů

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, zda některé z čísel je součtem zbývajících
dvou čísel, nebo zda žádná dvě čísla netvoří součet třetího čísla. Cíl příkladu a jeho výsledek
vypište vhodným způsobem na obrazovku. [5 bodů]

Ukázka:

Hodnota první proměnné je 4, hodnota druhé proměnné je 13, hodnota třetí proměnné je 9.

Ze zadaných čísel 4, 13 a 9 je číslo 13 součtem čísel 4 a 9.

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

    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int a = 4;
        int b = 13;
        int c = 9;

        // Любое из трёх чисел может оказаться суммой двух других, поэтому проверяем три варианта.
        // Цепочка else if выводит только первое найденное совпадение.
        if (a + b == c) {
            Console.WriteLine("Ze zadaných čísel {0}, {1} a {2} je číslo {3} součtem čísel {4} a {5}.", a, b, c, c, a, b);
        } else if (a + c == b) {
            Console.WriteLine("Ze zadaných čísel {0}, {1} a {2} je číslo {3} součtem čísel {4} a {5}.", a, b, c, b, a, c);
        } else if (b + c == a) {
            Console.WriteLine("Ze zadaných čísel {0}, {1} a {2} je číslo {3} součtem čísel {4} a {5}.", a, b, c, a, b, c);
        } else {
            Console.WriteLine("Ze zadaných čísel {0}, {1} a {2} žádná dvě čísla netvoří součet třetího čísla.", a, b, c);
        }
    }
}
