/*
Cvičení 05 — Úloha 5.4
Název: Testování rychlostí algoritmů
Body: 2 body; 2 body (v pořadí podúloh)

Zadání:
Otestujte třídící algoritmy z příkladu 5.3 na polích různé velikosti (1000, 10 000 a 30 000 prvků),
naplněných náhodnými hodnotami z intervalu 0 až velikost pole. Všechny třídící algoritmy budou
pracovat nad kopií stejného pole. Výsledky přehledně prezentujte.

a) s použitím počtu operací [2 body]

b) s použitím měření času [2 body]


Ukázka – část (a):

Prvku 1000

Selection sort – porovnani: 500500, prirazeni: 3000

Insertion sort – porovnani: 259065, prirazeni: 260069

Bubble sort – porovnani: 974025, prirazeni: 774213

Count sort – porovnani: 2995, prirazeni: 3006

Prvku 10000

...

Prvku 30000

...


Ukázka – část (b):

Prvku 1000

...

Prvku 10000

...

Prvku 30000

Selection sort – doba behu 11194 ms

Insertion sort – doba behu 9300 ms

Bubble sort – doba behu 39415 ms

Count sort – doba behu 4 ms

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

    static void selectionSort(int[] a) {
        for (int i = 0; i < a.Length; i++) {
            int maximum = i;
            for (int j = i + 1; j < a.Length; j++)
                if (greater(a[j], a[maximum])) maximum = j;
            int temporary = assign(a[i]);
            a[i] = assign(a[maximum]);
            a[maximum] = assign(temporary);
        }
    }
    static void insertionSort(int[] a) {
        for (int i = 1; i < a.Length; i++) {
            int key = assign(a[i]), j = i;
            while (j > 0 && less(a[j - 1], key)) {
                a[j] = assign(a[j - 1]);
                j--;
            }
            a[j] = assign(key);
        }
    }
    static void bubbleSort(int[] a) {
        for (int i = 0; i < a.Length; i++)
            for (int j = 0; j + 1 < a.Length - i; j++)
                if (less(a[j], a[j + 1])) swap(a, j);
    }
    static void swap(int[] a, int j) {
        int temporary = assign(a[j]);
        a[j] = assign(a[j + 1]);
        a[j + 1] = assign(temporary);
    }
    // Счётчики и диапазон соответствуют исходной версии Rust.
    static void countingSort(int[] a) {
        if (a.Length == 0) return;
        int maximum = 0;
        for (int i = 0; i < a.Length; i++) {
            if (a[i] < 0 || a[i] > 30000) throw new ArgumentException("Counting sort value outside range");
            if (greater(a[i], maximum)) maximum = assign(a[i]);
        }
        int[] counts = new int[30000 + 1];
        for (int i = 0; i < a.Length; i++) counts[a[i]] = assign(counts[a[i]] + 1);
        int position = 0;
        for (int number = maximum; number >= 0; number--)
            while (greater(counts[number], 0)) {
                a[position++] = assign(number);
                counts[number] = assign(counts[number] - 1);
            }
    }
    static void printArray(int[] a) {
        for (int i = 0; i < a.Length; i++) Console.Write(a[i] + " ");
        Console.WriteLine();
    }
    static void sort(int[] a, int algorithm) {
        if (algorithm == 0) selectionSort(a);
        else if (algorithm == 1) insertionSort(a);
        else if (algorithm == 2) bubbleSort(a);
        else countingSort(a);
    }

    static bool isDescending(int[] a) {
        for (int i = 1; i < a.Length; i++)
            if (a[i] > a[i - 1]) return false;
        return true;
    }
    static void benchmark(int[] original, int algorithm, string name) {
        int[] a = (int[])original.Clone();
        assignments = comparisons = 0;
        long start = System.Diagnostics.Stopwatch.GetTimestamp();
        sort(a, algorithm);
        double milliseconds = (System.Diagnostics.Stopwatch.GetTimestamp() - start) * 1000.0 / System.Diagnostics.Stopwatch.Frequency;
        if (!isDescending(a)) throw new InvalidOperationException("Sorting failed");
        Console.WriteLine(name + " | Comparisons: " + comparisons + " | Assignments: " + assignments + " | ms: " + milliseconds + " | OK");
    }
    public static void Main() {
        int[] sizes = {1000, 10000, 30000};
        string[] names = {"Selection", "Insertion", "Bubble", "Counting"};
        Random random = new Random();
        for (int i = 0; i < sizes.Length; i++) {
            int[] original = new int[sizes[i]];
            for (int j = 0; j < original.Length; j++) original[j] = random.Next(original.Length + 1);
            Console.WriteLine("ARRAY SIZE: " + sizes[i]);
            for (int k = 0; k < names.Length; k++) benchmark(original, k, names[k]);
        }
    }
}
