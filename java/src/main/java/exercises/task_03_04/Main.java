/*
Cvičení 03 — Úloha 3.4
Název: Komplexní čísla
Body: 1/2 bodu; 1/2 bodu; 1 bod (v pořadí podúloh)

Zadání:
Vytvořte strukturu pro reprezentaci komplexních čísel v algebraickém tvaru. Napište funkce pro
následující operace

a) sčítání komplexních čísel [1/2 bodu]

b) odečítání komplexních čísel [1/2 bodu]

c) násobení komplexních čísel [1 bod]

Ukázka:

x.re = 5.2; x.im = 3.25;

y.re = 1.6; y.im = -4;

a = secti(x, y); // 6.8 - 0.75i

b = odecti(x, y); // 3.6 + 7.25i

c = vynasob(x, y); // 21.32 - 15.6i

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

package exercises.task_03_04;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // re — действительная часть, im — коэффициент при мнимой единице i.
    static record Complex(double re, double im) {}
    // Складываем и вычитаем действительные и мнимые части отдельно.
    static Complex add(Complex x, Complex y) { return new Complex(x.re + y.re, x.im + y.im); }
    // Возвращаем разность комплексных чисел x и y.
    static Complex subtract(Complex x, Complex y) { return new Complex(x.re - y.re, x.im - y.im); }
    // Раскрываем скобки; i² = -1, поэтому действительная часть равна ac - bd.
    static Complex multiply(Complex x, Complex y) { return new Complex(x.re*y.re - x.im*y.im, x.re*y.im + x.im*y.re); }
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        Complex x = new Complex(5.2, 3.25), y = new Complex(1.6, -4.0);
        Complex a = add(x,y), b = subtract(x,y), c = multiply(x,y);
        // Четыре знака после запятой; для мнимой части явно показываем знак.
        System.out.printf(Locale.ROOT, "Sum: %.4f %+.4fi%n", a.re, a.im);
        System.out.printf(Locale.ROOT, "Difference: %.4f %+.4fi%n", b.re, b.im);
        System.out.printf(Locale.ROOT, "Product: %.4f %+.4fi%n", c.re, c.im);
    }
}
