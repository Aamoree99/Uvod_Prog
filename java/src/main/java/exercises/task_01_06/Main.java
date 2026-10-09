/*
Cvičení 01 — Úloha 1.6
Název: Různá čísla
Body: 2 body

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, zda zadané hodnoty čísel jsou navzájem (po
dvou) různé. Cíl příkladu a jeho výsledek vypište vhodným způsobem na obrazovku. [2 body]

Ukázka:

Hodnota první proměnné je 7, hodnota druhé proměnné je 3 hodnota třetí proměnné je 7.

Hodnoty 7, 3 a 7 nejsou navzájem různé.

Hodnota první proměnné je 25, hodnota druhé proměnné je 8 hodnota třetí proměnné je 19.

Hodnoty 25, 8 a 19 jsou navzájem různé.

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

package exercises.task_01_06;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        int a = 25;
        int b = 8;
        int c = 19;

        // Все три пары должны различаться одновременно. != означает «не равно».
        // Одной проверки a != b недостаточно: например, a и c могут совпадать.
        if (a != b && a != c && b != c) {
            System.out.printf(Locale.ROOT, "The numbers %s, %s and %s are pairwise distinct.%n", a, b, c);
        } else {
            System.out.printf(Locale.ROOT, "The numbers %s, %s and %s are not pairwise distinct.%n", a, b, c);
        }
    }
}
