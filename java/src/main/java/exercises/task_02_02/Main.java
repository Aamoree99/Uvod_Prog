/*
Cvičení 02 — Úloha 2.2
Název: Tři čísla vzestupně
Body: 3 body

Zadání:
Uživatel zadá tři čísla z klávesnice. Setřiďte tato číslo ve vnitřní paměti. Vypište tato čísla
setříděná vzestupně dle jejich hodnot. [3 body]

Ukázka:

Setridime tri cisla vzestupne

Zadejte prvni cislo: 33

Zadejte druhe cislo: 6

Zadejte treti cislo: 14

Setridena cisla jsou 6, 14, 33

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

package exercises.task_02_02;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    // Читаем одно целое число из строки ввода.
    private static int readInt() { return Integer.parseInt(INPUT.nextLine().trim()); }
    // Вводим три целых числа, каждое на своей строке; Предполагается корректный ввод.

    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        System.out.printf(Locale.ROOT, "Sort three numbers in ascending order%n");

        System.out.printf(Locale.ROOT, "Enter the first number:%n");

        int a = readInt();

        System.out.printf(Locale.ROOT, "Enter the second number:%n");
        // Каждое число читается отдельно.

        int b = readInt();

        System.out.printf(Locale.ROOT, "Enter the third number:%n");

        int c = readInt();

        // Три сравнения с обменом упорядочивают числа: сначала a и b, затем b и c,
        // потом снова a и b, потому что второй обмен мог нарушить их порядок.
        // В savedNumber сохраняем заменяемое значение, чтобы не потерять его.
        if (a > b) {
            var savedNumber = a;
            a = b;
            b = savedNumber;
        }

        if (b > c) {
            var savedNumber = b;
            b = c;
            c = savedNumber;
        }

        if (a > b) {
            var savedNumber = a;
            a = b;
            b = savedNumber;
        }

        System.out.printf(Locale.ROOT, "The sorted numbers are %s, %s, %s%n", a, b, c);
    }
}
