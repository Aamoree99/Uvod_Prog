/*
Cvičení 04 — Úloha 4.1
Název: Sestupné setřídění
Body: 1 bod

Zadání:
Napište funkci, která své dva parametry setřídí sestupně. [1 bod]

Ukázka:

int a = 5, b = 7;

SetridSestupne(ref a, ref b); // a = 7; b = 5;

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

    // Передаём оба числа по ссылке, чтобы обмен изменил переменные вызывающей функции.
    static void setrid_sestupne(ref int a,ref int b) {
        if (a < b) { int pomocna=a; a=b; b=pomocna; }
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int a=5,b=7;
        Console.WriteLine("Pred: a = {0}, b = {1}", a, b);
        setrid_sestupne(ref a,ref b);
        Console.WriteLine("Po:   a = {0}, b = {1}", a, b);
    }
}
