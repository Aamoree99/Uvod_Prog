/*
Cvičení 02 — Úloha 2.1
Název: Dvě čísla sestupně
Body: 1 bod

Zadání:
Uživatel zadá dvě čísla z klávesnice. Setřiďte tato číslo ve vnitřní paměti. Vypište tato čísla
setříděná sestupně dle jejich hodnot. [1 bod]

Ukázka:

Setridime dve cisla sestupne

Zadejte prvni cislo: 5

Zadejte druhe cislo: 9

Setridena cisla jsou 9, 5

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

package exercises.task_02_01;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    // Читаем одно целое число из строки ввода.
    private static int readInt() { return Integer.parseInt(INPUT.nextLine().trim()); }
    // Стандартный ввод нужен для чтения с клавиатуры. В этом примере вводим целые числа по одному на строке.

    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        System.out.printf(Locale.ROOT, "Sort two numbers in descending order%n");

        System.out.printf(Locale.ROOT, "Enter the first number:%n");
        // Читаем строку со стандартного ввода и убираем окружающие пробелы;
        // преобразуем её в целое число. Некорректный ввод вызывает ошибку.

        int a = readInt();

        System.out.printf(Locale.ROOT, "Enter the second number:%n");
        // Следующее число читается отдельно, без накопления предыдущего ввода.

        int b = readInt();

        // Для убывающего порядка первое число должно быть не меньше второго.
        // Временная переменная сохраняет a, чтобы оно не потерялось при обмене.
        if (a < b) {
            var savedNumber = a;
            a = b;
            b = savedNumber;
        }

        System.out.printf(Locale.ROOT, "The sorted numbers are %s, %s%n", a, b);
    }
}
