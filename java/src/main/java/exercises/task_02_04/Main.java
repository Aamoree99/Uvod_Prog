/*
Cvičení 02 — Úloha 2.4
Název: Faktoriál
Body: 1 bod

Zadání:
Uživatel zadá číslo. Vypište faktoriál tohoto čísla. Faktoriál čísla 0 je definován jako 1,
faktoriál pro číslo n>0 je definovaný jako součin všech čísel od 1 do n. Pro záporná čísla není
faktoriál definován. Př.: 6!  = 1*2*3*4*5*6 =720. [1 bod]

Ukázka:

Vypocteme faktorial kladneho cisla

Zadejte cislo: 6

Faktorial je 720

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

package exercises.task_02_04;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    private static int read_int() { return Integer.parseInt(INPUT.nextLine().trim()); }
    public static void main(String[] args) {
        System.out.printf(Locale.ROOT, "Zadejte cislo:%n");

        // trim убирает перевод строки, parse преобразует текст в int. Ввод должен быть целым числом.
        int a = read_int();

        // Факториал определён для неотрицательных целых. В Rust 34! помещается в u128, а 35! уже нет; сохраняем этот предел и здесь.
        if (a < 0) {
            System.out.printf(Locale.ROOT, "Faktorial zaporneho cisla neni definovan.%n");
        } else if (a > 34) {
            System.out.printf(Locale.ROOT, "Hodnota je prilis vysoka pro datovy typ u128.%n");
        } else {
            // Начинаем произведение с 1. Для 0 цикл не выполняется, поэтому 0! = 1.
            BigInteger factorial = BigInteger.ONE; // int вмещает 12!, но уже не вмещает 13!.
            int number = a;

            // Перемножаем a, a-1, ..., 1. Для точного большого произведения используем подходящий числовой тип.
            while (number >= 1) {
                factorial = factorial.multiply(BigInteger.valueOf(number));
                number -= 1;
            }

            System.out.printf(Locale.ROOT, "Faktorial je %s%n", factorial);
        }
    }
}
