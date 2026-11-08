/*
Cvičení 05 — Úloha 5.6
Název: Vylepšený Bubble Sort
Body: 2 body; 1 bod (v pořadí podúloh)

Zadání:
Algoritmus Bubble Sort třídící sestupně zadané pole lze implementovat ve dvou verzích. V základní
verzi algoritmu se ve vnořeném cyklu prochází pole až do posledního prvku.  Ve vylepšené verzi
algoritmu se ve vnořeném cyklu prochází pole jen do místa poslední výměny, která proběhla
v předchozí iteraci vnějšího cyklu.

a) Napište vylepšenou verzi Bubble sortu. Pokud jste vylepšenou verzi napsali již v příklad 5.3c,
napište nyní verzi základní. [2 body]

b) Změřte (dle příkladu 5.4) počty operací a dobu běhu vylepšené a základní verze. [1 bod]

Ukázka:

Prvku 1000

Bubble sort – porovnani: 944055, prirazeni: 763212

Bubble sort2 – porovnani: 496415, prirazeni: 763212

Bubble sort – doba behu 46 ms

Bubble sort2 – doba behu 30 ms

Prvku 10000

...

Prvku 30000

...

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

package exercises.task_05_06;

public class Main {
    // Считаем только операции над значениями, а не управление циклами.
    static long assignments = 0, comparisons = 0;
    static int assign(int value) { assignments++; return value; }
    static boolean less(int a, int b) { comparisons++; return a < b; }
    static boolean greater(int a, int b) { comparisons++; return a > b; }
    static void swap(int[] a, int j) {
        int temporary = assign(a[j]);
        a[j] = assign(a[j + 1]);
        a[j + 1] = assign(temporary);
    }

    static void basicBubbleSort(int[] a) {
        for (int i = 0; i + 1 < a.length; i++)
            for (int j = 0; j + 1 < a.length; j++)
                if (less(a[j], a[j + 1])) swap(a, j);
    }
    static void improvedBubbleSort(int[] a) {
        int limit = a.length;
        while (limit > 1) {
            int lastSwap = 0;
            for (int j = 0; j + 1 < limit; j++)
                if (less(a[j], a[j + 1])) {
                    swap(a, j);
                    lastSwap = j + 1;
                }
            limit = lastSwap;
        }
    }
    static void sort(int[] a, int algorithm) {
        if (algorithm == 0) basicBubbleSort(a);
        else improvedBubbleSort(a);
    }

    static boolean isDescending(int[] a) {
        for (int i = 1; i < a.length; i++)
            if (a[i] > a[i - 1]) return false;
        return true;
    }
    static void benchmark(int[] original, int algorithm, String name) {
        int[] a = original.clone();
        assignments = comparisons = 0;
        long start = System.nanoTime();
        sort(a, algorithm);
        double milliseconds = (System.nanoTime() - start) / 1000000.0;
        if (!isDescending(a)) throw new IllegalStateException("Sorting failed");
        System.out.println(name + " | Comparisons: " + comparisons + " | Assignments: " + assignments + " | ms: " + milliseconds + " | OK");
    }
    public static void main(String[] args) {
        int[] sizes = {1000, 10000, 30000};
        String[] names = {"Basic Bubble", "Improved Bubble"};
        java.util.Random random = new java.util.Random();
        for (int i = 0; i < sizes.length; i++) {
            int[] original = new int[sizes[i]];
            for (int j = 0; j < original.length; j++) original[j] = random.nextInt(original.length + 1);
            System.out.println("ARRAY SIZE: " + sizes[i]);
            for (int k = 0; k < names.length; k++) benchmark(original, k, names[k]);
        }
    }
}
