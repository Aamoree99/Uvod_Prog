/*
Cvičení 01 — Úloha 1.3
Název: Počet čísel menších než 10
Body: 1 bod

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, kolik ze zadaných hodnot je menších než
hodnota 10. Cíl příkladu a jeho výsledek vypište vhodným způsobem na obrazovku. [1 bod]

Ukázka:

Hodnota první proměnné je 8, hodnota druhé proměnné je 22, hodnota třetí proměnné je 10.

Byla zadána čísla 8, 22 a 10. Počet čísel s hodnotou menší než  hodnota 10 je 1.

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

package exercises.task_01_03;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        int a = 8;
        int b = 22;
        int c = 10;

        // Тернарный оператор превращает результат сравнения в 1 при истине и 0 иначе.
        // Сумма этих единиц — количество подходящих чисел. Само число 10 не подходит.
        var count = (a < 10 ? 1 : 0) + (b < 10 ? 1 : 0) + (c < 10 ? 1 : 0);

        System.out.printf(Locale.ROOT, "The entered numbers are %s, %s and %s. The count of numbers less than 10 is %s.%n", a, b, c, count);
    }
}
