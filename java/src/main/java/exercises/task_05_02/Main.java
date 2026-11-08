/*
Cvičení 05 — Úloha 5.2
Název: Příprava na třídění
Body: 1 bod; 1/2 bodu; 1/2 bodu (v pořadí podúloh)

Zadání:
Vytvořte dvě globální proměnné (prirazeni a porovnani), které budou sloužit pro měření počtu volání
funkcí vytvořených v tomto příkladu. Napište následující funkce, které budou nahrazovat operátory:

a) funkce přiřazení (globální proměnná prirazeni ) [1 bod]

b) funkce je menší (globální proměnná porovnani ) [1/2 bodu]

c) funkce je větší (globální proměnná porovnani ) [1/2 bodu]


Ukázka:

int x = 5, y = 4, z = 3;

bool je1 = JeMensi(y, z); // false

bool je2 = JeMensi(x, z); // false

bool je3 = JeVetsi(x, y); // true

Prirazeni(ref x, y); // x = 4

Prirazeni(ref y, z); // y = 3

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 2 3

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

package exercises.task_05_02;

public class Main {
    // Считаем только операции над значениями, а не управление циклами.
    static long assignments = 0, comparisons = 0;
    static int assign(int value) { assignments++; return value; }
    static boolean less(int a, int b) { comparisons++; return a < b; }
    static boolean greater(int a, int b) { comparisons++; return a > b; }

    public static void main(String[] args) {
        int x = 5, y = 4, z = 3;
        System.out.println("y < z: " + less(y, z));
        System.out.println("x < z: " + less(x, z));
        System.out.println("x > y: " + greater(x, y));
        x = assign(y);
        y = assign(z);
        System.out.println("x = " + x + ", y = " + y + ", z = " + z);
        System.out.println("Assignments: " + assignments + ", Comparisons: " + comparisons);
    }
}
