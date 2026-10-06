/*
Cvičení 02 — Úloha 2.5
Název: Nejmenší společný násobek
Body: 3 body

Zadání:
Uživatel zadá dvě čísla z klávesnice. Vypište jejich nejmenší společný násobek. Nejmenší společný
násobek čísel x a y je nejmenší číslo n takové, že x dělí n beze zbytku a zároveň y dělí n beze
zbytku.  [3 body]

Ukázka:

Spocteme nejmensi spolecny nasobek cisel

Zadej prvni cislo: 21

Zadej druhe cislo: 30

Nejmensi spolecny nasobek je 210

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

package exercises.task_02_05;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    private static int read_int() { return Integer.parseInt(INPUT.nextLine().trim()); }
    public static void main(String[] args) {
        System.out.printf(Locale.ROOT, "Zadej prvni cislo:%n");

        int a = read_int();

        System.out.printf(Locale.ROOT, "Zadej druhe cislo:%n");

        int b = read_int();

        // Этот алгоритм рассчитан на положительные a и b. b = 0 вызывает ошибку деления,
        // a = 0 сразу даёт 0; отрицательные значения не подходят для этого способа поиска.
        var nasobek = a;

        // Перебираем a, 2a, 3a, ... — они уже делятся на a. Первый такой кандидат,
        // который делится и на b без остатка, является наименьшим общим кратным.
        while (nasobek % b != 0) {
            nasobek += a;
        }

        System.out.printf(Locale.ROOT, "Nejmensi spolecny nasobek je %s%n", nasobek);
    }
}
