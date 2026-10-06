/*
Cvičení 03 — Úloha 3.1
Název: DPH
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadanou cenu zboží a číselně zadanou sazbu DPH spočte cenu zboží
s DPH. [1 bod]

b) Napište funkci, která pro sazbu DPH použije výčtový typ. Existují tři sazby DPH: základní 21%,
první snížená 15% a druhá snížená 10% [1 bod]:

Ukázka:

x = SpoctiDPH1(56.20, 21); // 68.002

y = SpoctiDPH2(115, DPH.PrvniSnizena); // 132.25

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

package exercises.task_03_01;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Перечисление ограничивает выбор учебными ставками из условия.
    enum Dph { Zakladni, PrvniSnizena, DruhaSnizena }
    // Прибавляем к цене налог, вычисленный как цена * процент / 100.
    static double dph1(double price, double dph) { return price + price * dph / 100.0; }
    static double dph2(double price, Dph dph) {
        double percent;
        if (dph == Dph.Zakladni) percent = 21.0;
        else if (dph == Dph.PrvniSnizena) percent = 15.0;
        else percent = 10.0;
        return price + price * percent / 100.0;
    }
    public static void main(String[] args) {
        double x = dph1(56.20, 21.0);
        double y1 = dph2(115.0, Dph.Zakladni);
        double y2 = dph2(115.0, Dph.PrvniSnizena);
        double y3 = dph2(115.0, Dph.DruhaSnizena);
        System.out.printf(Locale.ROOT, "Cena 56.20 Kč se sazbou DPH 21 %% je %.2f Kč.%n", x);
        System.out.printf(Locale.ROOT, "Cena 115.00 Kč se základní sazbou DPH 21 %% je %.2f Kč.%n", y1);
        System.out.printf(Locale.ROOT, "Cena 115.00 Kč s první sníženou sazbou DPH 15 %% je %.2f Kč.%n", y2);
        System.out.printf(Locale.ROOT, "Cena 115.00 Kč s druhou sníženou sazbou DPH 10 %% je %.2f Kč.%n", y3);
    }
}
