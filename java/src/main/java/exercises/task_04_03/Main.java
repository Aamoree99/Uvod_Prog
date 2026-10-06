/*
Cvičení 04 — Úloha 4.3
Název: Zaokrouhlení pole
Body: 2 body

Zadání:
Napište funkci, která pro zadané pole reálných čísel vrátí pole celých čísel. Hodnoty prvků v novém
poli vzniknou zaokrouhlením hodnot prvků z původního pole. [2 body]

Ukázka:

double[] pole = { 5.12, -3.5996, 156, 7.9 };

zaokrouhli = ZaokrouhliPole(pole); // 5, -4, 156, 8

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

package exercises.task_04_03;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Возвращаем новый массив из четырёх элементов, как в исходном примере.
    static int[] zaokrouhli_pole(double[] pole) {
        int[] vysledek = new int[] {0,0,0,0};
        int i=0;
        while (i < pole.length) {
            // В Rust половины округляются от нуля; выбираем такое же правило в каждом языке.
            vysledek[i] = (int)Math.copySign(Math.floor(Math.abs(pole[i]) + 0.5), pole[i]);
            i += 1;
        }
        return vysledek;
    }
    public static void main(String[] args) {
        double[] pole = new double[] {5.12,-3.5996,156.0,7.9};
        int[] zaokrouhlene=zaokrouhli_pole(pole);
        System.out.printf(Locale.ROOT, "Puvodni:      %s%n", Arrays.toString(pole)); System.out.printf(Locale.ROOT, "Zaokrouhlene: %s%n", Arrays.toString(zaokrouhlene));
    }
}
