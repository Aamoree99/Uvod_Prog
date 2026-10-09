/*
Cvičení 01 — Úloha 1.10
Název: Jízdné v MHD
Body: 3 body

Zadání:
Jízdné v MHD je závislé na věku cestujícího. Lze jezdit zdarma, za poloviční jízdné 12 Kč a za plné
jízdné 24 Kč v závislosti na splnění následujících podmínek. Pokud cestující splní více podmínek,
platí pro něj nejvýhodnější jízdné. Kdokoliv může jezdit za plné jízdné. Cestující mladší 6 let
mohou jezdit zdarma. Cestující mladší 15 let nebo ve věku 60 let a starší mohou jezdit za poloviční
jízdné.

Ve funkci Main deklarujte jednu celočíselnou proměnnou (pojmenujte je libovolně) a tuto proměnnou
inicializujte hodnotou, které by mohla reprezentovat věk cestujícího.  Napište program, který určí,
jízdné pro daný věk cestujícího. Cíl příkladu a jeho výsledek vypište vhodným způsobem na
obrazovku. [3 body]

Ukázka:

Hodnota proměnné je 9.

Cestující ve věku 9 let zaplatí jízdné 12 Kč.

Hodnota proměnné je 15.

Cestující ve věku 15 let zaplatí jízdné 24 Kč.

Hodnota proměnné je 3.

Cestující ve věku 3 let zaplatí jízdné 0 Kč.

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
        int age = 9;

        // Проверяем тарифы по порядку. После первой ветки уже известно, что возраст >= 6.
        // Во второй ветке скидку получают дети до 15 лет либо люди от 60 лет включительно.
        if (age < 6) {
            Console.WriteLine("A passenger aged {0} pays a fare of 0 CZK.", age);
        } else if (age < 15 || age >= 60) {
            Console.WriteLine("A passenger aged {0} pays a fare of 12 CZK.", age);
        } else {
            Console.WriteLine("A passenger aged {0} pays a fare of 24 CZK.", age);
        }
    }
}
