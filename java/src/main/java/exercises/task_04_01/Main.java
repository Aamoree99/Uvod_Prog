/*
Cvičení 04 — Úloha 4.1
Název: Sestupné setřídění
Body: 1 bod

Zadání:
Napište funkci, která své dva parametry setřídí sestupně. [1 bod]

Ukázka:

int a = 5, b = 7;

SetridSestupne(ref a, ref b); // a = 7; b = 5;

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

package exercises.task_04_01;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Java передаёт примитивы по значению: используем массив как изменяемую пару.
    static void sortDescending(int[] pair) {
        if (pair[0] < pair[1]) { int savedNumber=pair[0]; pair[0]=pair[1]; pair[1]=savedNumber; }
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        int a=5,b=7;
        System.out.printf("Before: a = %d, b = %d%n",a,b);
        int[] pair={a,b}; sortDescending(pair); a=pair[0]; b=pair[1];
        System.out.printf("After:   a = %d, b = %d%n",a,b);
    }
}
