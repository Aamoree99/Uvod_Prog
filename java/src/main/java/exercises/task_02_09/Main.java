/*
Cvičení 02 — Úloha 2.9
Název: Statistické informace
Body: 1/2 bodu; 1/2 bodu; 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Uživatel zadává posloupnost kladných čísel. Pokud uživatel zadá nekladné číslo, je posloupnost
ukončena (a toto nekladné číslo není její součástí). Vypište:

a) počet čísel v posloupnosti. Pokud posloupnost obsahuje nejvýše jedno číslo, nevypisujte další
části úkolu. [1/2 bodu]

b) součet hodnot čísel [1/2 bodu]

c) hodnotu největšího čísla [1 bod]

d) hodnotu nejmenšího čísla [1 bod]

e) hodnotu druhého největšího číslo [1 bod]

f) hodnotu druhého nejmenšího číslo [1 bod]

Ukázka:

Spocteme statisticke informace o zadanych kladnych cislech.

Zadejte cislo: 5

Zadejte cislo: 6

Zadejte cislo: 11

Zadejte cislo: 22

Zadejte cislo: 15

Zadejte cislo: 0

Delka posloupnosti je 5.

Soucet prvku posloupnosti je 59.

Nejmensi prvek posloupnosti je 5, druhy nejmensi je 6.

Nejvetsi prvek posloupnosti je 22, druhy nejvetsi je 15.

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

package exercises.task_02_09;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    private static int read_int() { return Integer.parseInt(INPUT.nextLine().trim()); }
    public static void main(String[] args) {
        int count = 0;
        int sum = 0;

        // Начальные границы позволяют первому числу заменить максимум и минимум.
        // В этой задаче вводятся только положительные числа, поэтому границы не совпадут с вводом.
        int max = Integer.MIN_VALUE;
        int sec_max = Integer.MIN_VALUE;

        int min = Integer.MAX_VALUE;
        int sec_min = Integer.MAX_VALUE;

        while ((true)) {
            System.out.printf(Locale.ROOT, "Zadejte cislo:%n");

            // Читаем одно целое число на строке. trim убирает пробелы, parse разбирает число;
            // Предполагается корректный ввод.
            int a = read_int();

            // Неположительное число завершает ввод и не входит ни в сумму, ни в количество.
            if (a <= 0) {
                break;
            }

            count += 1;
            sum += a;

            // Храним два наибольших элемента. Новый максимум сначала сдвигает старый на второе
            // место. Иначе кандидат может улучшить только второй максимум. Порядок присваиваний важен.
            // Повторы учитываются: для 9, 9, 5 оба максимума равны 9.
            if (a > max) {
                sec_max = max;
                max = a;
            } else if (a > sec_max) {
                sec_max = a;
            }

            // С минимумами делаем то же самое, но сравнения направлены в другую сторону.
            if (a < min) {
                sec_min = min;
                min = a;
            } else if (a < sec_min) {
                sec_min = a;
            }
        }

        System.out.printf(Locale.ROOT, "Delka posloupnosti je %s.%n", count);

        // При нуле или одном элементе по условию выводим только длину последовательности.
        if (count > 1) {
            System.out.printf(Locale.ROOT, "Soucet prvku posloupnosti je %s.%n", sum);
            System.out.printf(Locale.ROOT, "Nejmensi prvek posloupnosti je %s, druhy nejmensi je %s.%n", min, sec_min);
            System.out.printf(Locale.ROOT, "Nejvetsi prvek posloupnosti je %s, druhy nejvetsi je %s.%n", max, sec_max);
        }
    }
}
