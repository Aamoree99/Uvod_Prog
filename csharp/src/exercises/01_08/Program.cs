/*
Cvičení 01 — Úloha 1.8
Název: Dělitel čísla
Body: 5 bodů

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, zda některé z čísel je dělitelem jiného
čísla, nebo zda žádné číslo není dělitelem jiného čísla. Je-li více správných dvojic číslo a jeho
dělitel, uveďte libovolnou z těchto dvojic. Cíl příkladu a jeho výsledek vypište vhodným způsobem
na obrazovku. [5 bodů]

Ukázka:

Hodnota první proměnné je 6, hodnota druhé proměnné je 3 hodnota třetí proměnné je 8.

Ze zadaných čísel 6, 3 a 8 je číslo 3 dělitelem čísla 6.

Hodnota první proměnné je 7, hodnota druhé proměnné je 2 hodnota třetí proměnné je 5.

Ze zadaných čísel 7, 2 a 5 není žádné číslo dělitelem jiného čísla.

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
        int a = 2;
        int b = 6;
        int c = 4;

        // Флаг запоминает, нашли ли хотя бы одну подходящую пару.
        bool found = false;

        // Проверяем все шесть направлений делимости между тремя числами.
        // Условие слева от && проверяется первым: при нулевом делителе остаток не вычисляется.
        if (a != 0 && b % a == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", a, b);
            found = true;
        }

        if (b != 0 && a % b == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", b, a);
            found = true;
        }

        if (a != 0 && c % a == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", a, c);
            found = true;
        }

        if (c != 0 && a % c == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", c, a);
            found = true;
        }

        if (b != 0 && c % b == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", b, c);
            found = true;
        }

        if (c != 0 && b % c == 0) {
            Console.WriteLine("Číslo {0} je dělitelem čísla {1}.", c, b);
            found = true;
        }

        // Сообщение об отсутствии делителей выводим только после проверки всех пар.
        // Этот вариант выводит все найденные пары, хотя по условию достаточно одной.
        if (found == false) {
            Console.WriteLine("Ze zadaných čísel {0}, {1} a {2} není žádné číslo dělitelem jiného čísla.", a, b, c);
        }
    }
}
