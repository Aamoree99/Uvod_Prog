/*
Cvičení 05 — Úloha 5.7
Název: Vylepšený Count Sort
Body: 3 body

Zadání:
Rozšiřte funkci z příkladu  5.3d pro sestupné setřídění pole pomocí třídícího algoritmu Count Sort.
Hodnoty prvků pole jsou z omezeného intervalu, který může obsahovat i záporná čísla [3 body]


Ukázka:

int[] p1 = { -5, 7, 15, 22, 4, 0, 4, 9, -11 };

CountSort(p1); // 22, 15, 9, 7, 4, 4, 0, -5, -11

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

internal static class Program
{
    // Считаем только операции над значениями, а не управление циклами.
    static long assignments = 0, comparisons = 0;
    static int assign(int value) { assignments++; return value; }
    static bool less(int a, int b) { comparisons++; return a < b; }
    static bool greater(int a, int b) { comparisons++; return a > b; }
    // Смещение позволяет учитывать отрицательные значения без изменения порядка.
    static void countingSort(int[] a) {
        if (a.Length == 0) return;
        int minimum = a[0], maximum = a[0];
        for (int i = 1; i < a.Length; i++) {
            if (less(a[i], minimum)) minimum = assign(a[i]);
            if (greater(a[i], maximum)) maximum = assign(a[i]);
        }
        long range = (long)maximum - minimum + 1;
        if (range > 10000000) throw new ArgumentException("Counting sort range is too large");
        int[] counts = new int[(int)range];
        for (int i = 0; i < a.Length; i++) {
            int index = a[i] - minimum;
            counts[index] = assign(counts[index] + 1);
        }
        int position = 0;
        for (int index = counts.Length - 1; index >= 0; index--)
            while (greater(counts[index], 0)) {
                a[position++] = assign(index + minimum);
                counts[index] = assign(counts[index] - 1);
            }
    }
    static void printArray(int[] a) {
        for (int i = 0; i < a.Length; i++) Console.Write(a[i] + " ");
        Console.WriteLine();
    }

    public static void Main() {
        int[] a = {-5, 7, 15, 22, 4, 0, 4, 9, -11};
        countingSort(a);
        printArray(a);
        Console.WriteLine("Assignments: " + assignments + ", Comparisons: " + comparisons);
    }
}
