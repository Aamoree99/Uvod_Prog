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
    static void ArrayMaxima(int[] array, ref int largest, ref int secondLargest, ref int thirdLargest) {
        largest=0; secondLargest=0; thirdLargest=0;
        int i=0;
        while (i < array.Length) {
            int number=array[i];
            // Сдвигаем значения справа налево, чтобы не потерять прежние максимумы.
            if (number > largest) { thirdLargest=secondLargest; secondLargest=largest; largest=number; }
            else if (number > secondLargest) { thirdLargest=secondLargest; secondLargest=number; }
            else if (number > thirdLargest) { thirdLargest=number; }
            i += 1;
        }
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        int[] array = new int[] {2,5,7,1,4,6,9,8,2};
        int largest=0,secondLargest=0,thirdLargest=0;
        ArrayMaxima(array,ref largest,ref secondLargest,ref thirdLargest);
        Console.WriteLine("Array: {0}", "[" + string.Join(", ", array) + "]");
        Console.WriteLine("Largest:       {0}", largest);
        Console.WriteLine("Second largest: {0}", secondLargest);
        Console.WriteLine("Third largest: {0}", thirdLargest);
    }
}
