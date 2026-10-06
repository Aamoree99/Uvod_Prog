/*
Cvičení 04 — Úloha 4.2
Název: Absolutní hodnota pole
Body: 1 bod

Zadání:
Napište funkci, která v zadaném poli nahradí hodnoty jednotlivých prvků pole jejich absolutní
hodnotou. [1 bod]

Ukázka:

int[] pole = { 5, -2, 0, -1, 7, -12, 2, 4 };

AbsolutniHodnotaPole(pole); // 5, 2, 0, 1, 7, 12, 2, 4

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

    // Массив передаётся без копирования; изменяем каждый отрицательный элемент на месте.
    static void absolutni_hodnota_pole(int[] pole) {
        int i=0;
        while (i < pole.Length) {
            if (pole[i] < 0) pole[i] = -pole[i];
            i += 1;
        }
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int[] pole = new int[] {5,-2,0,-1,7,-12,2,4};
        Console.WriteLine("Pred: {0}", "[" + string.Join(", ", pole) + "]"); absolutni_hodnota_pole(pole); Console.WriteLine("Po:   {0}", "[" + string.Join(", ", pole) + "]");
    }
}
