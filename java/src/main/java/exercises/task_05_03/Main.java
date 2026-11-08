/*
Cvičení 05 — Úloha 5.3
Název: Sestupné třídění
Body: 3+1 body; 3+1 body; 2+1 body; 3+2 body (v pořadí podúloh)

Zadání:
Napište třídící funkce pro sestupné třídění. Body navíc lze získat za použití pomocných funkcí
z příkladu 5.2 pro porovnávání a přiřazování hodnot prvků. Pomocné funkce nepoužívejte pro práci
s řídící proměnnou cyklu. Výjimkou je Count sort, u kterého pomocné funkce se používají pro řídící
proměnnou cyklu v situacích, ve kterých je řídící proměnná cyklu brána jako prvek pole.

a) Selection sort [3+1 body]

b) Insertion sort [3+1  body]

c) Bubble sort [2+1  body]

d) Count sort [3+2 body]


Ukázka:

int[] pole = { 5, 7, 1, 5, 6, 8, 9, 0, 1, 2};

SelectionSort(pole); // 9, 8, 7, 6, 5, 5, 2, 1, 1, 0

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 30,  55

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

package exercises.task_05_03;

public class Main {
    // Считаем только операции над значениями, а не управление циклами.
    static long assignments = 0, comparisons = 0;
    static int assign(int value) { assignments++; return value; }
    static boolean less(int a, int b) { comparisons++; return a < b; }
    static boolean greater(int a, int b) { comparisons++; return a > b; }

    static void selectionSort(int[] a) {
        for (int i = 0; i < a.length; i++) {
            int maximum = i;
            for (int j = i + 1; j < a.length; j++)
                if (greater(a[j], a[maximum])) maximum = j;
            int temporary = assign(a[i]);
            a[i] = assign(a[maximum]);
            a[maximum] = assign(temporary);
        }
    }
    static void insertionSort(int[] a) {
        for (int i = 1; i < a.length; i++) {
            int key = assign(a[i]), j = i;
            while (j > 0 && less(a[j - 1], key)) {
                a[j] = assign(a[j - 1]);
                j--;
            }
            a[j] = assign(key);
        }
    }
    static void bubbleSort(int[] a) {
        for (int i = 0; i < a.length; i++)
            for (int j = 0; j + 1 < a.length - i; j++)
                if (less(a[j], a[j + 1])) swap(a, j);
    }
    static void swap(int[] a, int j) {
        int temporary = assign(a[j]);
        a[j] = assign(a[j + 1]);
        a[j + 1] = assign(temporary);
    }
    // Счётчики и диапазон соответствуют исходной версии Rust.
    static void countingSort(int[] a) {
        if (a.length == 0) return;
        int maximum = 0;
        for (int i = 0; i < a.length; i++) {
            if (a[i] < 0 || a[i] > 1000) throw new IllegalArgumentException("Counting sort value outside range");
            if (greater(a[i], maximum)) maximum = assign(a[i]);
        }
        int[] counts = new int[1000 + 1];
        for (int i = 0; i < a.length; i++) counts[a[i]] = assign(counts[a[i]] + 1);
        int position = 0;
        for (int number = maximum; number >= 0; number--)
            while (greater(counts[number], 0)) {
                a[position++] = assign(number);
                counts[number] = assign(counts[number] - 1);
            }
    }
    static void printArray(int[] a) {
        for (int i = 0; i < a.length; i++) System.out.print(a[i] + " ");
        System.out.println();
    }
    static void sort(int[] a, int algorithm) {
        if (algorithm == 0) selectionSort(a);
        else if (algorithm == 1) insertionSort(a);
        else if (algorithm == 2) bubbleSort(a);
        else countingSort(a);
    }

    public static void main(String[] args) {
        int[] original = {5, 7, 1, 5, 6, 8, 9, 0, 1, 2};
        String[] names = {"Selection", "Insertion", "Bubble", "Counting"};
        for (int k = 0; k < 4; k++) {
            int[] a = original.clone();
            assignments = comparisons = 0;
            sort(a, k);
            System.out.println(names[k]);
            printArray(a);
            System.out.println("Assignments: " + assignments + ", Comparisons: " + comparisons);
        }
    }
}
