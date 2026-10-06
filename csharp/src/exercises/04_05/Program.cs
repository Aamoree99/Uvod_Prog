/*
Cvičení 04 — Úloha 4.5
Název: Maximum z pole
Body: 1 bod; 1 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která v zadaném poli nezáporných čísel najde a pomocí parametrů funkce vrátí:

a) hodnotu největšího prvku [1 bod]

b) hodnotu druhého největšího prvku [1 bod]

c) třetího největšího prvku [2 body]

Ukázka:

int[] pole = { 2, 5, 7, 1, 4, 6, 9, 8, 2 };

MaxPole(pole, ref max, ref max2, ref max3); // 9, 8, 7

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

    // По условию числа неотрицательные, поэтому начальные максимумы равны нулю.
    // Повторы учитываются; для короткого массива незаполненные места остаются нулевыми.
    static void max_pole(int[] pole, ref int max, ref int max2, ref int max3) {
        max=0; max2=0; max3=0;
        int i=0;
        while (i < pole.Length) {
            int cislo=pole[i];
            // Сдвигаем значения справа налево, чтобы не потерять прежние максимумы.
            if (cislo > max) { max3=max2; max2=max; max=cislo; }
            else if (cislo > max2) { max3=max2; max2=cislo; }
            else if (cislo > max3) { max3=cislo; }
            i += 1;
        }
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int[] pole = new int[] {2,5,7,1,4,6,9,8,2};
        int max=0,max2=0,max3=0;
        max_pole(pole,ref max,ref max2,ref max3);
        Console.WriteLine("Pole: {0}", "[" + string.Join(", ", pole) + "]");
        Console.WriteLine("Nejvetsi:       {0}", max);
        Console.WriteLine("Druhe nejvetsi: {0}", max2);
        Console.WriteLine("Treti nejvetsi: {0}", max3);
    }
}
