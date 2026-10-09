/*
Cvičení 01 — Úloha 1.2
Název: Zbytek po celočíselném dělení bez operátoru %
Body: 3 body

Zadání:
Ve funkci Main deklarujte dvě celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami (ideálně různými). Napište program, který spočte zbytek po celočíselném
dělení první zadané proměnné druhou zadanou proměnnou. Program nesmí použít operátor zbytku po
celočíselném dělení %. Cíl výpočtu a jeho výsledek vypište vhodným způsobem na obrazovku. [3 body]

Ukázka:

Hodnota první proměnné je 14, hodnota druhé proměnné je 3.

Zbytek po celočíselném dělení čísla 14 číslem 3 je 2.

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

package exercises.task_01_02;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        int a = 14;
        int b = 3;
        // Целочисленное a / b отбрасывает дробную часть. Умножив частное на b,
        // получаем делимую часть числа; остаток — разность с a. Делитель b не должен быть нулём.
        var result = a - (a / b) * b;

        System.out.printf(Locale.ROOT, "The remainder of dividing %s by %s is %s.%n", a, b, result);
    }
}
