/*
Cvičení 04 — Úloha 4.5
Název: Maximum z pole
Body: 1 bod; 1 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která v zadaném poli nezáporných čísel najde a pomocí parametrů funkce vrátí:

a) hodnotu největšího prvku [1 bod]

b) hodnotu druhého největšího prvku [1 bod]

c) třetího největšího prvku [2 body]

Ukázka:

int[] pole = { 2, 5, 7, 1, 4, 6, 9, 8, 2 };

MaxPole(pole, ref max, ref max2, ref max3); // 9, 8, 7

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

package exercises.task_04_05;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // По условию числа неотрицательные, поэтому начальные максимумы равны нулю.
    // Повторы учитываются; для короткого массива незаполненные места остаются нулевыми.
    static void arrayMaxima(int[] array, int[] maxima) {
        maxima[0]=0; maxima[1]=0; maxima[2]=0;
        int i=0;
        while (i < array.length) {
            int number=array[i];
            // Сдвигаем значения справа налево, чтобы не потерять прежние максимумы.
            if (number > maxima[0]) { maxima[2]=maxima[1]; maxima[1]=maxima[0]; maxima[0]=number; }
            else if (number > maxima[1]) { maxima[2]=maxima[1]; maxima[1]=number; }
            else if (number > maxima[2]) { maxima[2]=number; }
            i += 1;
        }
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        int[] array = new int[] {2,5,7,1,4,6,9,8,2};
        int[] maxima=new int[3];
        arrayMaxima(array,maxima);
        System.out.printf(Locale.ROOT, "Array: %s%n", Arrays.toString(array));
        System.out.printf(Locale.ROOT, "Largest:       %s%n", maxima[0]);
        System.out.printf(Locale.ROOT, "Second largest: %s%n", maxima[1]);
        System.out.printf(Locale.ROOT, "Third largest: %s%n", maxima[2]);
    }
}
