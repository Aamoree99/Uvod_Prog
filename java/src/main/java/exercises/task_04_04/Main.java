/*
Cvičení 04 — Úloha 4.4
Název: Obrácení pole
Body: 3 body

Zadání:
Napište funkci, která v zadaném poli obrátí pořadí prvků. První prvek bude posledním, druhý prvek
předposledním, atd. Je zakázáno použít pomocné pole. [3 body]

Ukázka:

int[] pole = { 5, 7, 2, 1, 0, 6 };

ObratPole(pole); // 6, 0, 1, 2, 7, 5

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

package exercises.task_04_04;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Обмен крайних элементов разворачивает массив без вспомогательного массива.
    static void obrat_pole(int[] pole) {
        int levy=0, pravy=pole.length-1;
        while (levy < pravy) {
            int pomocna=pole[levy]; pole[levy]=pole[pravy]; pole[pravy]=pomocna;
            levy += 1; pravy -= 1;
        }
    }
    public static void main(String[] args) {
        int[] pole = new int[] {5,7,2,1,0,6};
        System.out.printf(Locale.ROOT, "Pred: %s%n", Arrays.toString(pole)); obrat_pole(pole); System.out.printf(Locale.ROOT, "Po:   %s%n", Arrays.toString(pole));
    }
}
